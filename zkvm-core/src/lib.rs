pub mod error;
pub mod frontend;
pub mod decoder;
pub mod elf_loader;
pub mod vm;

pub use error::{ZkvmConfig, ZkvmError};
pub use decoder::{DecodeError, Instruction, decode};
pub use frontend::ElfProgram;
pub use elf_loader::{load_elf, LoadSegment, SegmentFlags, LoadedElf};
pub use vm::{Vm, Memory, Trap, execute_program, prove_program, verify_program};
