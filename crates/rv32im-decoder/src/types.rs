use core::fmt;

pub type Result<T> = core::result::Result<T, ZkvmError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Mul { rd: u8, rs1: u8, rs2: u8 },
    Mulh { rd: u8, rs1: u8, rs2: u8 },
    Mulhsu { rd: u8, rs1: u8, rs2: u8 },
    Mulhu { rd: u8, rs1: u8, rs2: u8 },
    Div { rd: u8, rs1: u8, rs2: u8 },
    Divu { rd: u8, rs1: u8, rs2: u8 },
    Rem { rd: u8, rs1: u8, rs2: u8 },
    Remu { rd: u8, rs1: u8, rs2: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkvmError {
    InvalidOpcode(u8),
    InvalidFunct { opcode: u8, funct3: u8, funct7: u8 },
    InvalidRegister(u8),
    UnsupportedInstruction,
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOpcode(opcode) => write!(f, "invalid opcode: 0b{opcode:07b}"),
            Self::InvalidFunct {
                opcode,
                funct3,
                funct7,
            } => write!(
                f,
                "invalid funct combination: opcode=0b{opcode:07b}, funct3=0b{funct3:03b}, funct7=0b{funct7:07b}"
            ),
            Self::InvalidRegister(index) => write!(f, "invalid register index: x{index}"),
            Self::UnsupportedInstruction => f.write_str("unsupported instruction"),
        }
    }
}
