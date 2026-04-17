pub use rv32im_decoder::{decode, Csr, DecodeError, DecodeResult, Instruction, Register, Word};

pub type Zkvm = Instruction;
pub type ZkvmError = DecodeError;
