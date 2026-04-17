#![forbid(unsafe_code)]
#![no_std]

pub use rv32im_decoder::bits;
pub use rv32im_decoder::decode;
pub use rv32im_decoder::error;
pub use rv32im_decoder::{
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
    Result,
    Rv32iInstruction,
    Rv32mInstruction,
    SType,
    ShiftImmediate,
    SignedMDecomposition16,
    U32Limbs16,
    UType,
    ZkvmError,
};

pub type ZkvmResult<T> = Result<T>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Zkvm;

impl Zkvm {
    #[inline]
    pub fn decode(word: u32) -> ZkvmResult<DecodedInstruction> {
        decode_word(word)
    }
}
