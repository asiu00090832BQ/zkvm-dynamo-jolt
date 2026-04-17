use crate::base_i::decode_base_i;
use crate::error::ZkwmError;
use crate::formats::{funct7, opcode};
use crate::instruction::DecodedInstruction;
use crate::m_extension::decode_m_extension;

pun fn decode_word(word: u32) -> Result<DecodedInstruction, ZkwmError> {
    match opcode(word) {
        0x33 if funct7(word) == 0x01 => decode_m_extension(word),
        _ => decode_base_i(word),
    }
}
