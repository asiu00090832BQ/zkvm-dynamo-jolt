use zkvm_core::ZkvmError;

pub type DecodeResult<T> = Result<T, DecoderError>;

#[derive(Debug)]
pub enum DecoderError {
    UnknownOpcode { raw: u32, opcode: u8 },
    UnsupportedFunct3 { raw: u32, funct3: u8 },
    UnsupportedFunct7 { raw: u32, funct7: u8 },
    InvalidRegister { reg: u8 },
    InvariantViolation(&'static str),
    Core(ZkvmError),
}

impl From<ZkvmError> for DecoderError {
    fn from(err: ZkvmError) -> Self {
        Self::Core(err)
    }
}
