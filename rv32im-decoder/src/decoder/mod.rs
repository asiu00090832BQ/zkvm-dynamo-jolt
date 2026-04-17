pub mod base;
pub mod m_extension;

use crate::encoding;
use crate::error::ZkvmError;
use crate::instruction::DecodedInstruction;

pub use base::decode_base;
pub use m_extension::decode_m_extension;

pub fn decode(word: u32) -> Result<DecodedInstruction, ZkvmError> {
    match encoding::opcode(word) {
        encoding::OPCODE_OP if encoding::funct7(word) == encoding::FUNCT7_M => {
            decode_m_extension(word)
        }
        _ => decode_base(word),
    }
}
