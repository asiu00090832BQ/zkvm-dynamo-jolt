#![forbid(unsafe_code)]

pub mod decoder;
pub mod encoding;
pub mod error;
pub mod instruction;

pub use decoder::decode;
pub use error::ZkvmError;
pub use instruction::{
    BranchKind, DecodedInstruction, Instruction, LoadKind, MKind, OpImmKind, OpKind, Register,
    StoreKind,
};
