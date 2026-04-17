use crate::error::ZkvmResult;
use crate::instruction::Instruction;
use crate::fields;
use crate::extensions;

pub fn decode(word: u32) -> ZkvmResult<Instruction> {
    let op = fields::opcode(word);
    match op {
        0x33 => {
            let f7 = fields::funct7(word);
            if f7 == 0x01 {
                extensions::m::decode(word).ok_or(crate::error::ZkvmError::InvalidFunct3(fields::funct3(word)))
            } else {
                extensions::base_i::decode(word)
            }
        }
        _ => extensions::base_i::decode(word),
    }
}
