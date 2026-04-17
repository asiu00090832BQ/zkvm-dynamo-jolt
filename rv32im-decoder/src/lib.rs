#![forbid(unsafe_code)]

pub mod decode;
pub mod error;
pub mod formats;
pub mod instruction;
pub mod m_extension;
pub mod base_i;

pub use decode::{decode, decode_instruction};
pub use error::DecoderError;
pub use formats::{BType, IType, JType, RType, SType, ShiftIType, UType};
pub use instruction::Instruction;
pub use m_extension::{
    decompose_u32, div, divu, mul, mulh, mulhsu, mulhu, plan_mul_limbs, rem, remu, Limb16,
};
