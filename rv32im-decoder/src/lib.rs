#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

pub mod instruction;
pub mod encoding;
pub mod error;
pub mod decoder;

pub use decoder::decode;
pub use error::{DecodeError, DecodeResult, Zkvm, ZkvmError, ZkvmResult};
pub use instruction::Instruction;
