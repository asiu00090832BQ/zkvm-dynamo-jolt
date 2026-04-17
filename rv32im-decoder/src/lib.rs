pub mod decoder;
pub mod error;
pub mod instruction;
pub mod m_extension;
pub mod util;

pub use decoder::decode;
pub use error::ZkvmError;
pub use instruction::Instruction;
pub use m_extension::{
    decompose_u32_to_u16_limbs,
    mul_u32_via_u16_limbs,
    signed_mulh,
    signed_unsigned_mulh,
    unsigned_mulh,
};
