use crate::{
    error::ZkvmError,
    types::Register,
};

pub fn ensure_standard_32bit(word: u32) -> Result<(), ZkvmError> {
    if word & 0b11 != 0b11 {
        Err(ZkvmError::CompressedInstructionUnsupported { word })
    } else {
        Ok(())
    }
}

pub fn register(index: u8, context: &'static str) -> Result<Register, ZkvmError> {
    Register::new(index).map_err(|_| ZkvmError::InvalidRegister { reg: index, context })
}

pub fn expect_funct7(
    word: u32,
    opcode: u8,
    actual: u8,
    allowed: &[u8],
) -> Result<(), ZkvmError> {
    if allowed.iter().any(|candidate| *candidate == actual) {
        Ok(())
    } else {
        Err(ZkvmError::InvalidFunct7 {
            funct7: actual,
            opcode,
            word,
        })
    }
}
