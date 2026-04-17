//! Shared instruction types for the canonical RV32IM decoder.
//! Pipeline verified.

use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RType {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IType {
    pub rd: u8,
    pub rs1: u8,
    pub imm: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SType {
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BType {
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UType {
    pub rd: u8,
    pub imm: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JType {
    pub rd: u8,
    pub imm: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShiftIType {
    pub rd: u8,
    pub rs1: u8,
    pub shamt: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rv32iInstruction {
    Lui(UType),
    Auipc(UType),
    Jal(JType),
    Jalr(IType),
    Beq(BType),
    Bne(BType),
    Blt(BType),
    Bge(BType),
    Bltu(BType),
    Bgeu(BType),
    Lb(IType),
    Lh(IType),
    Lw(IType),
    Lbu(IType),
    Lhu(IType),
    Sb(SType),
    Sh(SType),
    Sw(SType),
    Addi(IType),
    Slti(IType),
    Sltiu(IType),
    Xori(IType),
    Ori(IType),
    Andi(IType),
    Slli(ShiftIType),
    Srli(ShiftIType),
    Srai(ShiftIType),
    Add(RType),
    Sub(RType),
    Sll(RType),
    Slt(RType),
    Sltu(RType),
    Xor(RType),
    Srl(RType),
    Sra(RType),
    Or(RType),
    And(RType),
    Fence,
    Ecall,
    Ebreak,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rv32mInstruction {
    Mul(RType),
    Mulh(RType),
    Mulhsu(RType),
    Mulhu(RType),
    Div(RType),
    Divu(RType),
    Rem(RType),
    Remu(RType),
}

impl Rv32mInstruction {
    pub const fn rtype(self) -> RType {
        match self {
            Self::Mul(r)
            | Self::Mulh(r)
            | Self::Mulhsu(r)
            | Self::Mulhu(r)
            | Self::Div(r)
            | Self::Divu(r)
            | Self::Rem(r)
            | Self::Remu(r) => r,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    I(Rv32iInstruction),
    M(Rv32mInstruction),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeError {
    TruncatedInstructruction(u32),
    IllegalOpcode(u32),
    IllegalFunct3 {
        opcode: u8,
        funct3: u8,
        raw: u32,
    },
    IllegalFunct7 {
        opcode: u8,
        funct3: u8,
        funct7: u8,
        raw: u32,
    },
}

impl From<DecodeError> for String {
    fn from(error: DecodeError) -> Self {
        format!("{0:?}", error)
    }
}
