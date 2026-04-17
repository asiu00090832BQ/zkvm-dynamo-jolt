use crate::{error::ZkvmError, instruction::{Instruction, Register}};
pub fn ensure_32bit_instruction(w: u32) -> Result<(), ZkvmError> { if w & 0b11 == 0b11 { Ok(()) } else { Err(ZkvmError::InvalidInstructionLength { word: w }) } }
pub fn ensure_register(r: Register) -> Result<(), ZkvmError> { if r < 32 { Ok(()) } else { Err(ZkvmError::InvalidRegister { reg: r }) } }
pub fn validate_instruction(_b: &Instruction) -> Result<(), ZkvmError> { Ok(()) }
pub fn sign_extend(v: u32, b: u8) -> i32 { let s = 32 - b; ((v << s) as i32) >> s }
pub fn ensure_alignment(_v: u32, _a: u32) -> Result<(), ZkvmError> { Ok(()) }
pub fn ensure_shamt(_s: u8) -> Result<(), ZkvmError> { Ok(()) }
