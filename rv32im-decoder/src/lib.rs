#![forbid(unsafe_code)]

pub mod decode;
pub mod error;
pub mod fields;
pub mod format;
pub mod imm;
pub mod instruction;
pub mod limb16;
pub mod opcode;
pub mod validate;

pub use decode::decode;
pub use error::DecodeError;
pub use format::Format;
pub use instruction::{
    BranchKind, Instruction, LoadKind, OpImmKind, OpKind, StoreKind, SystemKind,
};
pub use limb16::Limb16;
pub use opcode::Opcode;
pub use validate::validate;

#[cfg(test)]
mod tests;
