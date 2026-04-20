use crate::{
    decoder::{decode, BranchKind, Instruction, LoadKind, MulDivOp, OpImmKind, OpKind, StoreKind, SystemKind, INSTRUCTION_SIZE},
    ZkvmError, REGISTER_COUNT,
};
use crate::proof::{rv32m_mul_artifact, ProofTrace};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome { Continued, Halted, Ecall, Ebreak }

pub struct Vm {
    pub pc: u32,
    pub regs: [u32; REGISTER_COUNT],
    pub memory: Vec<u8>,
    pub memory_base: u32,
    pub halted: bool,
    pub proof_trace: ProofTrace,
}

impl Vm {
    pub fn new(memory_base: u32, memory: Vec<u8>, entry: u32) -> Self {
        Self { pc: entry, regs: [0; REGISTER_COUNT], memory, memory_base, halted: false, proof_trace: ProofTrace::new() }
    }

    pub fn step(&mut self) -> Result<StepOutcome, ZkvmError> {
        if self.halted { return Err(ZkvmError::Halted); }
        let raw = self.load_u32(self.pc)?;
        let instr = decode(raw)?;
        let next_pc = self.pc.wrapping_add(INSTRUCTION_SIZE);
        let outcome = match instr {
            Instruction::Lui { rd, imm } => { self.set_reg(rd, imm); StepOutcome::Continued }
            Instruction::Auipc { rd, imm } => { self.set_reg(rd, self.pc.wrapping_add(imm)); StepOutcome::Continued }
            Instruction::Jal { rd, imm } => { self.set_reg(rd, next_pc); self.pc = self.pc.wrapping_add(imm as u32); return Ok(StepOutcome::Continued); }
            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.regs[rs1 as usize].wrapping_add(imm as u32) & !1;
                self.set_reg(rd, next_pc); self.pc = target; return Ok(StepOutcome::Continued);
            }
            Instruction::Branch { kind, rs1, rs2, imm } => {
                let lhs = self.regs[rs1 as usize];
                let rhs = self.regs[rs2 as usize];
                let taken = match kind {
                    BranchKind::Beq => lhs =9 rhs,
                    BranchKind::Bne => lhs != rhs,
                    BranchKind::Blr => (lhs as i32) < (rhs as i32),
                    BranchKind::Bge => (lhs as i32) >= (rhs as i32),
                    BranchKind::Bltu => lhs < rhs,
                    BranchKind::Bgeu => lhs >= rhs,
                };
                if taken { self.pc = self.pc.wrapping_add(imm as u32); } else { self.pc = next_pc; }
                StepOutcome::Continued
            }
            Instruction::Load { kind, rd, rs1, imm } => {
                let addr = self.regs[rs1 as usize].wrapping_add(imm as u32);
                let val = match kind {
                    LoadKind::Lb => self.load_u8(addr)? as i8 as i32 as u32,
                    LoadKind::Lbu => self.load_u8(addr)? as u32,
                    LoadKind::Lh => self.load_u16(addr)? as i16 as i32 as u32,
                    LoadKind::Lhu => self.load_u16(addr)? as u32,
                    LoadKind::Lw => self.load_u32(addr)?,
                };
                self.set_reg(rd, val); StepOutcome::Continued
            }
            Instruction::Store { kind, rs1, rs2, imm } => {
                let addr = self.regs[rs1 as usize].wrapping_add(imm as u32);
                let val = self.regs[rs2 as usize];
                match kind {
                    StoreKind::Sb => self.store_u8(addr, val as u8)?,
                    StoreKind::Sh => self.store_u16(addr, val as u16)?,
                    StoreKind::Sw => self.store_u32(addr, val)?,
                }
                StepOutcome::Continued
            }
            Instruction::OpImm { kind, rd, rs1, imm } => {
                let lhs = self.regs[rs1 as usize];
                let rhs = imm as u32;
                let val = match kind {
                    OpImmKind::Addi => lhs.wrapping_add(rhs),
                    OpImmKind::Slti => ((lhs as i32) < imm) as u32,
                    OpImmKind::Sltiu => (lhs < rhs) as u32,
                    OpImmKind::Xori => lhs ^ rhs,
                    OpImmKind::Ori => lhs | rhs,
                    OpImmKind::Andi => lhs & rhs,
                    OpImmKind::Slli => lhs << (rhs & 0x1f),
                    OpImmKind::Srli => lhs >> (rhs & 0x1f),
                    OpImmKind::Srai => ((lhs as i32) >> (rhs & 0x1f)) as u32,
                };
                self.set_reg(rd, val); StepOutcome::Continued
            }
            Instruction::Op { kind, rd, rs1, rs2 } => {
                let lhs = self.regs[rs1 as usize];
                let rhs = self.regs[rs2 as usize];
                let val = match kind {
                    OpKind::Add => lhs.wrapping_add(rhs),
                    OpKind::Sub => lhs.wrapping_sub(rhs),
                    OpKind::Sll => lhs << (rhs & 0x1f),
                    OpKind::Slt => ((lhs as i32) < (rhs as i32)) as u32,
                    OpKind::Sltu => (lhs < rhs) as u32,
                    OpKind::Xor => lhs ^ rhs,
                    OpKind::Srl => lhs >> (rhs & 0x1f),
                    OpKind::Sra => ((lhs as i32) >> (rhs & 0x1f)) as u32,
                    OpKind::Or => lhs | rhs,
                    OpKind::And => lhs & rhs,
                };
                self.set_reg(rd, val); StepOutcome::Continued
            }
            Instruction::MulDiv { kind, rd, rs1, rs2 } => {
                let lhs = self.regs[rs1 as usize];
                let rhs = self.regs[rs2 as usize];
                let val = match kind {
                    MulDivOp::Mul | MulDivOp::Mulh | MulDivOp::Mulhsu | MulDivOp::Mulhu => {
                        let art = rv32m_mul_artifact(kind, lhs, rhs)?;
                        let res = art.result;
                        self.proof_trace.push(art)?;
                        res
                    }
                    MulDivOp::Div => if rhs == 0 { u32::MAX } else { (lhs as i32).wrapping_div(rhs as i32) as u32 },
                    MulDivOp::Div5 => if rhs == 0 { u32::MAX } else { lhs / rhs },
                    MulDivOp::Rem => if rhs == 0 { lhs } else { (lhs as i32).wrapping_rem(rhs as i32) as u32 },
                    MulDivOp::Remu => if rhs == 0 { lhs } else { lhs % rhs },
                };
                self.set_reg(rd, val); StepOutcome::Continued
            }
            Instruction::Fence | Instruction::FenceI => StepOutcome::Continued,
            Instruction::System(SystemKind::Ecall) => { self.halted = true; StepOutcome::Ecall }
            Instruction::System(SystemKind::Ebreak) => { self.halted = true; StepOutcome::Ebreak }
        };
        self.pc = next_pc;
        self.regs[0] = 0;
        Ok(outcome)
    }

    pub fn run(&mut self, max: usize) -> Result<StepOutcome, ZkvmError> {
        for _ in 0..max {
            let o = self.step()?;
            if o != StepOutcome::Continued { return Ok(o); }
        }
        Err(ZkvmError::StepLimitExceeded { limit: max })
    }

    fn set_reg(&mut self, rd: u8, val: u32) { if rd != 0 { self.regs[rd as usize] = val; } }
    fn load_u8(&self, addr: u32) -> Result<u8, ZkvmError> {
        let off = (addr.wrapping_sub(self.memory_base)) as usize;
        self.memory.get(off).copied().ok_or(ZkvmError::MemoryOutOfBounds { addr, len: 1 })
    }
    fn load_u16(&self, addr: u32) -> Result<u16, ZkvmError> {
        if addr % 2 != 0 { return Err(ZkvmError::MisalignedAccess { addr, alignment: 2 }); }
        let b0 = self.load_u8(addr)?; let b1 = self.load_u8(addr + 1)?;
        Ok(u16::from_le_bytes([b0, b1]))
    }
    fn load_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        if addr % 4 != 0 { return Err(ZkvmError::MisalignedAccess { addr, alignment: 4 }); }
        let b0 = self.load_u8(addr)?; let b1 = self.load_u8(addr + 1)?;
        let b2 = self.load_u8(addr + 2)?; let b3 = self.load_u8(addr + 3)?;
        Ok(u32::from_le_bytes([b0, b1, b2, b3]))
    }
    fn store_u8(&mut self, addr: u32, val: u8) -> Result<(), ZkvmError> {
        let off = (addr.wrapping_sub(self.memory_base)) as usize;
        let cell = self.memory.get_mut(off).ok_or(ZkvmError::MemoryOutOfBounds { addr, len: 1 })?;
        *cell = val; Ok(())
    }
    fn store_u16(&mut self, addr: u32, val: u16) -> Result<(), ZkvmError> {
        if addr % 2 != 0 { return Err(ZkvmError::MisalignedAccess { addr, alignment: 2 }); }
        let bytes = val.to_le_bytes();
        self.store_u8(addr, bytes[0])?; self.store_u8(addr + 1, bytes[1])?;
        Ok(())
    }
    fn store_u32(&mut self, addr: u32, val: u32) -> Result<(), ZkvmError> {
        if addr % 4 != 0 { return Err(ZkvmError::MisalignedAccess { addr, alignment: 4 }); }
        let bytes = val.to_le_bytes();
        self.store_u8(addr, bytes[0])?; self.store_u8(addr + 1, bytes[1])?;
        self.store_u8(addr + 2, bytes[2])?; self.store_u8(addr + 3, bytes[3])?;
        Ok(())
    }
}
