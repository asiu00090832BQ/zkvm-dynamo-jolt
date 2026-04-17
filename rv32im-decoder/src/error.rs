use core::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ZkvmError {
    InvalidInstruction(u32),
    UnsupportedInstruction(u32),
    UnsupportedExecution(&'static str),
    RegisterOutOfBounds(usize),
    PcOutOfBounds(u32),
    Utf8IntegrityViolation(&'static str),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZkvmError::InvalidInstruction(word) => write!(f, "invalid instruction word: 0x{word:08x}"),
            ZkvmError::UnsupportedInstruction(word) => write!(f, "unsupported instruction word: 0x{word:08x}"),
            ZkvmError::UnsupportedExecution(message) => write!(f, "{message}"),
            ZkvmError::RegisterOutOfBounds(index) => write!(f, "register index out of bounds: {index}"),
            ZkvmError::PcOutOfBounds(pc) => write!(f, "program counter out of bounds: 0x{pc:08x}"),
            ZkvmError::Utf8IntegrityViolation(path) => write!(f, "utf-8 integrity violation in: {path}"),
        }
    }
}

impl std::error::Error for ZkvmError {}
