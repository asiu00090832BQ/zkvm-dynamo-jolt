#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

pub mod m_extension;
pub mod types;

pub use m_extension::{decode_m_extension, decompose_u32_to_u16_limbs, mul_low_u32};
pub use types::{Instruction, Result, ZkvmError};
