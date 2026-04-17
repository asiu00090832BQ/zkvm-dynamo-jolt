use core::fmt;

use crate::error::DecodeError;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Register(u8);

impl Register {
    pub const ZERO: Self = Self(0);

    pub fn new(index: u8) -> Result<Self, DecodeError> {
        if index < 32 {
            Ok(Self(index))
        } else {
            Err(DecodeError::InvalidRegister(index))
        }
    }

    pub const fn from_u5(index: u8) -> Self {
        Self(index & 0x1f)
    }

    pub const fn index(self) -> u8 {
        self.0
    }

    pub const fn abi_name(self) -> &'static str {
        match self.0 {
            0 => "zero",
            1 => "ra",
            2 => "sp",
            3 => "gp",
            4 => "tp",
            5 => "t0",
            6 => "t1",
            7 => "t2",
            8 => "s0",
            9 => "s1",
            10 => "a0",
            11 => "a1",
            12 => "a2",
            13 => "a3",
            14 => "a4",
            15 => "a5",
            16 => "a6",
            17 => "a7",
            18 => "s2",
            19 => "s3",
            20 => "s4",
            21 => "s5",
            22 => "s6",
            23 => "s7",
            24 => "s8",
            25 => "s9",
            26 => "s10",
            27 => "s11",
            28 => "t3",
            29 => "t4",
            30 => "t5",
            31 => "t6",
            _ => "x?",
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{}", self.0)
    }
}

impl From<Register> for u8 {
    fn from(value: Register) -> Self {
        value.index()
    }
}
