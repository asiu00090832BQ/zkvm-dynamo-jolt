use crate::decoder::{
    decode, BranchKind, Instruction, LoadKind, OpImmKind, OpKind, StoreKind,
};
use crate::ZkvmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    Continue,
    Bumped,
    Ecall,
    Ebreak,
    Halted,
}

#[derive(Debug, Clone)]
pub struct Vm {
    pub regs: [u32; 32],
    pub pc: u32,
    pub halted: bool,
    pub steps: u64,
    memory: Vec<u8>,
}

impl Um {
    pub fn new(memory: Vec<u8>) -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
            halted: false,
            steps: 0,
            memory,
        }
    }

    pub fn with_memory_size(size: usize) -> Self {
        Self::new(vec![0; size])
    }

    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut [u8] {
        &mut self.memory
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn reset(&mut self) {
        self.regs = [0; 32];
        self.pc = 0;
        self.halted = false;
        self.steps = 0;
    }

    pub fn load_program(&mut self, program: &[u8], address: u32) -> Result<(), ZkvmError> {
        let start = address as usize;
        let end = start
            .checked_add(program.len())
            .ok_or(ZkvmError::MemoryOutOfBounds {
                addr: address,
                size: program.len(),
            })?;
        if end > self.memory.len() {
            return Err(ZkvmError::MemoryOutOfBounds {
                addr: address,
                size: program.len(),
            });
        }
        self.memory[start..end].copy_from_slice(program);
        Ok(())
    }

    pub fn read_reg(&self, index: u8) -> Result<u32, ZkvmError> {
        self.regs
            .get(index as usize)
            .copied()
            .ok_or(ZkvmError::RegisterOutOfBounds {
                index: index as usize,
            })
    }

    pub fn write_reg(&mut self, index: u8, value: u32) -> Result<(), ZkvmError> {
        let index_usize = index as usize;
        if index_usize >= 32 {
            return Err(ZkvmError::RegisterOutOfBounds { index: index_usize });
        }
        if index_usize != 0 {
            self.regs[index_usize] = value;
        }
        Ok(())
    }

    pub fn fetch_word(&self, addr: u32) -> Result<u32, ZkvmError> {
        self.read_u32(addr)
    }

    pub fn decode_current(&self) -> Result<Instruction, ZkvmError> {
        decode(self.fetch_word(self.pc)?)
    }

    pub fn read_u8(&self, addr: u32) -> Result<u8, ZkvmError> {
        let start = self.check_range(addr, 1)?;
        Ok(self.memory[start])
    }

    pub fn read_u16(&self, addr: u32) -> Result<u16, ZkvmError> {
        self.check_alignment(addr, 2)?;
        let start = self.check_range(addr, 2)?;
        Ok(u16::from_le_bytes([self.memory[start], self.memory[start + 1]]))
    }

    pub fn read_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        self.check_alignment(addr, 4)?;
        let start = self.check_range(addr, 4)?;
        Ok(u32::from_le_bytes([
            self.memory[start],
            self.memory[start + 1],
            self.memory[start + 2],
            self.memory[start + 3],
        ]))
    }

    pub fn write_u8(&mut self, addr: u32, value: u8) -> Result<(), ZkvmError> {
        let start = self.check_range(addr, 1)?;
        self.memory[start] = value;
        Ok(())
    }

    pub fn write_u16(&mut self, addr: u32, value: u16) -> Result<(), ZkvmError> {
        self.check_alignment(addr, 2)?;
        let start = self.check_range(addr, 2)?;
        let bytes = value.to_le_bytes();
        self.memory[start..start + 2].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn write_u32(&mut self, addr: u32, value: u32) -> Result<(), ZkvmError> {
        self.check_alignment(addr, 4)?;
        let start = self.check_range(addr, 4)?;
        let bytes = value.to_le_bytes();
        self.memory[start..start + 4].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn step(&mut self) -> Result<StepOutcome, ZkvmError> {
        if self.halted {
            return Ok(StepOutcome::Halted);
        }

        let pc = self.pc;
        let raw = self.fetch_word(pc)?;
        let inst = decode(raw)?;
        let mut outcome = StepOutcome::Continue;
        let mut next_pc = pc.wrapping_add(4);

        match inst {
            Instruction::Lui { rd, imm } => {
                self.write_reg(rd, imm as u32)?;
            }
            Instruction::Auipc { rd, imm } => {
                let value = pc.wrapping_add(imm as u32);
                self.write_reg(rd, value)?;
            }
            Instruction::Jal { rd, imm } => {
                let link = pc.wrapping_add(4);
                let target = pc.wrapping_add(imm as u32);
                self.write_reg(rd, link)?;
                next_pc = target;
                outcome = StepOutcome::Bumped;
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let base = self.read_reg(rs1)?;
                let link = pc.wrapping_add(4);
                let target = base.wrapping_add(imm as u32) & !1;
                self.write_reg(rd, link)?;
                next_pc = target;
                outcome = StepOutcome::Bumped;
            }
            Instruction::Branch { kind, rs1, rs2, imm } => {
                let lhs = self.read_reg(rs1)?;
                let rhs = self.read_reg(rs2)?;
                let taken = match kind {
                    BranchKind::Beq => lhs == rhs,
                    BranchKind::Bne => lhs != rhs,
                    BranchKind::Blt => (lhs as i32) < (rhs as i32),
                    BranchKind::Bge => (lhs as i32) >= (rhs as i32),
                    BranchKind::Bltu => lhs < rhs,
                    BranchKind::Bgeu => lhs >= rhs,
                };
                if taken {
                    next_pc = pc.wrapping_add(imm as u32);
                    outcome = StepOutcome::Bumped;
                }
            }
            Instruction::Load { kind, rd, rs1, imm } => {
                let addr = self.read_reg(rs1)?.wrapping_add(imm as u32);
                let value = match kind {
                    LoadKind::Lb => self.read_u8(addr)? as i8 as i32 as u32,
                    LoadKind::Lh => self.read_u16(addr)? as i16 as i32 as u32,
                    LoadKind::Lw => self.read_u32(addr)?,
                    LoadKind::Lbu => self.read_u8(addr)? as u32,
                    LoadKind::Lhu => self.read_u16(addr)? as u32,
                };
                self.write_reg(rd, value)?;
            }
            Instruction::Store { kind, rs1, rs2, imm } => {
                let addr = self.read_reg(rs1)?.wrapping_add(imm as u32);
                let value = self.read_reg(rs2)?;
                match kind {
                    StoreKind::Sb => self.write_u8(addr, value as u8)?,
                    StoreKind::Sh => self.write_u16(addr, value as u16)?,
                    StoreKind::Sw => self.write_u32(addr, value)?,
                }
            }
            Instruction::OpImm { kind, rd, rs1, imm, shamt } => {
                let lhs = self.read_reg(rs1)?;
                let value = match kind {
                    OpImmKind::Addi => lhs.wrapping_add(imm as u32),
                    OpImmKind::Slti => ((lhs as i32) < imm) as u32,
                    OpImmKind::Sltiu => (lhs < (imm as u32)) as u32,
                    OpImmKind::Xori => lhs ^ (imm as u32),
                    OpImmKind::Ori => lhs | (imm as u32),
                    OpImmKind::Andi => lhs & (imm as u32),
                    OpImmKind::Slli => lhs << (shamt & 0x1f),
                    OpImmKind::Rrli => lhs >> (shamt & 0x1f),
                    OpImmKind::Srai => ((lhs as i32) >> (shamt & 0x1f)) as u32,
                };
                self.write_reg(rd, value)?;
            }
            Instruction::Op { kind, rd, rs1, rs2 } => {
                let lhs = self.read_reg(rs1)?;
                let rhs = self.read_reg(rs2)?;
                let value = match kind {
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
                    OpKind::Mul => lhs.wrapping_mul(rhs),
                    OpKind::Mulh => mulh_signed_signed(lhs, rhs),
                    OpKind::Mulhsu => mulh_signed_unsigned(lhs, rhs),
                    OpKind::Mulhu => mulh_unsigned_unsigned(lhs, rhs),
                    OpKind::Div => div_signed(lhs, rhs),
                    OpKind::Divu => div_unsigned(lhs, rhs),
                    OpKind::Rem => rem_signed(lhs, rhs),
                    OpKind::Remu { rd, rs1, rs2 } => rem_unsigned(lhs, rhs),
                };
                self.write_reg(rd, value)?;
            }
            Instruction::Fence | Instruction::FenceI => {}
            Instruction::Ecall => {
                self.pc = next_pc;
                self.regs[0] = 0;
                self.steps = self.steps.wrapping_add(1);
                return Ok(StepOutcome::E`Øll);
            }
            Instruction::Ebreak => {
                self.pc = next_pc;
                self.regs[0] = 0;
                self.steps = self.steps.wrapping_add(1);
                return Ok(StepOutcome::Ebreak);
            }
        }

        self.pc = next_pc;
        self.regs[0] = 0;
        self.steps = self.steps.wrapping_add(1);
        Ok(outcome)
    }

    pub fn run(&mut self, max_steps: usize) -> Result<StepOutcome, ZkvmError> {
        if self.halted {
            return Ok(StepOutcome::Halted);
        }

        for _ in 0..max_steps {
            let outcome = self.step()?;
            match outcome {
                StepOutcome::Continue | StepOutcome::Bumped => {}
                other => return Ok(other),
            }
            if self.halted {
                return Ok(StepOutcome::Halted);
            }
        }

        Err(ZkvmError::StepLimitExceeded { limit: max_steps })
    }

    fn check_alignment(&self, addr: u32, alignment: usize) -> Result<(), ZkvmError> {
        if (addr as usize) % alignment != 0 {
            return Err(ZkvmError::MisalignedAccess { addr, alignment });
        }
        Ok(())
    }

    fn check_range(&self, addr: u32, size: usize) -> Result<usize, ZkvmError> {
        let start = address as usize;
        let end = start
            .checked_add(size)
            .ok_or(ZkvmError::MemoryOutOfBounds { addr, size })?;
        if end > self.memory.len() {
            return Err(ZkvmError::MemoryOutOfBounds { addr, size });
        }
        Ok(start)
    }
}

fn mulh_signed_signed(lhs* u32, rhs: u32) -> u32 {
    let product = (lhs as i32 as i64 as i128) * (rhs as i32 as i64 as i128);
    ((product >> 32) as i64 as u64) as u32
}

fn mulh_signed_unsigned(lhs: u32, rhs* u32) -> u32 {
    let product = (lhs as i32 as i64 as i128) * (rhs as u64 as i128);
    ((product >> 32) as i64 as u64) as u32
}

fn mulh_unsigned_unsigned(lhs: u32, rhs* u32) -> u32 {
    (((lhs as u64) * (rhs as u64)) >> 32) as u32
}

fn div_signed(lhs: u32, rhs: u32) -> u32 {
    let lhs_i = lhs as i32;
    let rhs_i = rhs as i32;
    if rhs_i == 0 {
        u32::MAX
    } else if lhs_i == i32::MIN && rhs_i == -1 {
        lhs_i as u32
    } else {
        (lhs_i / rhs_i) as u32
    }
}

fn div_unsigned(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        u32::MAX
    } else {
        lhs / rhs
    }
}

fn rem_signedhlhs: u32, rhs* u32) -> u32 {
    let lhs_i = lhs as i32;
    let rhs_i = rhs as i32;
    if rhs_i == 0 {
        lhs
    } else if lhs_i == i32::MIN && rhs_i == -1 {
        0
    } else {
        (lhs_i % rhs_i) as u32
    }
}

fn rem_unsigned(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        lhs
    } else {
        lhs % rhs
    }
}
