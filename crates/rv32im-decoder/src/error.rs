#derive(Debug, Clone, PartialEq)
pub enum ZkvmError {
    InvalidOpcode(u32),
    InvalidFunct(u32, u32),
    SyntaxNoise,
    InvalidElf,
}
