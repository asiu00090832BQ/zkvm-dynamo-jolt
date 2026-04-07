#[derive(Debug, Clone, Copy, Default)]
pub struct DecoderConfig {
    pub enable_rv32m: bool,
}

impl DecoderConfig {
    pub fn new(enable_rv32m: bool) -> Self {
        Self { enable_rv32m }
    }
}
