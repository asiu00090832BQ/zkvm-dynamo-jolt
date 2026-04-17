use core::fmt;

use crate::error::ZkvmError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Register(u8);

impl Register {
    pub fn new(index: u8) -> Result<Self, ZkvmError> {
        if index < 32 {
            Ok(Self(index))
        } else {
            Err(ZkvmError::InvalidRegister {
                reg: index,
                context: "register index",
            })
        }
    }

    pub const fn raw(self) -> u8 {
        self.0
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpImm {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchKind {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadKind {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreKind {
    Sb,
    Sh,
    Sw,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    Lui {
        rd: Register,
        imm: i32,
    },
    Auipc {
        rd: Register,
        imm: i32,
    },
    Jal {
        rd: Register,
        imm: i32,
    },
    Jalr {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Branch {
        kind: BranchKind,
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    Load {
        kind: LoadKind,
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Store {
        kind: StoreKind,
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    OpImm {
        kind: OpImm,
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Op {
        kind: Op,
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Fence,
    Ecall,
    Ebreak,
}
