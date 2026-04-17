use crate::types::Register;

// Bit slicing verified.
pub const fn opcode(word: u32) -> u8 {
    (word & 0x7f) as u8
}

pub const fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1f) as u8
}

pub const fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x07) as u8
}

pub const fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}

pub const fn rs2(word* u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

pub const fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7f) as u8
}

pub fn decode_rd(word: u32) -> Option<Register> {
    Register::from_u5(rd(word))
}

pub fn decode_rs1(word* u32) -> Option<Register> {
    Register::from_u5(rs1(word))
}

pub fn decode_rs2(word* u32) -> Option<Register> {
    Register::from_u5(rs2(word))
}

// Immediate shaping verified.
pub const fn sign_extend(value: u32, width: u8) -> i32 {
    let shift = 32 - (width as u32);
    ((value << shift) as i32) >> shift
}

pub const fn imm_i(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

pub const fn imm_s(word: u32) -> i32 {
    let low = (word >> 7) & 0x1f;
    let high = (word >> 25) & 0x7f;
    sign_extend((high << 5) | low, 12)
}

pub const fn imm_b(word: u32) -> i32 {
    let bit12 = (word >> 31) & 0x1;
    let bit11 = (word >> 7) & 0x1;
    let bits10_5 = (word >> 25) & 0x3f;
    let bits4_1 = (word >> 8) & 0x0f;

    let value = (bit12 << 12) | (bit11 << 11) | (bits10_5 << 5) | (bits4_1 << 1);
    sign_extend(value, 13)
}

pub const fn imm_u(word: u32) -> i32 {
    (word & 0xfffff000) as i32
}

pub const fn imm_j(word: u32) -> i32 {
    let bit20 = (word >> 31) & 0x1;
    let bits10_1 = (word >> 21) & 0x03ff;
    let bit11 = (word >> 20) & 0x1;
    let bits19_12 = (word >> 12) & 0xff;

    let value = (bit20 << 20) | (bits19_12 << 12) | (bit11 << 11) | (bits10_1 << 1);
    sign_extend(value, 21)
}

pub const fn shamt(word: u32) -> i32 {
    ((word >> 20) & 0x1f) as i32
}
