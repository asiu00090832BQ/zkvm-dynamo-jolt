use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ZkvmError {
    InvalidElf,
    InvalidInstruction(u32),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidElf => f.write_str("invalid ELF image"),
            Self::InvalidInstruction(word) => {
                write!(f, "invalid RV32IM instruction: 0x{word:08x}")
            }
        }
    }
}
