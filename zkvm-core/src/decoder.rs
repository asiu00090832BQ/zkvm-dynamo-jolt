use core::fmt;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction { Fence, Invalid(u32) }
pub struct DecoderConfig { pub enable: bool }
pub struct Decoder { config: DecoderConfig }
impl Decoder { pub fn new(c: DecoderConfig) -> Self { Self { config: c } } pub fn decode(&self, w: u32) -> Instruction { Instruction::Fence } }