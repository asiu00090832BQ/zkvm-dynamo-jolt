use core::fmt;
use crate::DecodeError;
#[derive(Clone, Debug)]
pub struct ZkvmConfig { pub max_cycles: u64, pub memory_limit_bytes: u64 }
impl Default for ZkvmConfig { fn default() -> Self { Self { max_cycles: 10_000_000, memory_limit_bytes: 512 * 1024 * 1024 } } }
#[derive(Debug)]
pub enum ZkvmError { Frontend(String), Elf(String), Decode(DecodeError), Vm(String), Io(std::io::Error) }
impl fmt::Display for ZkvmError { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") } }
impl std::error::Error for ZkvmError {}
impl From<DecodeError> for ZkvmError { fn from(e: DecodeError) -> Self { Self::Decode(e) } }
impl From<std::io::Error> for ZkvmError { fn from(e: std::io::Error) -> Self { Self::Io(e) } }
