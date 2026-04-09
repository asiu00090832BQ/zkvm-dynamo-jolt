#![forbid(unsafe_code)]
pub mod decoder;
pub mod elf_loader;
pub mod vm;
use core::fmt;
pub use decoder::{decode, AluImmKind, AluKind, BranchKind, DecodeError, DecoderConfig, Instruction, LoadKind, MulDivKind, ShiftKind, StoreKind};
pub use elf_loader::{load_elf, ElfImage, ElfLoaderError, ElfSegment};
pub use vm::{VmConfig, VmError, Zkvm};
#[derive(Debug)]
pub enum Error { Decode(DecodeError), ElfLoader(ElfLoaderError), Vm(VmError) }
impl fmt::Display for Error { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { match self { Self::Decode(error) => write!(f, "decode error: {error}"), Self::ElfLoader(error) => write!(f, "elf loader error: {error}"), Self::Vm(error) => write!(f, "vm error: {error}") } } }
impl std::error::Error for Error { fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { match self { Self::Decode(error) => Some(error), Self::ElfLoader(error) => Some(error), Self::Vm(error) => Some(error) } } }
impl From<DecodeError> for Error { fn from(error: DecodeError) -> Self { Self::Decode(error) } }
impl From<ElfLoaderError> for Error { fn from(error: ElfLoaderError) -> Self { Self::ElfLoader(error) } }
impl From<VmError> for Error { fn from(error: VmError) -> Self { Self::Vm(error) } }
pub type Result<T> = std::result::Result<T, Error>;