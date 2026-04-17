use crate::formats::{RType, IType, SType, BType, UType, JType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MInstruction {
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

impl MInstruction {
    pub const fn mnemonic(self) -> &'static str {
        match self {
            Self::Mul => "mul",
            Self::Mulh => "mulh",
            Self::Mulhsu => "mulhsu",
            Self::Mulhu => "mulhu",
            Self::Div => "div",
            Self::Divu => "divu",
            Self::Rem => "rem",
            Self::Remu => "remu",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodedInstruction {
    Lui(UType),
    Auipc(UType),
    Jal(JType),
    Jalr(IType),
    Branch(BType),
    Load(IType),
    Store(SType),
    OpImm(IType),
    Op(RType),
    MulDiv(MInstruction, RType),
    System(u32),
}
