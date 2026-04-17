#![forbid(unsafe_code)]
//! Canonical RV32IM decoder crate.

pub mod decoder;
pub mod i_extension;
pub mod m_extension;
pub mod types;

pub use crate::decoder::{decode, decode_word, is_rv32m};
pub use crate::i_extension::{decode_rv32i, execute_i_extension, IInstruction};
pub use crate::m_extension::{
    decode_rv32m, decompose_operands16, execute_rv32m, mul_u32_wide_limb16, MInstruction, Operands16,
};
pub use crate::types::{
    DecodeError, ITypeFields, RTypeFields, Instruction, DecodedInstruction
};
