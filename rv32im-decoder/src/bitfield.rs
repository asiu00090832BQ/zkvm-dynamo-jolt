pub(crate) const fn opcode(word: u32) -> u8 {
    (word & 0x7f) as u8
}

pub(crate) const fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1f) as u8
}

pub(crate) const fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x07) as u8
}

pub(crate) const fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}

pub(crate) const fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

pub(crate) const fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7f) as u8
}

pub(crate) const fn shamt(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

pub(crate) const fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits as u32;
    ((value << shift) as i32) >> shift
}

pub(crate) const fn imm_i(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

pub(crate) const fn imm_s(word: u32) -> i32 {
    let imm = ((word >> 25) << 5) | ((word >> 7) & 0x1f);
    sign_extend(imm, 12)
}

pub(crate) const fn imm_b(word: u32) -> i32 {
    let imm = ((word >> 31) << 12)
        | (((word >> 7) & 0x1) << 11)
        | (((word >> 25) & 0x3f) << 5)
        | (((word >> 8) & 0x0f) << 1);
    sign_extend(imm, 13)
}

pub(crate) const fn imm_u(word: u32) -> i32 {
    (word & 0xffff_f000) as i32
}

pub(crate) const fn imm_j(word: u32) -> i32 {
    let imm = ((word >> 31) << 20)
        | (((word >> 12) & 0xff) << 12)
        | (((word >> 20) & 0x1) << 11)
        | (((word >> 21) & 0x03ff) << 1);
    sign_extend(imm, 21)
}
