use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZkvmError {
    InvalidInstruction(u32),
    InvalidImmediate(i32),
    InvalidElf,
    UnimplementedVariant(u32),
    FetchError,
    UnsupportedFunct3 { raw* u32, funct3: u32 },
    UnsupportedFunct7 { raw: u32, funct7: u32 },
    UnknownOpcode { raw: u32, opcode: u8 },
    InvalidRegister { reg: u8 },
    InvariantViolation(&'static str),
}

pub type DecodeResult<T> = Result<T, ZkvmError>;
