pub const fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits as u32;
    ((value << shift) as i32) >> shift
}

pub const fn imm_i(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

pub const fn imm_s(word: u32) -> i32 {
    let low = (word >> 7) & 0x1f;
    let high = (word >> 25) & 0x7f;
    sign_extend(low | (high << 5), 12)
}

pub const fn imm_b(word: u32) -> i32 {
    let bit11 = ((word >> 7) & 0x1) << 11;
    let bits4_1 = ((word >> 8) & 0x0f) << 1;
    let bits10_5 = ((word >> 25) & 0x3f) << 5;
    let bit12 = ((word >> 31) & 0x1) << 12;
    sign_extend(bit12 | bit11 | bits10_5 | bits4_1, 13)
}

pub const fn imm_u(word: u32) -> i32 {
    (word & 0xfffff000) as i32
}

pub const fn imm_j(word: u32) -> i32 {
    let bits19_12 = ((word >> 12) & 0xff) << 12;
    let bit11 = ((word >> 20) & 0x1) << 11;
    let bits10_1 = ((word >> 21) & 0x03ff) << 1;
    let bit20 = ((word >> 31) & 0x1) << 20;
    sign_extend(bit20 | bits19_12 | bit11 | bits10_1, 21)
}
