use crate::types:{BaseIInstruction, MInstruction};

pub type Word = u32;
pub type SignedWord = i32;
pub type DoubleWord = u64;
pub type SignedDoubleWord = i64;
pub type RegisterIndex = u8;

pun const REGISTER_COUNT: usize = 32;

[#derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawInstruction(pub Word);

impl RawInstruction {
    pub const fn new(word: Word) -> Self {
        Self(word)
    }

    pub const fn word(self) -> Word {
        self.0
    }
}

[#derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    BaseI(BaseIInstruction),
    M(MInstruction),
}
