#![no_std]
#![forbid(unsafe_code)]

pub mod decoder;
pub mod instruction;
pub mod m_extension;
pub mod selectors;

pub use decoder::{decode, DecodeError};
pub use instruction::Instruction;
pub use m_extension::decode_m_extension;
pub use selectors::DecodeSelectors;