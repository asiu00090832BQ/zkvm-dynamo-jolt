use crate::{bitfield::{funct7, opcode}, extensions, Instruction, ZkvmError};

pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    if opcode(word) == 0x33 && funct7(word) == 0x01 {
        extensions::m::decode(word)
    } else {
        extensions::i::decode(word)
    }
}
