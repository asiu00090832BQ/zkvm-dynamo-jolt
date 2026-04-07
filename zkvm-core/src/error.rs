//! Error types and configuration for the zkVM.
use crate::decoder::DecodeError;
use core::fmt;
use std::error::Error as StdError;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZkvmConfig { pub max_memory: u64, pub entry_pc: u64, pub max_cycles: u64, }
impl Default for ZkvmConfig { fn default() -> Self { Self { max_memory: 16 * 1024 * 1024, entry_pc: 0, max_cycles: 1_000_000, } } }
impl ZkvmConfig { pub fn validate(&self) -> Result<(), ZkvmError> { if self.max_memory == 0 { return Err(ZkvmError::InvalidConfig("max_memory must be > 0".to_string())); } if self.max_cycles == 0 { return Err(ZkvmError::InvalidConfig("max_cycles must be > 0".to_string())); } Ok(()) } }
#[derive(Debug)]
pub enum ZkvmError { Io(std::io::Error), Decode(DecodeError), ElfLoad(String), InvalidElf(String), NoProgramLoaded, ExecutionLimitExceeded { limit: u64 }, MemoryError(String), Trap(String), InvalidConfig(String), }
impl fmt::Display for ZkvmError { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { match self { ZkvmError::Io(err) => write!(f, "I/O error: {err}"), ZkvmError::Decode(err) => write!(f, "decode error: {err}"), ZkvmError::ElfLoad(msg) => write!(f, "ELF load error: {msg}"), ZkvmError::InvalidElf(msg) => write!(f, "invalid ELF image: {msg}"), ZkvmError::NoProgramLoaded => write!(f, "no program loaded"), ZkvmError::ExecutionLimitExceeded { limit } => write!(f, "execution limit exceeded: {limit} cycles"), ZkvmError::MemoryError(msg) => write!(f, "memory error: {msg}"), ZkvmError::Trap(msg) => write!(f, "trap: {msg}"), ZkvmError::InvalidConfig(msg) => write!(f, "invalid configuration: {msg}"), } } }
impl StdError for ZkvmError { fn source(&self) -> Option<&(dyn StdError + 'static)> { match self { ZkvmError::Io(err) => Some(err), ZkvmError::Decode(err) => Some(err), _ => None, } } }
impl From<std::io::Error> for ZkvmError { fn from(err: std::io::Error) -> Self { ZkvmError::Io(err) } }
impl From<DecodeError> for ZkvmError { fn from(err: DecodeError) -> Self { ZkvmError::Decode(err) } }