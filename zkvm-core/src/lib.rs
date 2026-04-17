pub mod vm;
pub mod decoder;
pub use vm::{Zkvm, ZkvmError, ZkvmConfig};
pub use decoder::Decoder;
pub use rv32im_decoder::{DecodeError, Instruction};
