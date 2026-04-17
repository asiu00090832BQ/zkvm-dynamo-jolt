//! Thin shim over the canonical rv32im-decoder crate.
//! No local decode logic is permitted here.
//! Pipeline verified.

pub use rv32im_decoder::{
    decode, decode_rv32i, decode_rv32m, decode_word, decompose_operands16, execute_rv32m,
    is_rv32m, BType, DecodeError, IType, Instruction, JType, Operands16, RType,
    Rv32iInstruction, Rv32mInstruction, SType, ShiftIType, UType,
};
