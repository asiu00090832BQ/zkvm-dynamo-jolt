mod error;
mod instruction;
pub mod decoder;

pub use decoder::{
    decode, decode_base, decode_m_extension, div_signed, div_unsigned, mul_low, mul_u32_wide,
    mulh_signed, mulh_signed_unsigned, mulhu, rem_signed, rem_unsigned,
};
pub use error::ZkvmError;
pub use instruction::{Instruction, Register};
