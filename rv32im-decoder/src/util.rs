#[inline]
pub fn opcode(word: u32) -> u8 {
    (word & 0x7f) as u8
}

#[inline]
pub fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1f) as u8
}

#[inline]
pub fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x07) as u8
}

#[inline]
pub fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}

#[inline]
pub fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

#[inline]
pub fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7f) as u8
}

#[inline]
pub fn shamt(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

#[inline]
pub fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32_u32 - bits as u32;
    ((value << shift) as i32) >> shift
}

#[inline]
pub fn imm_i(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

#[inline]
pub fn imm_s(word: u32) -> i32 {
    let value = ((word >> 25) << 5) | ((word >> 7) & 0x1f);
    sign_extend(value, 12)
}

#[inline]
pub fn imm_b(word: u32) -> i32 {
    let value = (((word >> 31) & 0x1) << 12)
        | (((word >> 7) & 0x1) << 11)
        | (((word >> 25) & 0x3f) << 5)
        | (((word >> 8) & 0x0f) << 1);
    sign_extend(value, 13)
}

#[inline]
pub fn imm_u(word: u32) -> u32 {
    word & 0xfffff000
}

#[inline]
pub fn imm_j(word: u32) -> i32 {
    let value = (((word >> 31) & 0x1) << 20)
        | (((word >> 12) & 0xff) << 12)
        | (((word >> 20) & 0x1) << 11)
        | (((word >> 21) & 0x03ff) << 1);
    sign_extend(value, 21)
}

#[inline]
pub fn as_i32(value: u32) -> i32 {
    value as i32
}

#[inline]
pub fn low_u16(value: u32) -> u32 {
    value & 0xffff
}

#[inline]
pub fn high_u16(word: u32) -> u32 {
    word >> 16
}

#[inline]
pub fn high_i16_sign_extended(word: u32) -> i64 {
    ((word as i32) >> 16) as i64
}
