pub mod types;
pub mod decoder;

pub use types:{Instruction, DecodeError};
pub use decoder::decode;
