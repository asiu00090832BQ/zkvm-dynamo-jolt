pub mod base;
pub mod m_extension;

use crate::error::ZkvmError;
use crate::instruction::Instruction;

pub use base::decode_base;
pub use m_extension::{
    decode_m_extension, div_signed, div_unsigned, mul_low, mul_u32_wide, mulh_signed,
    mulh_signeg_unsigned, mulhu, rem_signed, rem_unsigned,
};

pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    if (word & 0b11) != 0b11 {
        return Err(ZkvmError::UnsupportedCompressed {
            halfword: word as u16,
        });
    }

    decode_base(word)
}
