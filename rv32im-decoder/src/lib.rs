pub mod error;
pub mod instruction;
pub mod formats;
pub mod base_i;
pub mod m_extension;
pub mod invariants;

pub use crate::error::{DecodeResult, ZkvmError};
pub use crate::instruction::Instruction;

use crate::base_i::decode_i_instruction;
use crate::invariants::ensure_zkvm_symbol_parity;
use crate::m_extension::decode_m_instruction;

pub fn decode(word: u32) -> DecodeResult<Instruction> {
    ensure_zkvm_symbol_parity()?;

    match (word & 0x7f) as u8 {
        0x33 if ((word >> 25) & 0x7f) == 0x01 => decode_m_instruction(word),
        0x03 | 0x13 | 0x17 | 0x23 | 0x33 | 0x37 | 0x63 | 0x67 | 0x6f | 0x73 => {
            decode_i_instruction(word)
        }
        opcode => Err(ZkvmError::UnknownOpcode { raw: word, opcode }),
    }
}
