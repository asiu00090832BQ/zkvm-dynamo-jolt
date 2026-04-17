use core::fmt;

use crate::{
    bits,
    error::{Result, ZkvmError},
};

pub mod rv32i;
pub mod rv32m;

pub use self::rv32i::{
    decode_rv32i,
    BType,
    FenceOperands,
    IType,
    JType,
    RType,
    Rv32iInstruction,
    SType,
    ShiftImmediate,
    UType,
};
pub use self::rv32m::{
    decode_rv32m,
    I32Limbs16,
    MDecomposition16,
    MOperands,
    Rv32mInstruction,
    SignedMDecomposition16,
    U32Limbs16,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Register(u8);

impl Register {
    pub const ZERO: Self = Self(0);

    #[inline]
    pub fn new(index: u8) -> Result<Self> {
        if index < 32 {
            Ok(Self(index))
        } else {
            Err(ZkvmError::InvalidRegister { reg: index })
        }
    }

    #[inline]
    pub const fn index(self) -> u8 {
        self.0
    }
}

impl core::convert::TryFrom<u8> for Register {
    type Error = ZkvmError;

    #[inline]
    fn try_from(value: u8) -> Result<Self> {
        Self::new(value)
    }
}

impl From<Register> for u8 {
    #[inline]
    fn from(register: Register) -> Self {
        register.0
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodedInstruction {
    Rv32i(Rv32iInstruction),
    Rv32m(Rv32mInstruction),
}

#[inline]
pub fn decode_word(word: u32) -> Result<DecodedInstruction> {
    if !bits::is_32bit(word) {
        return Err(ZkvmError::UnsupportedCompressedInstruction {
            halfword: word as u16,
        });
    }

    match bits::opcode(word) {
        0b0110011 if bits::funct7(word) == 0b0000001 => {
            decode_rv32m(word).map(DecodedInstruction::Rv32m)
        }
        0b0110111
        | 0b0010111
        | 0b1101111
        | 0b1100111
        | 0b1100011
        | 0b0000011
        | 0b0100011
        | 0b0010011
        | 0b0110011
        | 0b0001111
        | 0b1110011 => decode_rv32i(word).map(DecodedInstruction::Rv32i),
        opcode => Err(ZkvmError::UnknownOpcode { opcode, word }),
    }
}
