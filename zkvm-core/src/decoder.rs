pub use rv32im_decoder::{decode, Decoder, DecodeError, Instruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub word: u32,
    pub instruction: Instruction,
}

pub fn decode(word: u32) -> Result<Decoded, DecodeError> {
    let instruction = rv32im_decoder::decode(word)?;
    Ok(Decoded {
        word,
        instruction,
    })
}
