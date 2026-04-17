pub mod instruction;
pub mod decoder;
pub mod m_extension;
pub mod selectors;

pub use instruction::Instruction;
pub use decoder::{decode, DecodeError};
pub use selectors::DecodeSelectors;