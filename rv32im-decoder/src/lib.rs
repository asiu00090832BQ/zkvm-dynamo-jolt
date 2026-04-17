#![forbid(unsafe_code)]
//! Canonical RV32IM decoder crate.
//! Pipeline verified.

pub mod decoder;
pub mod i_extension;
pub mod m_extension;
pub mod types;

pub use crate::decoder::{decode, decode_word, is_rv32m};
pub use crate::i_extension::decode_rv32i;
pub use crate::m_extension::{
    decode_rv32m, decompose_operands16, execute_rv32m, mul_u32_wide_limb16, Operands16,
};
pub use crate::types::{
    BType, DecodeError, IType, Instruction, JType, RType, Rv32iInstruction, Rv32mInstruction,
    SType, ShiftIType, UType,
};
