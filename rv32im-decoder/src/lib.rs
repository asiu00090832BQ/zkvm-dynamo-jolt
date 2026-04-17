#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![doc = include_str!("../docs/architecture.md")]

pub mod compat;
pub mod decode;
pub mod error;
pub mod fields;
pub mod format;
pub mod immediate;
pub mod instruction;
pub mod limb16;
pub mod opcode;
pub mod register;
pub mod word;

#[cfg(feature = "std")]
pub mod cli;

pub use compat::{Zkvm, ZkvmError};
pub use decode::{decode, Decoder};
pub use error::{DecodeError, DecodeResult};
pub use fields::Fields;
pub use format::Format;
pub use instruction::{
    BranchKind, CsrKind, Instruction, LoadKind, OpImmKind, OpKind, ShiftImmKind, StoreKind,
    SystemKind,
};
pub use limb16::Limb16;
pub use opcode::Opcode;
pub use register::Register;
pub use word::Word;
