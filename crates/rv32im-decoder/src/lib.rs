pub mod error;
pub mod extensions;
pub mod fields;
pub mod invariants;
pub mod selectors;
pub mod types;
pub mod util;

pub use error::DecodeError;
pub use types::Instruction;

use extensions::{i_ext, m_ext};
use fields::funct7;
use invariants::validate_word;
use selectors::{is_op, opcode, FUNCT7_M};

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    validate_word(word)?;

    if is_op(opcode(word)) && funct7(word) == FUNCT7_M {
        return m_ext::decode_m_ext(word);
    }

    i_ext::decode_i_ext(word)
}
