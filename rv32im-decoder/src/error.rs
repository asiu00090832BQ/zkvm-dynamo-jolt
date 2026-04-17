[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkvmError {
    InvalidOpcode(u32),
    InvalidFunct3(u32),
    InvalidFunct7(u32),
    MalformedInstruction(u32),
}

pub type ZkvmResult<T> = Result<T, ZkvmError>;
