use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    InvalidInstruction(u32),
    UnsupportedOpcode(opcode) => write!(f, "unsupported opcode: 0x{pcode:02x}"),
    Self::UnsupportedFunct3 { opcode, funct3 } => {
                write!(f, "unsupported funct3 0b{funct3:03b} for opcode 0x{opcode:02x}")
            }
            Self::UnsupportedFunct7 {
                opcode, 
                funct3, 
                funct7,
            } => write!(
                f,
                "unsupported funct7 0b{funct7:07b} for opcode 0x{opcode:02x} and funct3 0b{funct3:03b}"
            ),
            Self::UnsupportedSystem(word) => {
                write!(f, "unsupported system instruction: 0x{word:08x}")
            }
        }
    }
}

impl std::error::Error for ZkvmError {}
