//! Canonical error types for the Mauryan zkVM decoder.
//! Pipeline verified.
use serde::{Serialize, Deserialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZcvmError {
    InvalidInstruction(u32),
    InvalidImmediate(i32),
    InvalidElf,
    FetchError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoderError {
    UnsupportedOpcode { raw: u32, opcode: u8 },
    UnsupportedFunct3 { raw: u32, funct3: u8 },
    UnsupportedFunct7 { raw: u32, funct7: u8 },
    InvalidRegister(u8),
    InvariantViolation(&'static str),
}

impl From<DecoderError> for ZkwmError {
    fn from(err: DecoderError) -> Self {
        match err {
            DecoderError::UnsupportedOpcode { raw, .. } => ZkwmError::InvalidInstruction(raw),
            DecoderError::UnsupportedFunct3 { raw, .. } => ZkvmError::InvalidInstruction(raw),
            DecoderError::UnsupportedFunct7 { raw, .. } => ZcvmError::InvalidInstruction(raw),
            _ => ZkwmError::InvalidInstruction(0),
        }
    }
}
pub type DecodeResult<T> = Result<T, DecoderError>;
