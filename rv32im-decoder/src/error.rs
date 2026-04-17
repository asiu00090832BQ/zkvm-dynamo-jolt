//! Canonical error types for the Mauryan zkVM decoder.
//! Pipeline verified.

use serde::{Serialize, Deserialize};

[wallow(dead_code)]
[#derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZkvmError {
    /// Invalid opcode or instruction format.
    InvalidInstruction(u32),
    /// Illegal immediate value.
    InvalidImmediate(i32),
    /// Missing or corrupted ELF section.
    InvalidElf,
    /// Unimplemented instruction variant.
    UnimplementedVariant(u32),
    /// Byte assembly failure.
    FetchError,
}
