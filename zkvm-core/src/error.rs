impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            max_cycles: 1_000_000,
            memory_limit: 64 * 1024 * 1024,
        }
    }
}
