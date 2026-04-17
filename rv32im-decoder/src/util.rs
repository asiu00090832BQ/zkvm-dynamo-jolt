use crate::types::{Limb16, OperandDecomposition};

pub const fn bit_mask(width: u8) -> u32 {
    if width >= 32 {
        u32::MAX
    } else {
        (1u32 << width) - 1
    }
}

pub const fn bits(value: u32, start: u8, width: u8) -> u32 {
    (value >> start) & bit_mask(width)
}

pub const fn opcode(word: u32) -> u8 {
    bits(word, 0, 7) as u8
}

pub const fn rd(word: u32) -> u8 {
    bits(word, 7, 5) as u8
}

pub const fn funct3(word: u32) -> u8 {
    bits(word, 12, 3) as u8
}

pub const fn rs1(word: u32) -> u8 {
    bits(word, 15, 5) as u8
}

pub const fn rs2(word: u32) -> u8 {
    bits(word, 20, 5) as u8
}

pub const fn shamt(word: u32) -> u8 {
    bits(word, 20, 5) as u8
}

pub const fn funct7(word: u32) -> u8 {
    bits(word, 25, 7) as u8
}

pub const fn sign_extend(value: u32, width: u8) -> i32 {
    let shift = 32u32 - width as u32;
    ((value << shift) as i32) >> shift
}

pub const fn imm_i(word: u32) -> i32 {
    sign_extend(bits(word, 20, 12), 12)
}

pub const fn imm_s(word: u32) -> i32 {
    let value = bits(word, 7, 5) | (bits(word, 25, 7) << 5);
    sign_extend(value, 12)
}

pub const fn imm_b(word: u32) -> i32 {
    let value = (bits(word, 8, 4) << 1)
        | (bits(word, 25, 6) << 5)
        | (bits(word, 7, 1) << 11)
        | (bits(word, 31, 1) << 12);
    sign_extend(value, 13)
}

pub const fn imm_u(word: u32) -> i32 {
    (word & 0xFFFF_F000) as i32
}

pub const fn imm_j(word: u32) -> i32 {
    let value = (bits(word, 21, 10) << 1)
        | (bits(word, 20, 1) << 11)
        | (bits(word, 12, 8) << 12)
        | (bits(word, 31, 1) << 20);
    sign_extend(value, 21)
}

pub const fn decompose_operand(value: u32) -> Limb16 {
    Limb16::from_u32(value)
}

pub const fn decompose_operands(a: u32, b: u32) -> OperandDecomposition {
    OperandDecomposition::from_operands(a, b)
}
