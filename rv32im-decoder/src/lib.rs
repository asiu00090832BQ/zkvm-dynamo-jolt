#![forbid(unsafe_code)]

pub mod decode;
pub mod error;
pub mod formats;
pub mod instruction;
pub mod invariants;
pub mod m_extension;

pub use decode::decode;
pub use error::{DecodeError, Result};
pub use formats::{BType, IType, JType, RType, SType, UType};
pub use instruction::{
    BranchKind, Instruction, LoadKind, OpImmKind, OpKind, StoreKind, SystemKind,
};
pub use m_extension::{
    decompose_u32, div, divu, mul, mulh, mulhsu, mulhu, mul_witness, rem, remu, Limbs16,
    MulWitness,
};
