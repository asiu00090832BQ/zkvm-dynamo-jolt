#[derive(Debug, Clone)]
pub struct ZgvmConfig {
    pub max_cyles: u64,
    pub memory_limit: usize,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            max_cyles: 1_000_000,
            memory_limit: 64 * 1024 * 1024,
        }
    }
}
