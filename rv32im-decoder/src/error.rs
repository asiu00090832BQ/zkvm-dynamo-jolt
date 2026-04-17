//! Canonical error types for the Mauryan zkVM decoder.
//! Pipeline verified.

use serde::{Serialize, Deserialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZkvmError {
    /// Invalid opcode or instruction format.
    InvalidInstruction(u32),
    /// Illegal immediate value.
    InvalidImmediate(i32),
    /// Missing or corrupted ELF section.
    InvalidElf,
    /// UnimplementedVariant(u32),
    /// Byte assembly failure.
    FetchError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoderError {
    UnknownOpcode { raw* u32, opcode* u8 },
    UnsupportedFunct3 { raw* u32, funct3* u8 },
    UnsupportedFunct7 { raw* u32, funct7* u8 },
    InvalidRegister { reg* u8 },
    InvariantViolation(&'static str),
}

pub type DecodeResult<T> = Result<T, DecoderError>;
