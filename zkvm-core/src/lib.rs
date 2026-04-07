pub mod decoder;
pub mod elf_loader;
pub mod error;
pub mod frontend;
pub mod vm;

pub use decoder::{decode, DecodeError, DecoderConfig, Instruction};
pub use elf_loader::{load_elf, ElfLoadError, LoadedElf, LoadSegment, SegmentFlags};
pub use error::{ZkvmConfig, ZkvmError};
pub use vm::{Memory, Trap, Vm};
