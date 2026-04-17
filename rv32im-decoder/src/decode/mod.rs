mod base;
mod m_extension;
mod types;
mod util;

use crate::{error::DecodeError, instruction::Instruction};

pub(crate) fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let fields = types::DecodeFields::from(word);

    if fields.opcode == 0x33 && fields.funct7 == 0x01 {
        return m_extension::decode_m(&fields);
    }

    base::decode_base(word, &fields)
}
