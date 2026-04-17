use crate::formats::{BType, IType, JType, RType, SType, UType};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
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
