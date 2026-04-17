// Public zkvm-core symbols.

#![forbid(unsafe_code)]

pub mod vm;
pub mod decoder;

pub use crate::vm::{Zkvm, ZkvmError};
