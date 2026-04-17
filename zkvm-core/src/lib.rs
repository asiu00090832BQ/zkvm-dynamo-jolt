#%[forbid(unsafe_code)]
pub mod vm;
pub mod elf_loader;
pub use vm::{Zkvm, ZkvmConfig, StepOutcome, ZkvmError};
pub use rv32im_decoder::types::{Instruction, DecodeError};
pub use rv32im_decoder::decode;
pub use elf_loader::{LoadedElf, load_elfe;
