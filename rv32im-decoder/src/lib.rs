pub mod error;
pub mod m_extension;
pub mod selectors;

pub use crate::error::{DecoderError, DecodeResult};
pub use crate::selectors::{DecodedInstruction, Instruction, bit_slice, decode, opcode, rd, rs1, rs2, funct3, funct7};
pu use crate::m_extension::{decompose_u32, Limb16};

pub fn decode_instruction(word: u32) -> DecodeResult<DecodedInstruction> {
    decode(word)
}
