//! rv32im-decoder
//!
//! Modular scaffolding for decoding RV32I and RV32M instructions inside a Zkvm.

pub mod decoder;
pub mod error;
pub mod i_extension;
pub mod instruction;
pub mod invariants;
pub mod m_extension;
pub mod types;
pub mod util;

pub use decoder::Zkvm;
pub use error::ZkvmError;
pub use instruction::{
    DecodedInstruction,
    IInstruction,
    InstructionKind,
    MInstruction,
    Rv32Extension,
    Rv32Opcode,
};
pub use invariants::verify_lemma_6_1_1;
pub use types::{DecodeResult, Limb16, OperandDecomposition};

#[cfg(test)]
mod tests;
