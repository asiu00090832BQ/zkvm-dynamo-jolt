use crate::error::{Result, ZkvmError};
use crate::fields::RawInstruction;

pub fn validate_word(word: u32) -> Result<()> {
    if word & 0b11 != 0b11 {
        return Err(ZkvmError::InvalidEncoding {
            word,
            reason: "compressed instructions are not supported",
        });
    }

    Ok(())
}

pub fn validate_register(index: u8) -> Result<()> {
    if index < 32 {
        Ok(())
    } else {
        Err(ZkvmError::InvalidRegister { index })
    }
}

pub fn validate_raw_registers(raw: RawInstruction) -> Result<()> {
    validate_register(raw.rd())?;
    validate_register(raw.rs1())?;
    validate_register(raw.rs2())?;
    Ok(())
}

pub fn validate_shift_funct7(raw: RawInstruction) -> Result<()> {
    match raw.funct7() {
        0b0000000 | 0b0100000 => Ok(()),
        _ => Err(ZkvmError::UnsupportedFunct7 {
            funct7: raw.funct7(),
            funct3: raw.funct3(),
            opcode: raw.opcode(),
            word: raw.word(),
        }),
    }
}

pub fn validate_rtype_funct7(raw: RawInstruction) -> Result<()> {
    match raw.funct7() {
        0b0000000 | 0b0100000 | 0b0000001 => Ok(()),
        _ => Err(ZkvmError::UnsupportedFunct7 {
            funct7: raw.funct7(),
            funct3: raw.funct3(),
            opcode: raw.opcode(),
            word: raw.word(),
        }),
    }
}
