#![forbid(unsafe_code)]

mod decode;
pub mod decoder;
pub mod instruction;
pub mod limbs;
pub mod m_extension;

pub use decoder::{decode, DecodeError};
pub use instruction::Instruction;
pub use limbs::{Limb16, WideMul16};
pub use m_extension::{
    decode_m_instruction, div_i32, divu_u32, mul_low_u32, mulh_i32, mulhsu_i32_u32, mulhu_u32,
    rem_i32, remu_u32,
};
