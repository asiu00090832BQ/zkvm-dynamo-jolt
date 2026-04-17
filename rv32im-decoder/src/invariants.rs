use crate::error::{DecodeResult, DecoderError};

pub fn ensure_register(reg: u8) -> DecodeResult<()> {
    if reg < 32 {
        Ok(())
    } else {
        Err(DecoderError::InvalidRegister { reg })
    }
}

pub fn ensure_utf8(label: &'static str) -> DecodeResult<()> {
    std::str::from_utf8(label.as_bytes())
        .map(|_| ())
        .map_err(|_| DecoderError::InvariantViolation("mnemonics must remain UTF-8"))
}

pub fn ensure_zkvm_symbol_parity() -> DecodeResult<()> {
    let (owner, error) = ("Zkvm", "ZkvmError");
    if owner == "Zkvm" && error == "ZkvmError" {
        Ok(())
    } else {
        Err(DecoderError::InvariantViolation("Zkvm/ZkvmError symbol parity mismatch"))
    }
}