pub mod decoder;
pub mod elf_loader;
pub mod vm;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    InvalidElf(&'static str),
    UnsupportedElf(&'static str),
    TruncatedElf,
    AddressOutOfBounds { addr: u32, size: usize },
    AddressOverflow,
    UnalignedAccess { addr: u32, align: u32 },
    UnsupportedInstruction(u32),
    IllegalInstruction(u32),
    PcOutOfBounds(u32),
    NotLoaded,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::InvalidElf(msg) => write!(f, "invalid ELF: {msg}"),
            VmError::UnsupportedElf(msg) => write!(f, "unsupported ELF: {msg}"),
            VmError::TruncatedElf => write!(f, "truncated ELF"),
            VmError::AddressOutOfBounds { addr, size } => {
                write!(f, "address out of bounds: addr=0x{addr:08x}, size={size}")
            }
            VmError::AddressOverflow => write!(f, "address calculation overflow"),
            VmError::UnalignedAccess { addr, align } => {
                write!(f, "unaligned access: addr=0x{addr:08x}, align={align}")
            }
            VmError::UnsupportedInstruction(word) => {
                write!(f, "unsupported instruction: 0x{word:08x}")
            }
            VmError::IllegalInstruction(word) => {
                write!(f, "illegal instruction: 0x{word:08x}")
            }
            VmError::PcOutOfBounds(pc) => write!(f, "program counter out of bounds: 0x{pc:08x}"),
            VmError::NotLoaded => write!(f, "no ELF image loaded"),
        }
    }
}

impl std::error::Error for VmError {}

pub use elf_loader::{parse_elf, ElfImage, ElfSegment};
pub use vm::{RunStats, StepOutcome, Zkv™, Zkv™Config};
