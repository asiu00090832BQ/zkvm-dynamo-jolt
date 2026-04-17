use crate::{
    error::ZkvmError,
    i_extension,
    invariants,
    instruction::DecodedInstruction,
    m_extension,
    types::{DecodeResult, OperandDecomposition},
    util,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Zkvm;

impl Zkvm {
    pub const fn new() -> Self {
        Self
    }

    pub fn decode_word(&self, word: u32) -> DecodeResult<DecodedInstruction> {
        match util::opcode(word) {
            0b0110011 if util::funct7(word) == 0b0000001 => m_extension::decode(word),
            0b0110011
            | 0b0010011
            | 0b0000011
            | 0b0100011
            | 0b1100011
            | 0b1101111
            | 0b1100111
            | 0b0110111
            | 0b0010111
            | 0b0001111
            | 0b1110011 => i_extension::decode(word),
            opcode => Err(ZkvmError::UnsupportedOpcode { opcode }),
        }
    }

    pub fn decompose_operands(&self, a: u32, b: u32) -> DecodeResult<OperandDecomposition> {
        invariants::verify_lemma_6_1_1(a, b)
    }
}
