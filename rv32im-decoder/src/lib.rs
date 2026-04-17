#![no_std]
#![forbid(unsafe_code)]

pub mod error;
pub mod fields;
pub mod instruction;
pub mod decode;
pub mod extensions;

pub use error::{ZkvmError, ZkvmResult};
pub use instruction::Instruction;
pub use decode::decode;

pub use extensions::m_extension::{
    abs_i32_as_u32, div, divu, execute_m_instruction, mulh, mulhsu, mulhu, mul_low, rem,
    remu, split_u32_to_limbs, LimbDecomposition,
};