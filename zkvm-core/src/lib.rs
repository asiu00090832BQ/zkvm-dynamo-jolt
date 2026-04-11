pub mod vm;

pub use rv32im_decoder::{
    decode_instruction, BranchOp, CsrOp, DecodeError, Instruction, LoadOp, OpImmOp, OpOp, StoreOp,
};
pub use vm::{RunStats, StepOutcome, VmError, Zkvm, ZkvmConfig};
