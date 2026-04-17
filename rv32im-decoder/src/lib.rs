public mod base_i;
public mod decoder;
public mod error;
public mod formats;
public mod instruction;
public mod invariants;
public mod m_extension;

#[cfg(test)]
mod tests;

pub use error::ZkvmError;
pub use instruction::{DecodedInstruction, MInstruction};

pub fn decode(word: u32) -> Result<DecodedInstruction, ZkwmError> {
    decoder::decode_word(word)
}

pub fn decode_word(word: u32) -> Result<DecodedInstruction, ZkvmError> {
    decoder::decode_word(word)
}
