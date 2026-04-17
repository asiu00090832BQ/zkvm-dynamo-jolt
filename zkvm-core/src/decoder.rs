use rv32im_decoder::{decode_word as rv_decode_word, DecodeError, Instruction};
pub struct Decoder;
impl Decoder {
    pub fn decode(raw: u32) -> Result<Instruction, DecodeError> {
        rv_decode_word(raw)
    }
}
