pub mod decoder;
pub mod error;
pub mod instruction;
pub mod types;

pub use crate::decoder::{decode, Decoder};
pub use crate::error::{Result, ZkvmError};
pub use crate::instruction::{BaseIInstruction, Instruction, MInstruction};
pub use crate::types::{
    bits, mul_u32_via_limbs, sign_extend, DecodedFields, LimbDecomposition16, Register,
    SignedWord, Word, REGISTER_COUNT, ZERO_REGISTER,
};

pub mod base_i {
    pub use crate::instruction::BaseIInstruction;
}

pub mod m_extension {
    pub use crate::instruction::MInstruction;
}
