#![no_std]

pub mod decoder;
pub mod error;
pub mod i_extension;
pub mod instruction;
pub mod m_extension;
pub mod selectors;
pub mod types;
pub mod util;

pub use crate::decoder::{decode, Decoder};
pub use crate::error::DecoderError;
pub use crate::instruction::{
    BranchKind, Instruction, LoadKind, MulKind, OpImmKind, OpKind, StoreKind, SystemKind,
};
pub use crate::m_extension::{execute_mul_kind, lemma_6_1_1_product, mulh, mulhsu, mulhu, Product64};
pub use crate::types::{Immediate, InstructionWord, RegisterIndex, Word};
