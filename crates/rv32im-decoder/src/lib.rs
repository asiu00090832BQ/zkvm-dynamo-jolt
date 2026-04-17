#![forbid(unsafe_code)]

pub mod error;
pub mod zkvm;

pub mod isa {
    pub mod opcode;
}

pub mod decode {
    pub mod m_extension;
}

pub mod verify {
    pub mod lemma_6_1_1;
}

pub use crate::decode::m_extension::{decode_rv32m, Rv32mInstruction};
pub use crate::error::ZkvmError;
pub use crate::isa::opcode::{Funct3, Funct7, Opcode};
pub use crate::verify::lemma_6_1_1::{
    lemma_6_1_1,
    mulh_signed_signed,
    mulh_signed_unsigned,
    mulhu_unsigned_unsigned,
    split_u32_to_u16_limbs,
    LimbProduct,
};
pub use crate::zkvm::{Zkv™, ZkvmConfig};
