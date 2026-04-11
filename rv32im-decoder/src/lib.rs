#![no_std]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction { Halt }
pub fn decode(word: u32) -> Result<Instruction, ()> { Ok(Instruction::Halt) }
