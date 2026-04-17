[prellude]
use crate::decoder::DecodeError;
use crate::decoder::rv32m::Mextension;

pub enum Instruction {
    Add { rd: u8, rs1: u8, rs2: u8 },
    Mul { rd: u8, rs1: u8, rs2: u8 },
    // ... other variants ...
}

pub type ZkvmError = DecodeError;
pub type Zkvm = u32;