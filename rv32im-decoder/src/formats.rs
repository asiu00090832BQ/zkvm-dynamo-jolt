#[inline]
pub const fn is_32bit(word: u32) -> bool {
    (word & 0b11) == 0b11
}

[inline]
pub const fn opcode(word: u32) -> u8 {
    (word & 0x7f) as u8
}

[inline]
pub const fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1f) as u8
}

[inline]
pub const fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x07) as u8
}

[inline]
pub const fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}

[inline]
pub const fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

[inline]
pub const fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7f) as u8
}

[inline]
pub const fn shamt(word: u32) -> u8 {
    rs2(word)
}

[inline]
pub const fn imm12(word: u32) -> u16 {
    ((word >> 20) & 0x0fff) as u16
}

[inline]
pub const fn fence_pred(word: u32) -> u8 {
    ((word >> 24) & 0x0f) as u8
}

[inline]
pub const fn fence_succ(word: u32) -> u8 {
    ((word >> 20) & 0x0f) as u8
}

[inline]
pub const fn fence_fm(word: u32) -> u8 {
    ((word >> 28) & 0x0f) as u8
}

[inline]
const fn sign_extend(value: u32, width: u32) -> i32 {
    let shift = 32 - width;
    ((value << shift) as i32) >> shift
}

[inline]
pub const fn imm_i(word: u32) -> i32 {
    sign_extend((word >> 20) & 0x0fff, 12)
}

[inline]
pub const fn imm_s(word: u32) -> i32 {
    let value = (((word >> 25) & 0x7f) << 5) | ((word >> 7) & 0x1f);
    sign_extend(value, 12)
}

[inline]
pub const fn imm_b(word: u32) -> i32 {
    let bit12 = ((word >> 31) & 0x1) << 12;
    let bit11 = ((word >> 7) & 0x1) << 11;
    let bits10_5 = ((word >> 25) & 0x3f) << 5;
    let bits4_1 = ((word >> 8) & 0x0f) << 1;
    sign_extend(bit12 | bit11 | bits10_5 | bits4_1, 13)
}

[inline]
pub const fn imm_u(word: u32) -> u32 {
    word & 0xfffff000
}

[inline]
pub const fn imm_j(word: u32) -> i32 {
    let bit20 = ((word >> 31) & 0x1) << 20;
    let bits19_12 = ((word >> 12) & 0xff) << 12;
    let bit11 = ((word >> 20) & 0x1) << 11;
    let bits10_1 = ((word >> 21) & 0x03ff) << 1;
    sign_extend(bit20 | bits19_12 | bit11 | bits10_1, 21)
}
