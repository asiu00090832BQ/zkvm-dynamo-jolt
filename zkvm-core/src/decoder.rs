use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecoderConfig {
    pub enable_rv32m: bool,
}

impl Default for DecoderConfig {
    fn default() -> Self {
        Self { enable_rv32m: true }
    }
}
