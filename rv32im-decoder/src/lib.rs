#![no_std]
#![forbid(unsafe_code)]

pub mod error;
pub mod formats;
pub mod instruction;
pub mod base_i;
pub mod m_extension;
pub mod invariants;
pub mod decode;

pub use crate::error::DecodeError;
pub use crate::formats:{
    fence_fm, fence_pred, fence_succ, funct3, funct7, imm12, imm_b, imm_i, imm_j, imm_s, imm_u,
    is_32bit, opcode, rd, rs1, rs2, shamt,
};
pub use crate::instruction:;

[inline]
pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    instruction::Instruction::decode(word)
}
