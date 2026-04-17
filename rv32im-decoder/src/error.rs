use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecoderError {
    InvalidInstruction(u32),
    InvalidElf,
    UnsupportedFunct3 { raw: u32, funct3: u8 },
    UnsupportedFunct7 { raw: u32, funct7: u8 },
    UnknownOpcode { raw: u32, opcode: u8 },
    InvariantViolation(''static str),
}

pub type DecodeResult<T> = Result<T, DecoderError>;
