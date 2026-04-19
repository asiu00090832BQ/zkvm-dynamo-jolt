use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZkvmError { DecodeError, InvalidElf, MemoryOutOfBounds, InvalidInstruction(u32), StepLimitReached, Trap }
impl fmt::Display for ZkvmError { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) } }
impl std::error::Error for ZkvmError {}
pub mod decoder; pub mod elf_loader; pub mod proof; pub mod vm;
pub use decoder::{decode, Instruction}; pub use elf_loader::LoadedElf; pub use proof::{prove_lemma_6_1_1, Lemma611Proof, ProofPipeline}; pub use vm::{StepOutcome, Zkvm, ZkvmConfig};