use crate::types::{Address, Instruction, RegisterIndex, Word};
use rv32im_decoder::{
    decode, execute_mul_kind, BranchKind, DecoderError, LoadKind, OpImmKind, OpKind, StoreKind,
    SystemKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZkvmConfig {
    pub reset_pc: Address,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self { reset_pc: 0 }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ZkvmError {
    Decode(DecoderError),
    MemoryOutOfBounds { addr: Address, size: usize },
    MisalignedAccess { addr: Address, alignment: Address },
    UnsupportedSystem { kind: SystemKind },
}

pub struct Zkvm<'a> {
    config: ZkvmConfig,
    regs: [Word; 32],
    pc: Address,
    memory: &'a mut [u8],
}

impl<'a> Zkvm<'a> {
    pub fn new(memory: &'a mut [u8], config: ZkvmConfig) -> Self {
        Self {
            config,
            regs: [0; 32],
            pc: config.reset_pc,
            memory,
        }
    }

    pub fn reset(&mut self) {
        self.regs = [0; 32];
        self.pc = self.config.reset_pc;
    }

    #[inline]
    pub const fn pc(&self) -> Address {
        self.pc
    }

    #[inline]
    pub fn reg(&self, index: RegisterIndex) -> Word {
        if index == 0 {
            0
        } else {
            self.regs[index as usize]
        }
    }

    #[inline]
    fn write_reg(&mut self, index: RegisterIndex, value: Word) {
        if index != 0 {
            self.regs[index as usize] = value;
        }
    }

    pub fn step(&mut self) -> Result<(), ZkvmError> {
        let word = self.fetch_word(self.pc)?;
        let instruction = decode(word).map_err(ZkvmError::Decode)?;
        self.execute_instruction(instruction)
    }

    pub fn execute_word(&mut self, word: Word) -> Result<(), ZkvmError> {
        let instruction = decode(word).map_err(ZkvmError::Decode)?;
        self.execute_instruction(instruction)
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), ZkvmError> {
        let next_pc = self.pc.wrapping_add(4);

        match instruction {
            Instruction::Lui { rd, imm } => {
                self.write_reg(rd, imm);
                self.pc = next_pc;
            }
            Instruction::Auipc { rd, imm } => {
                let val = self.pc.wrapping_add(imm);
                self.write_reg(rd, val);
                self.pc = next_pc;
            }
            Instruction::Jal { rd, imm } => {
                self.write_reg(rd, next_pc);
                self.pc = self.pc.wrapping_add(imm as u32);
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.reg(rs1).wrapping_add(imm as u32) & !1;
                self.write_reg(rd, next_pc);
                self.pc = target;
            }
            Instruction::Branch { kind, rs1, rs2, imm } => {
                let lhs = self.reg(rs1);
                let rhs = self.reg(rs2);
                if Self::branch_taken(kind, lhs, rhs) {
                    self.pc = self.pc.wrapping_add(imm as u32);
                } else {
                    self.pc = next_pc;
                }
            }
            Instruction::Load { kind, rd, rs1, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                let val = match kind {
                    LoadKind::Byte => self.load_u8(addr)? as i8 as i32 as u32,
                    LoadKind::Half => self.load_u16(addr)? as i16 as i32 as u32,
                    LoadKind::Word => self.load_u32(addr)?,
                    LoadKind::ByteUnsigned => self.load_u8(addr)? as u32,
                    LoadKind::HalfUnsigned => self.load_u16(addr)? as u32,
                };
                self.write_reg(rd, val);
                self.pc = next_pc;
            }
            Instruction::Store { kind, rs1, rs2, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                let val = self.reg(rs2);
                match kind {
                    StoreKind::Byte => self.store_u8(addr, val as u8)?,
                    StoreKind::Half => self.store_u16(addr, val as u16)?,
                    StoreKind::Word => self.store_u32(addr, val)?,
                }
                self.pc = next_pc;
            }
            Instruction::OpImm { kind, rd, rs1, imm } => {
                let lhs = self.reg(rs1);
                let shamt = (imm as u32) & 0x1f;
                let val = match kind {
                    OpImmKind::Addi => lhs.wrapping_add(imm as u32),
                    OpImmKind::Slti => {
                        if (lhs as i32) < imm {
                            1
                        } else {
                            0
                        }
                    }
                    OpImmKind::Sltiu => {
                        if lhs < imm as u32 {
                            1
                        } else {
                            0
                        }
                    }
                    OpImmKind::Xori => lhs ^ imm as u32,
                    OpImmKind::Ori => lhs | imm as u32,
                    OpImmKind::Andi => lhs & imm as u32,
                    OpImmKind::Slli => lhs << shamt,
                    OpImmKind::Srli => lhs >> shamt,
                    OpImmKind::Srai => ((lhs as i32) >> shamt) as u32,
                };
                self.write_reg(rd, val);
                self.pc = next_pc;
            }
            Instruction::Op { kind, rd, rs1, rs2 } => {
                let lhs = self.reg(rs1);
                let rhs = self.reg(rs2);
                let shamt = rhs & 0x1f;
                let val = match kind {
                    OpKind::Add => lhs.wrapping_add(rhs),
                    OpKind::Sub => lhs.wrapping_sub(rhs),
                    OpKind::Sll => lhs << shamt,
                    OpKind::Slt => {
                        if (lhs as i32) < (rhs as i32) {
                            1
                        } else {
                            0
                        }
                    }
                    OpKind::Sltu => {
                        if lhs < rhs {
                            1
                        } else {
                            0
                        }
                    }
                    OpKind::Xor => lhs ^ rhs,
                    OpKind::Srl => lhs >> shamt,
                    OpKind::Sra => ((lhs as i32) >> shamt) as u32,
                    OpKind::Or => lhs | rhs,
                    OpKind::And => lhs & rhs,
                };
                self.write_reg(rd, val);
                self.pc = next_pc;
            }
            Instruction::Mul { kind, rd, rs1, rs2 } => {
                let lhs = self.reg(rs1);
                let rhs = self.reg(rs2);
                let val = execute_mul_kind(kind, lhs, rhs);
                self.write_reg(rd, val);
                self.pc = next_pc;
            }
            Instruction::Fence | Instruction::FenceI => {
                self.pc = next_pc;
            }
            Instruction::System(kind) => {
                return Err(ZkvmError::UnsupportedSystem { kind });
            }
        }

        Ok(())
    }

    #[inline]
    fn branch_taken(kind: BranchKind, lhs: Word, rhs: Word) -> bool {
        match kind {
            BranchKind::Beq => lhs == rhs,
            BranchKind::Bne => lhs != rhs,
            BranchKind::Blt => (lhs as i32) < (rhs as i32),
            BranchKind::Bge => (lhs as i32) >= (rhs as i32),
            BranchKind::Bltu => lhs < rhs,
            BranchKind::Bgeu => lhs >= rhs,
        }
    }

    fn fetch_word(&self, addr: Address) -> Result<Word, ZkvmError> {
        self.load_u32(addr)
    }

    fn ensure_range(&self, addr: Address, size: usize) -> Result<usize, ZkvmError> {
        let start = addr as usize;
        let end = start
            .checked_add(size)
            .ok_or(ZkvmError::MemoryOutOfBounds { addr, size })?;
        if end > self.memory.len() {
            return Err(ZkvmError::MemoryOutOfBounds { addr, size });
        }
        Ok(start)
    }

    #[inline]
    fn ensure_alignment(addr: Address, alignment: Address) -> Result<(), ZkvmError> {
        if addr & (alignment - 1) == 0 {
            Ok(())
        } else {
            Err(ZkvmError::MisalignedAccess { addr, alignment })
        }
    }

    fn load_u8(&self, addr: Address) -> Result<u8, ZkvmError> {
        let start = self.ensure_range(addr, 1)?;
        Ok(self.memory[start])
    }

    fn load_u16(&self, addr: Address) -> Result<u16, ZkvmError> {
        Self::ensure_alignment(addr, 2)?;
        let start = self.ensure_range(addr, 2)?;
        Ok(u16::from_le_bytes([self.memory[start], self.memory[start + 1]]))
    }

    fn load_u32(&self, addr: Address) -> Result<u32, ZkvmError> {
        Self::ensure_alignment(addr, 4)?;
        let start = self.ensure_range(addr, 4)?;
        Ok(u32::from_le_bytes([
            self.memory[start],
            self.memory[start + 1],
            self.memory[start + 2],
            self.memory[start + 3],
        ]))
    }

    fn store_u8(&mut self, addr: Address, value: u8) -> Result<(), ZkvmError> {
        let start = self.ensure_range(addr, 1)?;
        self.memory[start] = value;
        Ok(())
    }

    fn store_u16(&mut self, addr: Address, value: u16) -> Result<(), ZkvmError> {
        Self::ensure_alignment(addr, 2)?;
        let start = self.ensure_range(addr, 2)?;
        let bytes = value.to_le_bytes();
        self.memory[start] = bytes[0];
        self.memory[start + 1] = bytes[1];
        Ok(())
    }

    fn store_u32(&mut self, addr: Address, value: u32) -> Result<(), ZkvmError> {
        Self::ensure_alignment(addr, 4)?;
        let start = self.ensure_range(addr, 4)?;
        let bytes = value.to_le_bytes();
        self.memory[start] = bytes[0];
        self.memory[start + 1] = bytes[1];
        self.memory[start + 2] = bytes[2];
        self.memory[start + 3] = bytes[3];
        Ok(())
    }
}
