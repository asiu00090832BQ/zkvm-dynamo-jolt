pub mod decode;
pub mod error;
pub mod types;

pub mod decoder {
    pub mod base_i;
    pub mod fields;
    pub mod invariants;
    pub mod m_ext;
}

#[path = "../../zkvm-core/src/vm.rs"]
pub mod vm;

pub use decode::{decode_hex, decode_word};
pub use error::ZkvmError;
pub use types::{
    BranchKind, Instruction, LoadKind, Op, OpImm, Register, StoreKind,
};

#[cfg(test)]
mod tests;
