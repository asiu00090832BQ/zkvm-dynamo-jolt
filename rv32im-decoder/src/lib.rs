#![no_std]

pub mod decoder;
pub mod error;
pub mod instruction;
pub mod m_extension;
pub mod selectors;

pub use decoder::decode;
pub use error::DecodeError;
pub use instruction::Instruction;
