use core::{convert::TryFrom, fmt};

use crate::error::ZkvmError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Register(u8);

impl Register {
    pub const fn index(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for Register {
    type Error = ZkvmError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 32 {
            Ok(Self(value))
        } else {
            Err(ZkvmError::InvalidRegister(value))
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MulVariant {
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    Lui { rd: Register, imm: i32 },
    Auipc { rd: Register, imm: i32 },
    Jal { rd: Register, imm: i32 },
    Jalr { rd: Register, rs1: Register, imm: i32 },

    Beq { rs1: Register, rs2: Register, imm: i32 },
    Bne { rs1: Register, rs2: Register, imm: i32 },
    Blt { rs1: Register, rs2: Register, imm: i32 },
    Bge { rs1: Register, rs2: Register, imm: i32 },
    Bltu { rs1: Register, rs2: Register, imm: i32 },
    Bgeu { rs1: Register, rs2: Register, imm: i32 },

    Lb { rd: Register, rs1: Register, imm: i32 },
    Lh { rd: Register, rs1: Register, imm: i32 },
    Lw { rd: Register, rs1: Register, imm: i32 },
    Lbu { rd: Register, rs1: Register, imm: i32 },
    Lhu { rd: Register, rs1: Register, imm: i32 },

    Sb { rs1: Register, rs2: Register, imm: i32 },
    Sh { rs1: Register, rs2: Register, imm: i32 },
    Sw { rs1: Register, rs2: Register, imm: i32 },

    Addi { rd: Register, rs1: Register, imm: i32 },
    Slti { rd: Register, rs1: Register, imm: i32 },
    Sltiu { rd: Register, rs1: Register, shamt: u8 },
    Xori { rd: Register, rs1: Register, imm: i32 },
    Ori { rd: Register, rs1: Register, imm: i32 },
    Andi { rd: Register, rs1: Register, imm: i32 },
    Slli { rd: Register, rs1: Register, shamt: u8 },
    Srli { rd: Register, rs1: Register, shamt: u8 },
    Srai { rd: Register, rs1: Register, shamt: u8 },

    Add { rd: Register, rs1: Register, rs2: Register },
    Sub { rd: Register, rs1: Register, rs2: Register },
    Sll { rd: Register, rs1: Register, rs2: Register },
    Slt { rd: Register, rs1: Register, rs2: Register },
    Sltu { rd: Register, rs1: Register, rs2: Register },
    Xor { rd: Register, rs1: Register, rs2: Register },
    Srl { rd: Register, rs1: Register, rs2: Register },
    Sra { rd: Register, rs1: Register, rs2: Register },
    Or { rd: Register, rs1: Register, rs2: Register },
    And { rd: Register, rs1: Register, rs2: Register },

    Fence { pred: u8, succ: u8 },
    Ecall,
    Ebreak,

    Mul { rd: Register, rs1: Register, rs2: Register },
    Mulh { rd: Register, rs1: Register, rs2: Register },
    Mulhsu { rd: Register, rs1: Register, rs2: Register },
    Mulhu { rd: Register, rs1: Register, rs2: Register },
    Div { rd: Register, rs1: Register, rs2: Register },
    Divu { rd: Register, rs1: Register, rs2: Register },
    Rem { rd: Register, rs1: Register, rs2: Register },
    Remu { rd: Register, rs1: Register, rs2: Register },
}

impl Instruction {
    pub const fn mnemonic(&self) -> &'static str {
        match self {
            Self::Lui { .. } => "lui",
            Self::Auipc { .. } => "auipc",
            Self::Jal { .. } => "jal",
            Self::Jalr { .. } => "jalr",
            Self::Beq { .. } => "beq",
            Self::Bne { .. } => "bne",
            Self::Blt { .. } => "blt",
            Self::Bge { .. } => "bge",
            Self::Bltu { .. } => "bltu",
            Self::Bgeu { .. } => "bgeu",
            Self::Lb { .. } => "lb",
            Self::Lh { .. } => "lh",
            Self::Lw { .. } => "lw",
            Self::Lbu { .. } => "lbu",
            Self::Lhu { .. } => "lhu",
            Self::Sb { .. } => "sb",
            Self::Sh { .. } => "sh",
            Self::Sw { .. } => "sw",
            Self::Addi { .. } => "addi",
            Self::Slti { .. } => "slti",
            Self::Sltiu { .. } => "sltiu",
            Self::Xori { .. } => "xori",
            Self::Ori { .. } => "ori",
            Self::Andi { .. } => "andi",
            Self::Slli { .. } => "slli",
            Self::Srli { .. } => "srli",
            Self::Srai { .. } => "srai",
            Self::Add { .. } => "add",
            Self::Sub { .. } => "sub",
            Self::Sll { .. } => "sll",
            Self::Slt { .. } => "slt",
            Self::Sltu { .. } => "sltu",
            Self::Xor { .. } => "xor",
            Self::Srl { .. } => "srl",
            Self::Sra { .. } => "sra",
            Self::Or { .. } => "or",
            Self::And { .. } => "and",
            Self::Fence { .. } => "fence",
            Self::Ecall => "ecall",
            Self::Ebreak => "ebreak",
            Self::Mul { .. } => "mul",
            Self::Mulh { .. } => "mulh",
            Self::Mulhsu { .. } => "mulhsu",
            Self::Mulhu { .. } => "mulhu",
            Self::Div { .. } => "div",
            Self::Divu { .. } => "divu",
            Self::Rem { .. } => "rem",
            Self::Remu { .. } => "remu",
        }
    }
}
