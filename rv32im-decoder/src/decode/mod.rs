pub mod i;
pub mod m;

use crate::{
    error::{DecodeError, DecodeResult},
    fields::Fields,
    instruction::Instruction,
    opcode::Opcode,
    word::Word,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Decoder;

impl Decoder {
    pub const fn new() -> Self {
        Self
    }

    pub fn decode(&self, word: u32) -> DecodeResult<Instruction> {
        decode(word)
    }
}

pub fn decode(word: u32) -> DecodeResult<Instruction> {
    let word = Word::new(word);

    if !word.is_standard_32() {
        return Err(DecodeError::Non32BitInstruction {
            low_bits: (word.raw() & 0b11) as u8,
        });
    }

    let fields = Fields::from_word(word);
    let opcode = Opcode::decode(fields.opcode)?;

    match opcode {
        Opcode::Op if fields.funct7 == 0b0000001 => m::decode(fields),
        _ => i::decode(word, fields, opcode),
    }
}
