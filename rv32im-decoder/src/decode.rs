use crate::{base_i::decode_base_i, error::ZkvmError, formats::RawInstruction, instruction::Instruction, invariants::ensure_32bit_instruction, m_extension::decode_m};
pub fn is_m_extension_word(word: u32) -> bool { let r = RawInstruction::new(word); r.opcode() == 0x33 && r.funct7() == 0x01 }
pub fn decode_instruction(word: u32) -> Result<Instruction, ZkvmError> { ensure_32bit_instruction(word)?; if is_m_extension_word(word) { Ok(Instruction::M(decode_m(word)?)) } else { Ok(Instruction::BaseI(decode_base_i(word)?)) } }
pub fn decode(word: u32) -> Result<Instruction, ZkvmError> { decode_instruction(word) }
