use crate::error::{DecodeError, Result};
use crate::fields;
use crate::format::InstructionFormat;
use crate::imm;
use crate::opcode::MajorOpcode;
use crate::types::{DecodedInstruction, Extension, Mnemonic, Register};

pub fn decode_basei(word: u32, opcode: MajorOpcode) -> Result<DecodedInstruction> {
    Ok(DecodedInstruction::new(
        word,
        opcode,
        InstructionFormat::Unknown,
        Extension::BaseI,
        Mnemonic::Addi,
    ))
}
