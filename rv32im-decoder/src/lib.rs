pub mod decode;
pub mod error;
pub mod isa;
pub use decode::decode_word;
pub use error::DecodeError;
pub use isa::Instruction;
