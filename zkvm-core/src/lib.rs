pub mod decoder;
pub mod elf_loader;
pub use decoder::{decode, DecodeError, Decoder, Inst as Instruction};
pub use elf_loader::{load_elf, ElfLoadError, LoadSegment, LoadedElf, SegmentFlags};