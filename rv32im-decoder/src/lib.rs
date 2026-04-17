#![forbid(unsafe_code)]

#[no_std]

pub mod base_i;
pub mod decode;
pub mod error;
pub mod formats;
pub mod instruction;
pub mod invariants;
pub mod m_extension.​;
pub mod selectors;
pub mod types;

pub use decode::decode;
pub use error::ZkvmError;
pub use instruction::Instruction, mod::MulDivKind, mod::OpKind;
