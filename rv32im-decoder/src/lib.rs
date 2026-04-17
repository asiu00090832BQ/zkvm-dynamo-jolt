#![forbid(unsafe_code)]

pub mod bits;
pub mod decode;
pub mod decoded;
pub mod error;
pub mod format;
pub mod instruction;
pub mod selectors;

pub use decoded::DecodedInstruction;
pub use error::DecodeError;
pub use format::InstructionFormat;
pub use instruction::Instruction;

use selectors::{funct7, opcode};

pub fn decode(word: u32) -> Result<DecodedInstruction, DecodeError> {
    match opcode(word) {
        0x73 => decode::system::decode(word),
        0x33 if funct7(word) == 0x01 => decode::rv32m::decode(word),
        _ => decode::rv32i::decode(word),
    }
}

#[inline]
pub fn decode_instruction(word: u32) -> Result<DecodedInstruction, DecodeError> {
    decode(word)
}
