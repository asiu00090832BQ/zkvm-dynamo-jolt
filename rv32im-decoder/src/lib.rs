#![no_std]
#![forbid(unsafe_code)]
pub mod error;
pub mod fields;
pub mod instruction;
pub mod decode;
pub mod extensions;

pub use error::{ZkvmError, ZkvmResult};
pub use instruction::Instruction;
pub use decode::decode;
