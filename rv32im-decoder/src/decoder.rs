use crate::error::DecoderError;
use crate::i_extension::{
    decode_branch, decode_load, decode_misc_mem, decode_op, decode_op_imm, decode_store,
    decode_system,
};
use crate::instruction::Instruction;
use crate::m_extension::decode_m;
use crate::selectors::{
    FUNCT7_M, OPCODE_AUIPC, OPCODE_BRANCH, OPCODE_JAL, OPCODE_JALR, OPCODE_LOAD,
    OPCODE_LUI, OPCODE_MISC_MEM, OPCODE_OP, OPCODE_OPIMM, OPCODE_STORE, OPCODE_SYSTEM,
};
use crate::types::Word;
use crate::util::{funct3, funct7, imm_i, imm_j, imm_u, opcode, rd, rs1};

#[derive(Clone, Copy, Debug, Default)]
pub struct Decoder;

impl Decoder {
    #[inline]
    pub const fn new() -> Self {
        Self
    }

    #[inline]
    pub fn decode(&self, word: Word) -> Result<Instruction, DecoderError> {
        decode(word)
    }
}

pub fn decode(word: Word) -> Result<Instruction, DecoderError> {
    match opcode(word) {
        OPCODE_LUI => Ok(Instruction::Lui {
            rd: rd(word),
            imm: imm_u(word),
        }),
        OPCODE_AUIPC => Ok(Instruction::Auipc {
            rd: rd(word),
            imm: imm_u(word),
        }),
        OPCODE_JAL => Ok(Instruction::Jal {
            rd: rd(word),
            imm: imm_j(word),
        }),
        OPCODE_JALR => {
            if funct3(word) != 0 {
                return Err(DecoderError::InvalidFunct3 {
                    opcode: OPCODE_JALR,
                    funct3: funct3(word),
                });
            }
            Ok(Instruction::Jalr {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            })
        }
        OPCODE_BRANCH => decode_branch(word),
        OPCODE_LOAD => decode_load(word),
        OPCODE_STORE => decode_store(word),
        OPCODE_OPIMM => decode_op_imm(word),
        OPCODE_OP => {
            if funct7(word) == FUNCT7_M {
                decode_m(word)
            } else {
                decode_op(word)
            }
        }
        OPCODE_MISC_MEM => decode_misc_mem(word),
        OPCODE_SYSTEM => decode_system(word),
        opcode_value => Err(DecoderError::UnsupportedOpcode(opcode_value)),
    }
}
