use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeError {
    Non32BitInstruction { low_bits: u8 },
    UnsupportedOpcode(u8),
    UnsupportedFunct3 { opcode: u8, funct3: u8 },
    UnsupportedFunct7 { opcode: u8, funct3: u8, funct7: u8 },
    InvalidRegister(u8),
    InvalidEncoding(&'static str),
}

pub type DecodeResult<T> = core::result::Result<T, DecodeError>;
pub type ZkvmError = DecodeError;

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Non32BitInstruction { low_bits } => {
                write!(f, "expected a 32-bit instruction, found low bits {low_bits:#04b}")
            }
            Self::UnsupportedOpcode(opcode) => {
                write!(f, "unsupported opcode {opcode:#09b}")
            }
            Self::UnsupportedFunct3 { opcode, funct3 } => {
                write!(
                    f,
                    "unsupported funct3 {funct3:#05b} for opcode {opcode:#09b}"
                )
            }
            Self::UnsupportedFunct7 {
                opcode,
                funct3,
                funct7,
            } => {
                write!(
                    f,
                    "unsupported funct7 {funct7:#09b} for opcode {opcode:#09b} and funct3 {funct3:#05b}"
                )
            }
            Self::InvalidRegister(index) => write!(f, "invalid register index {index}"),
            Self::InvalidEncoding(reason) => write!(f, "invalid encoding: {reason}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}
