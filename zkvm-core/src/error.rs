use std::error::Error as StdError;
use std::fmt;

use crate::decoder::DecodeError;

#[derive(Debug, Clone)]
pub struct ZkvmConfig {
    pub max_memory: u64,
    pub entry_pc: Option<u64>,
    pub max_cycles: u64,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            max_memory: 16 * 1024 * 1024,
            entry_pc: None,
            max_cycles: 10000,
        }
    }
}

#[derive(Debug)]
pub enum ZkvmError {
    Io(std::io::Error),
    Decode(DecodeError),
    NoProgramLoaded,
    ExecutionLimitExceeded { limit: u64 },
    InvalidConfiguration(String),
    InvalidElf(String),
    ElfLoad(String),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZkvmError::Io(e) => write!(f, "IO error: {}", e),
            ZkvmError::Decode(e) => write!(f, "Decode error: {:?}", e),
            ZkvmError::NoProgramLoaded => write!(f, "No program loaded"),
            ZkvmError::ExecutionLimitExceeded { limit } => {
                write!(f, "Execution limit exceeded: {}", limit)
            }
            ZkvmError::InvalidConfiguration(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
            ZkvmError::InvalidElf(msg) => write!(f, "Invalid ELF: {}", msg),
            ZkvmError::ElfLoad(msg) => write!(f, "ELF load error: {}", msg),
        }
    }
}

impl StdError for ZkvmError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ZkvmError::Io(e) => Some(e),
            ZkvmError::Decode(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ZkvmError {
    fn from(e: std::io::Error) -> Self {
        ZkvmError::Io(e)
    }
}

impl From<DecodeError> for ZkvmError {
    fn from(e: DecodeError) -> Self {
        ZkvmError::Decode(e)
    }
}
