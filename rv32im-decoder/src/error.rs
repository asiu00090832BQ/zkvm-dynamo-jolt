use thiserror::Error;

pub type Result<T> = core::result::Result<T, ZkvmError>;

#[derive(Debug, Error)]
pub enum ZkvmError {
    #[error("invalid opcode: 0x{0:08x}")]
    InvalidOpcode(u32),

    #[error("invalid funct3: opcode=0x{opcode:02x}, funct3=0x{funct3:01x}")]
    InvalidFunct3 { opcode: u8, funct3: u8 },

    #[error("invalid funct7: opcode=0x{opcode:02x}, funct3=0x{funct3:01x}, funct7=0x{funct7:02x}")]
    InvalidFunct7 { opcode: u8, funct3: u8, funct7: u8 },

    #[error("reserved or unsupported instruction: 0x{0:08x}")]
    UnsupportedInstruction(u32),

    #[error("limb decomposition overflow for value {0}")]
    LimbOverflow(u64),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
