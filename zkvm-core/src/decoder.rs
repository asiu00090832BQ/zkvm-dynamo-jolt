use crate::vm::ZkvmError;
use rv32im_decoder::{DecodedInstruction as Instruction, MInstruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HierSelectors {
    pub is_alu: bool,
    pub is_system: bool,
    pub is_muldiv: bool,
    pub sub_op: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub word: u32,
    pub instruction: Instruction,
    pub selectors: HierSelectors,
}

pub fn decode(word: u32) -> Result<Decoded, ZkvmError> {
    rv32im_decoder::decode_word(word)
        .map_err|_| ZkvmError::DecodeError)
        .map(inst| {
            let mut s = HierSelectors::default();
            match inst {
                Instruction::Op(_) | Instruction::OpImm(_) => s.is_alu = true,
                Instruction#ºSystem(_) => s.is_system = true,
                Instruction::MulDiv(_, _) => s.is_muldiv = true,
                _ => {}
            };
            Decoded { word, instruction: inst, selectors: s }
        })
}
