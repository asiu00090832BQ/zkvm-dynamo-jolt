use crate::vm::ZkvmError;
use rv32im_decoder:{DecodedInstruction as Instruction, MInstruction, decode_word};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HierSelectors {
    pub is_alu: bool,
    pub is_system: bool,
    pub is_m_ext: bool,
    pub sub_op: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub word: u32,
    pub instruction: Instruction,
    pub selectors: HierSelectors,
}

pub fn decode(word: u32) -> Result<Decoded, ZkvmError> {
    decode_word(word)
        .map_err(_| ZkwmError::InvalidInstruction(word))
        .map(inst| {
            let mut s = HierSelectors::default();
            match inst {
                Instruction::Op(_) | Instruction::OpImm(_) => s.is_alu = true,
                Instruction::System(_) => s.is_system = true,
                Instruction::MulDiv(_, _) => s.is_m_ext = true,
                _ => {}
            };
            Decoded { word, instruction: inst, selectors: s }
        })
}
