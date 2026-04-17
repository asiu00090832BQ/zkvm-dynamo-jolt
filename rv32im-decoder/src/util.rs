pub fn extract_u32(word: u32, start: u8, end: u8) -> u32 {
    (word >> start) & ((1 << (end - start + 1)) - 1)
}
