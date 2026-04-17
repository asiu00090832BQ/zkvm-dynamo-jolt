#![no_std]
#![forbid(unsafe_code)]

pub mod instruction;
pub mod mul;

pub use instruction::{decode, DecodeError, Instruction, RegisterIndex};
