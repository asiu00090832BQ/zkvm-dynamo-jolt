#![forbid(unsafe_code)]

pub mod decoder;
pub mod vm;

pub use decoder::{decode, Decoder, Decoded};
pub use rv32im_decoder::{
    BTypeFields, DecodeError, ITypeFields, Instruction, JTypeFields, RTypeFields, STypeFields,
    ShiftImmFields, UTypeFields,
};
pub use vm::{Vm, VmError};
