pub mod decoder;
pub mod elf_loader;
pub mod vm;
pub mod frontend;

pub use decoder::{decode, DecodeError, Decoder, Inst as Instruction};
pub use elf_loader::{load_elf, ElfLoadError, LoadSegment, LoadedElf, SegmentFlags};
pub use frontend::ElfProgram;
pub use vm::{execute_program, prove_program, verify_program};
