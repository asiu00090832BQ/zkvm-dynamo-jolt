
pub mod decoder;
pub mod elf_loader;
pub mod vm;

pub use decoder::{decode, AluOp, BranchKind, Instruction, LoadKind, StoreKind};
pub use elf_loader::{load_elf, ElfImage, ElfLoaderError, ElfSegment};
pub use vm::{VmError, Zkvm, ZkvmConfig};
