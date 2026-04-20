#![forbid(unsafe_code)]

pub const XLEN: usize = 32;
pub const REGISTER_COUNT: usize = 32;

pub mod decoder;
pub mod elf_loader;
pub mod proof;
pub mod vm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZkvmError {
    DecodeError(u32),
    InvalidElf,
    MemoryOutOfBounds { addr: u32, len: usize },
    MisalignedAccess { addr: u32, alignment: usize },
    RegisterOutOfBounds { index: usize },
    StepLimitExceeded { limit: usize },
    Halted,
    Trap,
}

pub type ZkvmResult<T> = core::result::Result<T, ZkvmError>;

pub use decoder::{
    decode, BranchKind, Instruction, LoadKind, MulDivOp, OpImmKind, OpKind, StoreKind, SystemKind,
    INSTRUCTION_SIZE,
};
pub use elf_loader::{load_elf, load_elf_into_vm, ProgramImage};
pub use proof::{
    lemma_6_1_1_witness, rv32m_mul_artifact, rv32m_mul_result, Lemma611Witness, Limb16,
    MulProofArtifact, ProofTrace,
};
pub use vm::{StepOutcome, Vm};
