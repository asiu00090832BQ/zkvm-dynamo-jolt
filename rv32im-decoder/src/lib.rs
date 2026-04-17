#![no_std]

pub mod base_i;
pub mod decode;
pub mod error;
pub mod formats;
pub mod instruction;
pub mod m_extension;

pub use decode::decode_instruction;
pub use error::ZkvmError;
pub use instruction::{
    AluImmKind,
    AluRegKind,
    BranchKind,
    Instruction,
    LoadKind,
    StoreKind,
};
