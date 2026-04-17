use rv32im_decoder::{decode, Instruction, ZkvmError};

pub type VmResult<T> = Result<T, ZkvmError>;
pub type DecodedInstruction = Instruction;
pub type ZkvmVm = Vm;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vm;

impl Vm {
    pub fn new() -> Self {
        Self
    }

    pub fn decode(&self, word: u32) -> VmResult<DecodedInstruction> {
        decode(word)
    }

    pub fn decode_instruction(&self, word: u32) -> VmResult<DecodedInstruction> {
        self.decode(word)
    }

    pub fn decode_program(&self, words: &[u32]) -> VmResult<Vec<DecodedInstruction>> {
        words.iter().copied().map(|word| self.decode(word)).collect()
    }
}
