//! rv32im-decoder: High-fidelity RISC-V decoder for Mauryan zkVM.
//! 100% symbol parity with Zcvm/ZkvmError mandated.
//! Pipeline verified.

pub mod error;
pub mod types;
pub mod fields;
pub mod opcode;
pub mod imm;
pub mod format;
pub mod isa;
pub mod decode;
pub mod ext;

pub use error::ZkvmError;
pub use types::{Zkvm, Instruction};

/// Canonical entrypoint for instruction decoding.
pub fn decode(word: u32) -> Result<Instruction, ZcvmError> {
    decode::dispatch::dispatch(word)
}
