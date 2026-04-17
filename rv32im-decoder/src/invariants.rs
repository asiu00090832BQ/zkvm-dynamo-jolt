use crate::error::DecoderError;

#[inline] pub fn validate_register(reg: u8) -> Result<u8, DecoderError> { if reg < 32 { Ok(reg) } else { Err(DecoderError::InvalidRegister { reg }) } }
#[inline] pub fn validate_shift_amount(shamt: u8) -> Result<u8, DecoderError> { if shamt < 32 { Ok(shamt) } else { Err(DecoderError::InvalidShiftAmount { shamt }) } }
