#![forbid(unsafe_code)]
#![no_std]

pub mod bits;
pub mod decode;
pub mod error;

pub use crate::decode::{
    decode_rv32i,
    decode_rv32m,
    decode_word,
    BType,
    DecodedInstruction,
    FenceOperands,
    I32Limbs16,
    IType,
    JType,
    MDecomposition16,
    MOperands,
    RType,
    Register,
    Rv32iInstruction,
    Rv32mInstruction,
    SType,
    ShiftImmediate,
    SignedMDecomposition16,
    U32Limbs16,
    UType,
};
pub use crate::error::{Result, ZkvmError};
