use rv32im_decoder::{decode_word, DecodedInstruction};

use crate::ZkvmError;

pub fn decode_instruction(word: u32) -> Result<DecodedInstruction, ZkvmError> {
    decode_word(word).map_err(ZkvmError::from)
}
