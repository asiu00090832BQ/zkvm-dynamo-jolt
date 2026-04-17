pub fn extract_field(word: u32, start: usize, len: usize) -> u32 {
    (word >> start) & ((1 << len) - 1)
}