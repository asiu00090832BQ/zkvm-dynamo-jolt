pub mod error; pub mod instruction; pub mod decoder; pub mod elf_loader;
pub use instruction::{Instruction, Register};
pub use decoder::decode;
pub use elf_loader::{load_elf, LoadedElf, LoadedSegment};
pub use error::{DecodeError, ElfLoadError};