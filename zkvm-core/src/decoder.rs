pub use rv32im_decoder::{DecodeError, Instruction};

pub fn decode_instruction(word: u32) -> Result<Instruction, DecodeError> {
    rv32im_decoder::decode(word)
}
