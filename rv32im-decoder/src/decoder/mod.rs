pub mod rv32i;
pub mod rv32m;
pub mod sign_ext;

use crate::error::DecodeError;
use crate::fields::{funct7, opcode};
use crate::invariants;
use crate::selectors::SelectorRow;
use crate::types::Instruction;

pub trait Decoder {
    fn decode(&self, word: u32) -> Result<Instruction, DecodeError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Rv32ImDecoder;

impl Rv32ImDecoder {
    pub const fn new() -> Self {
        Self
    }
}

impl Decoder for Rv32ImDecoder {
    fn decode(&self, word: u32) -> Result<Instruction, DecodeError> {
        let instruction = match (opcode(word), funct7(word)) {
            (0b0110011, 0b0000001) => rv32m::decode_rv32m(word)?,
            _ => rv32i::decode_rv32i(word)?,
        };

        invariants::validate_instruction(&instruction)?;
        SelectorRow::from_instruction(&instruction).validate()?;
        Ok(instruction)
    }
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let decoder = Rv32ImDecoder;
    decoder.decode(word)
}
