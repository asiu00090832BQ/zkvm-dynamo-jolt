pub mod i;
pub mod m;

use crate::error::ZkvmError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Register(u8);

impl Register {
    pub const ZERO: Self = Self(0);

    pub fn new(index: u8) -> Result<Self, ZkvmError> {
        if index < 32 {
            Ok(Self(index))
        } else {
            Err(ZkvmError::InvalidRegister { index })
        }
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }

    pub const fn raw(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RTypeFields {
    pub rd: Register,
    pub rs1: Register,
    pub rs2: Register,
    pub funct3: u8,
    pub funct7: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ITypeFields {
    pub rd: Register,
    pub rs1: Register,
    pub imm: i32,
    pub funct3: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct STypeFields {
    pub rs1: Register,
    pub rs2: Register,
    pub imm: i32,
    pub funct3: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BTypeFields {
    pub rs1: Register,
    pub rs2: Register,
    pub imm: i32,
    pub funct3: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UTypeFields {
    pub rd: Register,
    pub imm: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JTypeFields {
    pub rd: Register,
    pub imm: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShiftImmFields {
    pub rd: Register,
    pub rs1: Register,
    pub shamt: u8,
    pub funct3: u8,
    pub funct7: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    I(i::Rv32I),
    M(m::Rv32M),
}

impl Instruction {
    pub fn rd(&self) -> Option<Register> {
        match self {
            Self::I(i::Rv32I::Lui(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Auipc(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Jal(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Jalr(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Beq(_)) => None,
            Self::I(i::Rv32I::Bne(_)) => None,
            Self::I(i::Rv32I::Blt(_)) => None,
            Self::I(i::Rv32I::Bge(_)) => None,
            Self::I(i::Rv32I::Bltu(_)) => None,
            Self::I(i::Rv32I::Bgeu(_)) => None,
            Self::I(i::Rv32I::Lb(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Lh(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Lw(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Lbu(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Lhu(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Sb(_)) => None,
            Self::I(i::Rv32I::Sh(_)) => None,
            Self::I(i::Rv32I::Sw(_)) => None,
            Self::I(i::Rv32I::Addi(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Slti(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Sltiu(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Xori(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Ori(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Andi(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Slli(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Srli(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Srai(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Add(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Sub(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Sll(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Slt(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Sltu(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Xor(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Srl(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Sra(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Or(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::And(fields)) => Some(fields.rd),
            Self::I(i::Rv32I::Fence) => None,
            Self::I(i::Rv32I::FenceI) => None,
            Self::I(i::Rv32I::Ecall(_)) => None,
            Self::I(i::Rv32I::Ebreak) => None,
            Self::M(inst) => Some(inst.fields().rd),
        }
    }
}
