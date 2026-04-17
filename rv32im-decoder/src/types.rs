use crate::error::ZkvmError;

pub type DecodeResult<T> = Result<T, ZkvmError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Limb16 {
    pub low: u16,
    pub high: u16,
}

impl Limb16 {
    pub const fn new(low: u16, high: u16) -> Self {
        Self { low, high }
    }

    pub const fn from_u32(value: u32) -> Self {
        Self {
            low: (value & 0xFFFF) as u16,
            high: ((value >> 16) & 0xFFFF) as u16,
        }
    }

    pub const fn recompose(self) -> u32 {
        (self.low as u32) | ((self.high as u32) << 16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OperandDecomposition {
    pub a: Limb16,
    pub b: Limb16,
}

impl OperandDecomposition {
    pub const fn new(a: Limb16, b: Limb16) -> Self {
        Self { a, b }
    }

    pub const fn from_operands(a: u32, b: u32) -> Self {
        Self {
            a: Limb16::from_u32(a),
            b: Limb16::from_u32(b),
        }
    }

    pub const fn a0(self) -> u16 {
        self.a.low
    }

    pub const fn a1(self) -> u16 {
        self.a.high
    }

    pub const fn b0(self) -> u16 {
        self.b.low
    }

    pub const fn b1(self) -> u16 {
        self.b.high
    }
}
