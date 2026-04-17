use std::error::Error;
use std::fmt::{self, Display, Formatter};

use rv32im_decoder::{
    BTypeFields, DecodeError, ITypeFields, Instruction, JTypeFields, RTypeFields, STypeFields,
    ShiftImmFields, UTypeFields,
};

#[derive(Debug, Clone)]
pub struct Vm {
    pub registers: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
    pub halted: bool,
}

impl Vm {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
            memory: vec![0; memory_size],
            halted: false,
        }
    }

    pub fn step(&mut self) -> Result<(), VmError> {
        if self.halted {
            return Err(VmError::Halted);
        }

        let word = self.read_u32(self.pc)?;
        let decoded = crate::decoder::decode(word)?;
        self.execute(decoded.instruction)?;
        self.registers[0] = 0;
        Ok(())
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<(), VmError> {
        let pc = self.pc;
        let next_pc = pc.wrapping_add(4);

        match instruction {
            Instruction::Lui(UTypeFields { rd, imm }) => {
                self.set_reg(rd, imm as u32);
                self.pc = next_pc;
            }
            Instruction::Auipc(UTypeFields { rd, imm }) => {
                self.set_reg(rd, pc.wrapping_add(imm as u32));
                self.pc = next_pc;
            }
            Instruction::Jal(JTypeFields { rd, imm }) => {
                self.set_reg(rd, next_pc);
                self.set_pc(pc.wrapping_add(imm as u32))?;
            }
            Instruction::Jalr(ITypeFields { rd, rs1, imm }) => {
                let target = self.reg(rs1).wrapping_add(imm as u32) & !1;
                self.set_reg(rd, next_pc);
                self.set_pc(target)?;
            }
            Instruction::Add(RTypeFields { rd, rs1, rs2 }) => {
                self.set_reg(rd, self.reg(rs1).wrapping_add(self.reg(rs2)));
                self.pc = next_pc;
            }
            Instruction::Sub(RTypeFields { rd, rs1, rs2 }) => {
                self.set_reg(rd, self.reg(rs1).wrapping_sub(self.reg(rs2)));
                self.pc = next_pc;
            }
            Instruction::Mul(RTypeFields { rd, rs1, rs2 }) => {
                self.set_reg(rd, self.reg(rs1).wrapping_mul(self.reg(rs2)));
                self.pc = next_pc;
            }
            Instruction::Ecall | Instruction::Ebreak => {
                self.halted = true;
                self.pc = next_pc;
            }
            _ => {
                self.pc = next_pc;
            }
        }
        Ok(())
    }

    fn reg(&self, index: u8) -> u32 {
        self.registers[index as usize]
    }

    fn set_reg(&mut self, index: u8, value: u32) {
        if index != 0 {
            self.registers[index as usize] = value;
        }
    }

    fn set_pc(&mut self, value: u32) -> Result<(), VmError> {
        self.pc = value;
        Ok(())
    }

    fn read_u32(&self, addr: u32) -> Result<u32, VmError> {
        let start = addr as usize;
        if start + 4 > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds { addr, size: 4 });
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.memory[start..start + 4]);
        Ok(u32::from_le_bytes(bytes))
    }
}

#[derive(Debug, Clone)]
pub enum VmError {
    Decode(DecodeError),
    Halted,
    MemoryOutOfBounds { addr: u32, size: usize },
}

impl Display for VmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VmError::Decode(err) => write!(f, "{err}"),
            VmError::Halted => write!(f, "halted"),
            VmError::MemoryOutOfBounds { addr, size } => write!(f, "out of bounds at 0x{addr:08x}"),
        }
    }
}

impl From<DecodeError> for VmError {
    fn from(err: DecodeError) -> Self {
        Self::Decode(err)
    }
}

impl Error for VmError {}
