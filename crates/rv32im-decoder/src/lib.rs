#![no_std]

pub mod decode;
pub mod error;
pub mod isa;

pub use decode::{decode_word, execute_rv32m, split16};
pub use error::{ZkvmError, ZkvmResult};
pub use isa::i::{Ecall, Rv32I, Sub};
pub use isa::m::Rv32M;
pub use isa::{
    BTypeFields, ITypeFields, Instruction, JTypeFields, RTypeFields, Register, STypeFields,
    ShiftImmFields, UTypeFields,
};
