pub const fn sign_extend(value: u32, bits: u8) -> i32 {
    if bits >= 32 {
        value as i32
    } else {
        let shift = 32 - bits as u32;
        ((value << shift) as i32) >> shift
    }
}

pub const fn zero_extend(value: u32, bits: u8) -> u32 {
    if bits >= 32 {
        value
    } else {
        value & ((1u32 << bits) - 1)
    }
}

pub const fn sign_extend_byte(value: u8) -> i32 {
    (value as i8) as i32
}

pub const fn sign_extend_half(value: u16) -> i32 {
    (value as i16) as i32
}
