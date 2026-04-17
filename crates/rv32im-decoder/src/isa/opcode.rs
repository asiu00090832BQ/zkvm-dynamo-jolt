use core::convert::TryFrom;

use crate::error::ZkvmError;

pub const RV32M_FUNCT7: u8 = 0b0000001;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Op = 0b0110011,
}

impl TryFrom<u8> for Opcode {
    type Error = ZkvmError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::Op as u8 => Ok(Self::Op),
            opcode => Err(ZkvmError::InvalidOpcode { opcode }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MFunct3 {
    Mul = 0b000,
    Mulh = 0b001,
    Mulhsu = 0b010,
    Mulhu = 0b011,
    Div = 0b100,
    Divu = 0b101,
    Rem = 0b110,
    Remu = 0b111,
}

impl TryFrom<u8> for MFunct3 {
    type Error = ZkvmError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::Mul as u8 => Ok(Self::Mul),
            x if x == Self::Mulh as u8 => Ok(Self::Mulh),
            x if x == Self::Mulhsu as u8 => Ok(Self::Mulhsu),
            x if x == Self::Mulhu as u8 => Ok(Self::Mulhu),
            x if x == Self::Div as u8 => Ok(Self::Div),
            x if x == Self::Divu as u8 => Ok(Self::Divu),
            x if x == Self::Rem as u8 => Ok(Self::Rem),
            x if x == Self::Remu as u8 => Ok(Self::Remu),
            funct3 => Err(ZkvmError::InvalidFunct3 { funct3 }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegisterTriple {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
}

impl RegisterTriple {
    pub const fn new(rd: u8, rs1: u8, rs2: u8) -> Self {
        Self { rd, rs1, rs2 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MInstructionKind {
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
pub struct MInstruction {
    pub kind: MInstructionKind,
    pub regs: RegisterTriple,
}

impl MInstruction {
    pub const fn new(kind: MInstructionKind, rd: u8, rs1: u8, rs2: u8) -> Self {
        Self {
            kind,
            regs: RegisterTriple::new(rd, rs1, rs2),
        }
    }

    pub const fn rd(&self) -> u8 {
        self.regs.rd
    }

    pub const fn rs1(&self) -> u8 {
        self.regs.rs1
    }

    pub const fn rs2(&self) -> u8 {
        self.regs.rs2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodedInstruction {
    M(MInstruction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstructionWord(pub u32);

impl InstructionWord {
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub const fn opcode(self) -> u8 {
        (self.0 & 0x7f) as u8
    }

    pub const fn rd(self) -> u8 {
        ((self.0 >> 7) & 0x1f) as u8
    }

    pub const fn funct3(self) -> u8 {
        ((self.0 >> 12) & 0x07) as u8
    }

    pub const fn rs1(self) -> u8 {
        ((self.0 >> 15) & 0x1f) as u8
    }

    pub const fn rs2(self) -> u8 {
        ((self.0 >> 20) & 0x1f) as u8
    }

    pub const fn funct7(self) -> u8 {
        ((self.0 >> 25) & 0x7f) as u8
    }
}
