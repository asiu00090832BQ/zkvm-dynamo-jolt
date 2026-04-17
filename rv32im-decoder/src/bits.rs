#[inline]
pub const fn bit_mask(width: u8) -> u32 {
    if width >= 32 {
        u32::MAX
    } else if width == 0 {
        0
    } else {
        (1u32 << width) - 1
    }
}

#[inline]
pub const fn field(word: u32, lsb: u8, width: u8) -> u32 {
    (word >> lsb) & bit_mask(width)
}

#[inline]
pub const fn is_32bit(word: u32) -> bool {
    (word & 0b11) == 0b11
}

#[inline]
pub const fn opcode(word: u32) -> u8 {
    field(word, 0, 7) as u8
}

#[inline]
pub const fn rd(word: u32) -> u8 {
    field(word, 7, 5) as u8
}

#[inline]
pub const fn funct3(word: u32) -> u8 {
    field(word, 12, 3) as u8
}

#[inline]
pub const fn rs1(word: u32) -> u8 {
    field(word, 15, 5) as u8
}

#[inline]
pub const fn rs2(word: u32) -> u8 {
    field(word, 20, 5) as u8
}

#[inline]
pub const fn shamt(word: u32) -> u8 {
    field(word, 20, 5) as u8
}

#[inline]
pub const fn funct7(word: u32) -> u8 {
    field(word, 25, 7) as u8
}

#[inline]
pub const fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits as u32;
    ((value << shift) as i32) >> shift
}

#[inline]
pub const fn imm_i(word: u32) -> i32 {
    sign_extend(field(word, 20, 12), 12)
}

#[inline]
pub const fn imm_s(word: u32) -> i32 {
    let value = (field(word, 25, 7) << 5) | field(word, 7, 5);
    sign_extend(value, 12)
}

#[inline]
pub const fn imm_b(word: u32) -> i32 {
    let value = (field(word, 31, 1) << 12)
        | (field(word, 7, 1) << 11)
        | (field(word, 25, 6) << 5)
        | (field(word, 8, 4) << 1);
    sign_extend(value, 13)
}

#[inline]
pub const fn imm_u(word: u32) -> u32 {
    word & 0xfffff000
}

#[inline]
pub const fn imm_j(word: u32) -> i32 {
    let value = (field(word, 31, 1) << 20)
        | (field(word, 12, 8) << 12)
        | (field(word, 20, 1) << 11)
        | (field(word, 21, 10) << 1);
    sign_extend(value, 21)
}

#[inline]
pub const fn fence_pred(word: u32) -> u8 {
    field(word, 24, 4) as u8
}

#[inline]
pub const fn fence_succ(word: u32) -> u8 {
    field(word, 20, 4) as u8
}

#[inline]
pub const fn fence_fm(word: u32) -> u8 {
    field(word, 28, 4) as u8
}

#[inline]
pub const fn system_imm12(word: u32) -> u16 {
    field(word, 20, 12) as u16
}
