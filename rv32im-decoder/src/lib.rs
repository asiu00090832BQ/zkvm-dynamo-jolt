#![no_std]
#![forbid(unsafe_code)]

pub mod error;
pub mod formats;
pub mod instruction;
pub mod invariants;
pub mod m_extension;
pub mod selectors;

pub use error::DecoderError;
pub use instruction::{DecodedInstruction, Instruction};
pub use m_extension::{div, divu, mul, mulh, mulhsu, mulhu, rem, remu};
pub use selectors::decode;
