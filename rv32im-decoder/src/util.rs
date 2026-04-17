use crate::types::{Immediate, RegisterIndex, Word};

#[inline]
pub const fn bit_slice(word: Word, start: u32, width: u32) -> Word {
    (word >> start) & ((1u32 << width) - 1)
}

#[inline]
pub const fn opcode(word: Word) -> u32 {
    bit_slice(word, 0, 7)
}

#[inline]
pub const fn rd(word: Word) -> RegisterIndex {
    bit_slice(word, 7, 5) as RegisterIndex
}

#[inline]
pub const fn funct3(word: Word) -> u32 {
    bit_slice(word, 12, 3)
}

#[inline]
pub const fn rs1(word: Word) -> RegisterIndex {
    bit_slice(word, 15, 5) as RegisterIndex
}

#[inline]
pub const fn rs2(word: Word) -> RegisterIndex {
    bit_slice(word, 20, 5) as RegisterIndex
}

#[inline]
pub const fn shamt(word: Word) -> u32 {
    bit_slice(word, 20, 5)
}

#[inline]
pub const fn funct7(word: Word) -> u32 {
    bit_slice(word, 25, 7)
}

#[inline]
pub const fn sign_extend(value: u32, width: u32) -> Immediate {
    let shift = 32 - width;
    ((value << shift) as i32) >> shift
}

#[inline]
pub const fn imm_i(word: Word) -> Immediate {
    sign_extend(bit_slice(word, 20, 12), 12)
}

#[inline]
pub const fn imm_s(word: Word) -> Immediate {
    let upper = bit_slice(word, 25, 7) << 5;
    let lower = bit_slice(word, 7, 5);
    sign_extend(upper | lower, 12)
}

#[inline]
pub const fn imm_b(word: Word) -> Immediate {
    let bit12 = bit_slice(word, 31, 1) << 12;
    let bit11 = bit_slice(word, 7, 1) << 11;
    let bits10_5 = bit_slice(word, 25, 6) << 5;
    let bits4_1 = bit_slice(word, 8, 4) << 1;
    sign_extend(bit12 | bit11 | bits10_5 | bits4_1, 13)
}

#[inline]
pub const fn imm_u(word: Word) -> Word {
    word & 0xfffff000
}

#[inline]
pub const fn imm_j(word: Word) -> Immediate {
    let bit20 = bit_slice(word, 31, 1) << 20;
    let bits19_12 = bit_slice(word, 12, 8) << 12;
    let bit11 = bit_slice(word, 20, 1) << 11;
    let bits10_1 = bit_slice(word, 21, 10) << 1;
    sign_extend(bit20 | bits19_12 | bit11 | bits10_1, 21)
}
