pub mod error;
pub mod format;
pub mod instruction;
pub mod register;

mod decode;

pub use error::DecodeError;
pub use format::InstructionFormat;
pub use instruction::{Instruction, Mnemonic};
pub use register::Register;

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    decode::decode(word)
}
