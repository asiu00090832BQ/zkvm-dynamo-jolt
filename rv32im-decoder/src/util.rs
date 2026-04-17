pub const fn bit(word: u32, index: u8) -> u32 {
    (word >> index) & 1
}

pub const fn bits(word: u32, hi: u8, lo: u8) -> u32 {
    let width = (hi - lo + 1) as u32;
    let mask = if width >= 32 {
        u32::MAX
    } else {
        (1u32 << width) - 1
    };
    (word >> lo) & mask
}

pub const fn wrapping_add_signed(lhs: u32, rhs: i32) -> u32 {
    lhs.wrapping_add(rhs as u32)
}

pub const fn low_u32(value: u64) -> u32 {
    value as u32
}

pub const fn high_u32(value: u64) -> u32 {
    (value >> 32) as u32
}
