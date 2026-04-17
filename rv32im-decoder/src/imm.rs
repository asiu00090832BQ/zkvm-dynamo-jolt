pub const fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits as u32;
    ((value << shift) as i32) >> shift
}

pub const fn i_imm(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

pub const fn s_imm(word: u32) -> i32 {
    let low = (word >> 7) & 0x1f;
    let high = (word >> 25) & 0x7f;
    sign_extend(low | (high << 5), 12)
}

pub const fn b_imm(word: u32) -> i32 {
    let bit11 = ((word >> 7) & 0x01) << 11;
    let bits4_1 = ((word >> 8) & 0x0f) << 1;
    let bits10_5 = ((word >> 25) & 0x3f) << 5;
    let bit12 = ((word >> 31) & 0x01) << 12;
    sign_extend(bit12 | bit11 | bits10_5 | bits4_1, 13)
}

pub const fn u_imm(word: u32) -> i32 {
    (word & 0xffff_f000) as i32
}

pub const fn j_imm(word: u32) -> i32 {
    let bits10_1 = ((word >> 21) & 0x03ff) << 1;
    let bit11 = ((word >> 20) & 0x01) << 11;
    let bits19_12 = ((word >> 12) & 0x00ff) << 12;
    let bit20 = ((word >> 31) & 0x01) << 20;
    sign_extend(bit20 | bits19_12 | bit11 | bits10_1, 21)
}

pub const fn zimm(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}
