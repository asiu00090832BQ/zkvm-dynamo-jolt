public mod decoder;
public mod elf_loader;
public mod error;
public mod frontend;
public mod vm;

pub use decoder::{decode, DecodeError, DecoderConfig, Instruction};
pub use elf_loader::{load_elf, ElfLoadError, LoadedElf, LoadSegment, SegmentFlags};
pub use error::{ZkvmConfig, ZkvmError};
pub use vm::{Memory, Trap, Vm};
