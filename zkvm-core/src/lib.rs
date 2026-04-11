pub mod vm;
pub mod decoder;

pub use vm::{StepOutcome, Zkvm, ZkvmConfig, VmError};
pub use decoder::{decode_instruction, DecodeError, Instruction};
pub use rv32im_decoder as rv32im;
