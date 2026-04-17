use core::fmt;

pub const OPCODE_OP_IMM: u8 = 0b0010011;
pub const OPCODE_OP: u8 = 0b0110011;
pub const FUNCT7_M: u8 = 0b0000001;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RTypeFields {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub funct7: u8,
}

impl RTypeFields {
    pub fn decode(raw: u32) -> Self {
        Self {
            rd: ((raw >> 7) & 0x1f) as u8,
            funct3: ((raw >> 12) & 0x07) as u8,
            rs1: ((raw >> 15) & 0x1f) as u8,
            rs2: ((raw >> 20) & 0x1f) as u8,
            funct7: ((raw >> 25) & 0x7f) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ITypeFields {
    pub rd: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub imm: i32,
}

impl ITypeFields {
    pub fn decode(raw: u32) -> Self {
        Self {
            rd: ((raw >> 7) & 0x1f) as u8,
            funct3: ((raw >> 12) & 0x07) as u8,
            rs1: ((raw >> 15) & 0x1f) as u8,
            imm: (raw as i32) >> 20,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodedInstruction {
    I(crate::i_extension::IInstruction),
    M(crate::m_extension::MInstruction),
}

pub type Instruction = DecodedInstruction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    UnsupportedOpcode(u32),
    UnsupportedFunct3(u8, u8, u32),
    UnsupportedFunct7(u8, u8, u8, u32),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
