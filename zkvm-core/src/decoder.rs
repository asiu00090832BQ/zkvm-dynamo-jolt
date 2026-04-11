#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Ecall,
    Invalid(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Invalid(u32),
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    if word == 0x73 {
        Ok(Instruction::Ecall)
    } else {
        Ok(Instruction::Invalid(word))
    }
}
