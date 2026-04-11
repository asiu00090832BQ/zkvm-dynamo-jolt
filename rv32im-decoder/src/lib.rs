pub enum Instruction { Add { rd: u8, rs1: u8, rs2: u8 } }
pub enum DecodeError { Invalid(u32) }
pub fn decode_instruction(_: u32) -> Result<Instruction, DecodeError> { Ok(Instruction::Add { rd: 0, rs1: 0, rs2: 0 }) }