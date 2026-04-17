use core::fmt;
use crate::decoder::{decode_word, execute_rv32m, execute_i_extension, DecodedInstruction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZkvmConfig { pub mem_size: usize }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome { Continue, Halt }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zkvm {
    pub pc: u32,
    pub registers: [u32; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkvmError {
    Decode(crate::decoder::DecodeError),
    MemoryOutOfBounds { addr: u32, len: usize },
    InvalidElf,
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

impl From<crate::decoder::DecodeError> for ZkvmError {
    fn from(value: crate::decoder::DecodeError) -> Self { Self::Decode(value) }
}

impl Default for Zkvm {
    fn default() -> Self { Self { pc: 0, registers: [0; 32] } }
}

impl Zkvm {
    pub fn new() -> Self { Self::default() }
    pub fn step(&mut self, instruction_word: u32) -> Result<StepOutcome, ZkvmError> {
        let decoded = decode_word(instruction_word).map_err(ZkvmError::Decode)?;
        match decoded {
            DecodedInstruction::I(inst) => {
                let rs1_val = self.registers[inst.fields().rs1 as usize];
                let val = execute_i_extension(inst, rs1_val);
                self.set_reg(inst.fields().rd, val);
            }
            DecodedInstruction::M(inst) => {
                let rs1_val = self.registers[inst.fields().rs1 as usize];
                let rs2_val = self.registers[inst.fields().rs2 as usize];
                let val = execute_rv32m(inst, rs1_val, rs2_val);
                self.set_reg(inst.fields().rd, val);
            }
        }
        self.pc = self.pc.wrapping_add(4);
        Ok(StepOutcome::Continue)
    }
    fn set_reg(&mut self, rd: u8, val: u32) {
        if rd != 0 { self.registers[rd as usize] = val; }
    }
}
