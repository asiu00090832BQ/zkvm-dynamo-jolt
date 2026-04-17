use crate::{
    decoder::{base_i, invariants, m_ext},
    error::ZkvmError,
    types::Instruction,
};

pub fn decode_word(word: u32) -> Result<Instruction, ZkvmError> {
    invariants::ensure_standard_32bit(word)?;
    if m_ext::is_m_extension(word) {
        m_ext::decode(word)
    } else {
        base_i::decode(word)
    }
}

pub fn decode_hex(input: &str) -> Result<Instruction, ZkvmError> {
    let trimmed = input.trim();
    let normalized = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);

    let word = u32::from_str_radix(normalized, 16).map_err(|_| ZkvmError::ParseError {
        input: input.to_owned(),
    })?;

    decode_word(word)
}
