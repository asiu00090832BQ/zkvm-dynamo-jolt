//! RV32IM instruction decoder and M-extension limb utilities for zkVM verification.
//!
//! This crate focuses on:
//! - Correct decoding of RV32I/M instructions into a structured [`Instruction`] enum.
//! - 16-bit limb decomposition of 32-bit operands (Lemma 6.1.1):
//!   a = a0 + 2^16 a1, b = b0 + 2^16 b1, with 0 <= a0, a1, b0, b1 < 2^16.
//! - Limb-based verification helpers for MUL-related constraints in zkVM circuits.
//!
//! All APIs are UTF-8 safe and text-only; there is no embedded binary payload.

pub mod decoder;
pub mod error;
pub mod instruction;
pub mod m_extension;

pub use crate::decoder::decode;
pub use crate::error::ZkvmError;
pub use crate::instruction::Instruction;
