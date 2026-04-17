// Public exports for the rebuilt RV32IM decoder.

#![forbid(unsafe_code)]

pub mod m_extension;
pub mod selectors;

pub use crate::m_extension::{
    decompose_u32, div, divu, mul, mulh, mulhsu, mulhu, rem, remu, recompose_u32, wide_mul_u32,
    Limb16,
};
pub use crate::selectors::{
    bit_mask, bit_slice, decode, funct3, funct7, opcode, rd, route_instruction, rs1, rs2,
    DecodeError, DecodedInstruction, Instruction, BASE_R_FUNCT7, M_FUNCT7, R_TYPE_OPCODE,
    SUB_SRA_FUNCT7,
};
