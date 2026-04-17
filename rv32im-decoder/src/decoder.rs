use crate::i_extension::{
    decode_auipc, decode_branch, decode_fence, decode_jal, decode_jalr, decode_load, decode_lui,
    decode_op, decode_op_imm, decode_store, decode_system,
};
use crate::m_extension::decode_m_extension;
use crate::types::{DecodeError, Instruction};
use crate::util::{funct7, opcode};

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    match opcode(word) {
        0b0110111 => decode_lui(word),
        0b0010111 => decode_auipc(word),
        0b1101111 => decode_jal(word),
        0b1100111 => decode_jalr(word),
        0b1100011 => decode_branch(word),
        0b0000011 => decode_load(word),
        0b0100011 => decode_store(word),
        0b0010011 => decode_op_imm(word),
        0b0110011 => match funct7(word) {
            0b0000000 | 0b0100000 => decode_op(word),
            0b0000001 => decode_m_extension(word),
            funct7 => Err(DecodeError::UnsupportedFunct7 {
                word,
                funct3: crate::util::funct3(word),
                funct7,
            }),
        },
        0b0001111 => decode_fence(word),
        0b1110011 => decode_system(word),
        opcode => Err(DecodeError::UnsupportedOpcode { word, opcode }),
    }
}
