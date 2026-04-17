pub mod vm;

pub use rv32im_decoder::{decode as decode_word, decode_bytes as decode_le_bytes};
pub use rv32im_decoder::{BranchKind, DecodeError, Instruction, LoadKind, OpImmKind, OpKind, StoreKind, SystemKind};
pub use vm::{Zkvm as Vm, ZkvmError as VmError};
