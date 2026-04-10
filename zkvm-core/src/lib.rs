pub mod decoder;
pub mod elf_loader;
pub mod vm;

pub use decoder::{decode, DecodedInstruction, DecodeError, InstructionKind};
pub use elf_loader::{load_elf, ElfImage, ElfLoaderError, ElfSegment};
pub use vm::{Zkvm, ZkvmConfig, VmError};
