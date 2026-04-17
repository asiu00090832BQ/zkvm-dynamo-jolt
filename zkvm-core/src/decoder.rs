pub use rv32im_decoder::{Instruction, DecodeError, Register, decode};

pub struct Decoded {
    pub word: u32,
    pub instruction: Instruction,
}

pub struct Decoder;

impl Decoder {
    pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
        rv32im_decoder::decode(word)
    }
}
