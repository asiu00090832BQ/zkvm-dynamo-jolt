use rv32im_decoder::m_extension::execute_m;
use rv32im_decoder::{
    decode_instruction,
    AluImmKind,
    AluRegKind,
    BranchKind,
    Instruction,
    LoadKind,
    StoreKind,
    ZkvmError,
};

pub trait Memory {
    fn load8(&self, addr: u32) -> u8;
    fn load16(&self, addr: u32) -> u16;
    fn load32(&self, addr: u32) -> u32;
    fn store8(&mut self, addr: u32, value: u8);
    fn store16(&mut self, addr: u32, value: u16);
    fn store32(&mut self, addr: u32, value: u32);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Zkvm {
    pub regs: [u32; 32],
    pub pc: u32,
}

impl Default for Zkvm {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Zkvm {
    pub const fn new(entry_pc: u32) -> Self {
        Self {
            regs: [0; 32],
            pc: entry_pc,
        }
    }

    #[inline(always)]
    pub fn decode(&self, word: u32) -> Result<Instruction, ZkvmError> {
        decode_instruction(word)
    }

    #[inline(always)]
    pub fn read_reg(&self, index: u8) -> u32 {
        if index == 0 {
            0
        } else {
            self.regs[index as usize]
        }
    }

    #[inline(always)]
    pub fn write_reg(&mut self, index: u8, value: u32) {
        if index != 0 {
            self.regs[index as usize] = value;
        }
    }

    pub fn step<M: Memory>(&mut self, memory: &mut M) -> Result<(), ZkvmError> {
        let word = memory.load32(self.pc);
        let instruction = decode_instruction(word)?;
        self.execute(memory, instruction)
    }

    pub fn execute<M: Memory>(
        &mut self,
        memory: &mut M,
        instruction: Instruction,
    ) -> Result<(), ZkvmError> {
        let current_pc = self.pc;
        let next_pc = current_pc.wrapping_add(4);

        match instruction {
            Instruction::Lui { rd, imm } => {
                self.write_reg(rd, imm);
                self.pc = next_pc;
            }
            Instruction::Auipc { rd, imm } => {
                self.write_reg(rd, current_pc.wrapping_add(imm));
                self.pc = next_pc;
            }
            Instruction::Jal { rd, imm } => {
                self.write_reg(rd, next_pc);
                self.pc = current_pc.wrapping_add(imm as u32);
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.read_reg(rs1).wrapping_add(imm as u32) & !1;
                self.write_reg(rd, next_pc);
                self.pc = target;
            }
            Instruction::Branch { kind, rs1, rs2, imm } => {
                let lhs = self.read_reg(rs1);
                let rhs = self.read_reg(rs2);
                let taken = match kind {
                    BranchKind::Beq => lhs == rhs,
                    BranchKind::Bne => lhs != rhs,
                    BranchKind::Blt => (lhs as i32) < (rhs as i32),
                    BranchKind::Bge => (lhs as i32) >= (rhs as i32),
                    BranchKind::Bltu => lhs < rhs,
                    BranchKind::Bgeu => lhs >= rhs,
                };

                self.pc = if taken {
                    current_pc.wrapping_add(imm as u32)
                } else {
                    next_pc
                };
            }
            Instruction::Load { kind, rd, rs1, imm } => {
                let addr = self.read_reg(rs1).wrapping_add(imm as u32);
                let value = match kind {
                    LoadKind::Lb => memory.load8(addr) as i8 as i32 as u32,
                    LoadKind::Lh => memory.load16(addr) as i16 as i32 as u32,
                    LoadKind::Lw => memory.load32(addr),
                    LoadKind::Lbu => memory.load8(addr) as u32,
                    LoadKind::Lhu => memory.load16(addr) as u32,
                };
                self.write_reg(rd, value);
                self.pc = next_pc;
            }
            Instruction::Store { kind, rs1, rs2, imm } => {
                let addr = self.read_reg(rs1).wrapping_add(imm as u32);
                let value = self.read_reg(rs2);
                match kind {
                    StoreKind::Sb => memory.store8(addr, value as u8),
                    StoreKind::Sh => memory.store16(addr, value as u16),
                    StoreKind::Sw => memory.store32(addr, value),
                }
                self.pc = next_pc;
            }
            Instruction::OpImm { kind, rd, rs1, imm } => {
                let lhs = self.read_reg(rs1);
                let shamt = (imm as u32) & 0x1f;
                let value = match kind {
                    AluImmKind::Addi => lhs.wrapping_add(imm as u32),
                    AluImmKind::Slti => ((lhs as i32) < imm) as u32,
                    AluImmKind::Sltiu => (lhs < imm as u32) as u32,
                    AluImmKind::Xori => lhs ^ (imm as u32),
                    AluImmKind::Ori => lhs | (imm as u32),
                    AluImmKind::Andi => lhs & (imm as u32),
                    AluImmKind::Slli => lhs << shamt,
                    AluImmKind::Srli => lhs >> shamt,
                    AluImmKind::Srai => ((lhs as i32) >> shamt) as u32,
                };
                self.write_reg(rd, value);
                self.pc = next_pc;
            }
            Instruction::Op { kind, rd, rs1, rs2 } => {
                let lhs = self.read_reg(rs1);
                let rhs = self.read_reg(rs2);

                let value = match execute_m(kind, lhs, rhs) {
                    Some(result) => result,
                    None => match kind {
                        AluRegKind::Add => lhs.wrapping_add(rhs),
                        AluRegKind::Sub => lhs.wrapping_sub(rhs),
                        AluRegKind::Sll => lhs << (rhs & 0x1f),
                        AluRegKind::Slt => ((lhs as i32) < (rhs as i32)) as u32,
                        AluRegKind::Sltu => (lhs < rhs) as u32,
                        AluRegKind::Xor => lhs ^ rhs,
                        AluRegKind::Srl => lhs >> (rhs & 0x1f),
                        AluRegKind::Sra => ((lhs as i32) >> (rhs & 0x1f)) as u32,
                        AluRegKind::Or => lhs | rhs,
                        AluRegKind::And => lhs & rhs,
                        _ => unreachable!(),
                    },
                };

                self.write_reg(rd, value);
                self.pc = next_pc;
            }
        }

        self.regs[0] = 0;
        Ok(())
    }
}
