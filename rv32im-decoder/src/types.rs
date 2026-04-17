pub type Word = u32;
pub type Immediate = i32;
pub type RegisterIndex = u8;

pub const REG_COUNT: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InstructionWord(pub Word);

impl InstructionWord {
    #[inline]
    pub const fn new(word: Word) -> Self {
        Self(word)
    }

    #[inline]
    pub const fn bits(self) -> Word {
        self.0
    }
}
