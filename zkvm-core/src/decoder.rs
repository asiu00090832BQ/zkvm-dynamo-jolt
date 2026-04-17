use crate::vm::ZkvmError;
pub use rv32im_decoder::{Instruction, DecodeError, Decoded, decode as decode_inner};

pub fn decode(word: u32) -> Result<Decoded, ZkvmError> {
    let instr = decode_inner(word).map_err(|_| ZkvmError::InvalidInstruction { pc: 0, raw: word })?;
    Ok(Decoded { word, instruction: instr })
}
