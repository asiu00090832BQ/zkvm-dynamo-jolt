use crate::types::{Csr, Register, Word};

#[inline]
pub const fn bits(value: Word, lsb: u8, width: u8) -> Word {
    let shifted = value >> (lsb as u32);
    if width == 0 {
        0
    } else if width >= 32 {
        shifted
    } else {
        shifted & ((1u32 << (width as u32)) - 1)
    }
}

#[inline]
pub const fn sign_extend(value: Word, bit_width: u8) -> i32 {
    if bit_width == 0 {
        0
    } else {
        let shift = 32u32 - (bit_width as u32);
        ((value << shift) as i32) >> shift
    }
}

#[inline]
pub const fn opcode(word: Word) -> u8 {
    bits(word, 0, 7) as u8
}

#[inline]
pub const fn rd(word: Word) -> Register {
    bits(word, 7, 5) as Register
}

#[inline]
pub const fn funct3(word: Word) -> u8 {
    bits(word, 12, 3) as u8
}

#[inline]
pub const fn rs1(word: Word) -> Register {
    bits(word, 15, 5) as Register
}

#[inline]
pub const fn rs2(word: Word) -> Register {
    bits(word, 20, 5) as Register
}

#[inline]
pub const fn shamt(word: Word) -> u8 {
    bits(word, 20, 5) as u8
}

#[inline]
pub const fn funct7(word: Word) -> u8 {
    bits(word, 25, 7) as u8
}

#[inline]
pub const fn csr(word: Word) -> Csr {
    bits(word, 20, 12) as Csr
}

#[inline]
pub const fn imm_i(word: Word) -> i32 {
    sign_extend(bits(word, 20, 12), 12)
}

#[inline]
pub const fn imm_s(word: Word) -> i32 {
    let value = bits(word, 7, 5) | (bits(word, 25, 7) << 5);
    sign_extend(value, 12)
}

#[inline]
pub const fn imm_b(word: Word) -> i32 {
    let value = (bits(word, 8, 4) << 1)
        | (bits(word, 25, 6) << 5)
        | (bits(word, 7, 1) << 11)
        | (bits(word, 31, 1) << 12);
    sign_extend(value, 13)
}

#[inline]
pub const fn imm_u(word: Word) -> i32 {
    (word & 0xfffff000) as i32
}

#[inline]
pub const fn imm_j(word: Word) -> i32 {
    let value = (bits(word, 21, 10) << 1)
        | (bits(word, 20, 1) << 11)
        | (bits(word, 12, 8) << 12)
        | (bits(word, 31, 1) << 20);
    sign_extend(value, 21)
}

#[inline]
pub const fn fence_pred(word: Word) -> u8 {
    bits(word, 24, 4) as u8
}

#[inline]
pub const fn fence_succ(word: Word) -> u8 {
    bits(word, 20, 4) as u8
}

#[inline]
pub const fn fence_fm(word: Word) -> u8 {
    bits(word, 28, 4) as u8
}
