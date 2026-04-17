#![forbid(unsafe_code)]

pub mod decoder;
pub mod error;
pub mod instruction;
pub mod m_extension;

pub use decoder::{decode, Decoder};
pub use error::DecodeError;
pub use instruction::{
    BTypeFields, ITypeFields, Instruction, JTypeFields, RTypeFields, STypeFields,
    ShiftImmFields, UTypeFields,
};
