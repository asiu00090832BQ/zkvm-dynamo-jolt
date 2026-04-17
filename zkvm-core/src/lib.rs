pub mod vm;

pub use rv32im_decoder::{decode, Instruction, Register, ZkvmError};
pub use vm::Vm;
