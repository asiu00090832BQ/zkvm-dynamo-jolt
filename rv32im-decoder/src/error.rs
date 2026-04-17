use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    InvalidInstruction(u32),
    UnsupportedOpcode(u8),
    UnsupportedFunct3 {
        opcode: u8,
        funct3: u8,
        word: u32,
    },
    UnsupportedFunct7 {
        opcode: u8,
        funct3: u8,
        funct7: u8,
        word: u32,
    },
    InvalidShiftEncoding(u32),
    InstructionAddressMisaligned(u32),
    MisalignedAccess {
        addr: u32,
        alignment: u32,
    },
    MemoryOutOfBounds {
        addr: u32,
        size: usize,
    },
    PcOutOfBounds(u32),
    Ecall,
    Ebreak,
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZkvmError::InvalidInstruction(word) => {
                write!(f, "invalid instruction encoding: 0x{word:08x}")
            }
            ZkvmError::UnsupportedOpcode(opcode) => {
                write!(f, "unsupported opcode: 0x{opcode:02x}")
            }
            ZkvmError::UnsupportedFunct3 {
                opcode,
                funct3,
                word,
            } => write!(
                f,
                "unsupported funct3 0b{funct3:03b} for opcode 0x{opcode:02x} in 0x{word:08x}"
            ),
            ZkvmError::UnsupportedFunct7 {
                opcode,
                funct3,
                funct7,
                word,
            } => write!(
                f,
                "unsupported funct7 0b{funct7:07b} with funct3 0b{funct3:03b} for opcode 0x{opcode:02x} in 0x{word:08x}"
            ),
            ZkvmError::InvalidShiftEncoding(word) => {
                write!(f, "invalid RV32 shift-immediate encoding: 0x{word:08x}")
            }
            ZkvmError::InstructionAddressMisaligned(addr) => {
                write!(f, "instruction address misaligned: 0x{addr:08x}")
            }
            ZkvmError::MisalignedAccess { addr, alignment } => {
                write!(f, "misaligned memory access at 0x{addr:08x}, expected alignment {alignment}")
            }
            ZkvmError::MemoryOutOfBounds { addr, size } => {
                write!(f, "memory access out of bounds at 0x{addr:08x} for {size} byte(s)")
            }
            ZkvmError::PcOutOfBounds(pc) => write!(f, "program counter out of bounds: 0x{pc:08x}"),
            ZkvmError::Ecall => write!(f, "environment call trap"),
            ZkvmError::Ebreak => write!(f, "breakpoint trap"),
        }
    }
}

impl std::error::Error for ZkvmError {}
