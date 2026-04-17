use core::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecoderError {
    InvalidOpcode(u8),
    InvalidFunct3 { opcode: u8, funct3: u8 },
    InvalidFunct7 { opcode: u8, funct7: u8 },
    InvalidRegister { reg: u8 },
    InvalidShiftAmount { shamt: u8 },
    UnsupportedInstruction(u32),
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::InvalidOpcode(opcode) => write!(f, "invalid opcode: {opcode:#09b}"),
            Self::InvalidFunct3 { opcode, funct3 } => {
                write!(f, "invalid funct3 {funct3:#05b} for opcode {opcode:#09b}")
            }
            Self::InvalidFunct7 { opcode, funct7 } => {
                write!(f, "invalid funct7 {funct7:#09b} for opcode {opcode:#09b}")
            }
            Self::InvalidRegister { reg } => write!(f, "invalid register index: {reg}"),
            Self::InvalidShiftAmount { shamt } => write!(f, "invalid shift amount: {shamt}"),
            Self::UnsupportedInstruction(raw) => {
                write!(f, "unsupported instruction encoding: {raw:#010x}")
            }
        }
    }
}
