use crate::error::{DecodeError, Result};
use crate::fields;
use crate::isa::{basei, mext};
use crate::opcode::MajorOpcode;
use crate::types::DecodedInstruction;

pub fn decode(word: u32) -> Result<DecodedInstruction> {
    if !fields::is_32bit_instruction(word) {
        return Err(DecodeError::InvalidInstructionWord(word));
    }

    let opcode_bits = fields::opcode(word);
    let opcode = MajorOpcode::from_u8(opcode_bits);

    match opcode {
        MajorOpcode::Unknown(bits) => Err(DecodeError::InvalidOpcode(bits)),
        MajorOpcode::Op if fields::funct7(word) == 0b0000001 => mext::decode_m(word),
        known => basei::decode_basei(word, known),
    }
}
