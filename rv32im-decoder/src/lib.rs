pub mod decoder;
pub mod error;
pub mod fields;
pub mod instruction;

pub use decoder::decode;
pub use decoder::invariants;
pub use error::{Result, ZkvmError};
pub use fields::{BType, IType, JType, RType, RawInstruction, SType, UType};
pub use instruction::{
    ArithmeticOp, BranchOp, Instruction, LoadOp, OpImmOp, StoreOp, SystemOp,
};

#[cfg(test)]
mod tests;
