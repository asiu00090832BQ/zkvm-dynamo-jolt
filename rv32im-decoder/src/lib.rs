pub mod decoder;
pub mod instruction;
pub mod types;
pub mod util;

pub use decoder::decode;
pub use instruction::Instruction;
pub use types::{Csr, DecodeError, DecodeResult, Register, Word};

pub type Zkvm = Instruction;
pub type ZkvmError = DecodeError;
