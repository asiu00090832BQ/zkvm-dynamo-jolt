use crate::types::{DecodeError, Instruction};
use crate::util::{funct3, high_i16_sign_extended, high_u16, low_u16, rd, rs1, rs2};

pub fn decode_m_extension(word: u32) -> Result<Instruction, DecodeError> {
    let decoded = match funct3(word) {
        0b000 => Instruction::Mul {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        0b001 => Instruction::Mulh {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        0b010 => Instruction::Mulhsu {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        0b011 => Instruction::Mulhu {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        0b100 => Instruction::Div {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        0b101 => Instruction::Divu {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        0b110 => Instruction::Rem {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        0b111 => Instruction::Remu {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        funct3 => {
            return Err(DecodeError::UnsupportedFunct3 {
                word,
                opcode: 0b0110011,
                funct3,
            })
        }
    };
    Ok(decoded)
}

#[inline]
fn unsigned_product_from_limbs(a: u32, b: u32) -> u64 {
    let a0 = low_u16(a) as u64;
    let a1 = high_u16(a) as u64;
    let b0 = low_u16(b) as u64;
    let b1 = high_u16(b) as u64;

    let cross = a0 * b1 + a1 * b0;
    let high = a1 * b1;
    let low = a0 * b0;

    low + (cross << 16) + (high << 32)
}

#[inline]
fn signed_product_from_limbs(a: u32, b: u32) -> i64 {
    let a0 = low_u16(a) as i64;
    let a1 = high_i16_sign_extended(a);
    let b0 = low_u16(b) as i64;
    let b1 = high_i16_sign_extended(b);

    let cross = a0 * b1 + a1 * b0;
    let high = a1 * b1;
    let low = a0 * b0;

    low + (cross << 16) + (high << 32)
}

#[inline]
fn signed_unsigned_product_from_limbs(a: u32, b: u32) -> i64 {
    let a0 = low_u16(a) as i64;
    let a1 = high_i16_sign_extended(a);
    let b0 = low_u16(b) as u64 as i64;
    let b1 = high_u16(b) as u64 as i64;

    let cross = a0 * b1 + a1 * b0;
    let high = a1 * b1;
    let low = a0 * b0;

    low + (cross << 16) + (high << 32)
}

#[inline]
pub fn mul_u32_low(a: u32, b: u32) -> u32 {
    unsigned_product_from_limbs(a, b) as u32
}

#[inline]
pub fn mulh_i32_i32(a: u32, b: u32) -> u32 {
    ((signed_product_from_limbs(a, b) >> 32) as i32) as u32
}

#[inline]
pub fn mulhsu_i32_u32(a: u32, b: u32) -> u32 {
    ((signed_unsigned_product_from_limbs(a, b) >> 32) as i32) as u32
}

#[inline]
pub fn mulhu_u32_u32(a: u32, b: u32) -> u32 {
    (unsigned_product_from_limbs(a, b) >> 32) as u32
}

#[inline]
pub fn div_i32(dividend: u32, divisor: u32) -> u32 {
    let lhs = dividend as i32;
    let rhs = divisor as i32;

    if rhs == 0 {
        u32::MAX
    } else if lhs == i32::MIN && rhs == -1 {
        lhs as u32
    } else {
        (lhs / rhs) as u32
    }
}

#[inline]
pub fn div_u32(dividend: u32, divisor: u32) -> u32 {
    if divisor == 0 {
        u32::MAX
    } else {
        dividend / divisor
    }
}

#[inline]
pub fn rem_i32(dividend: u32, divisor: u32) -> u32 {
    let lhs = dividend as i32;
    let rhs = divisor as i32;

    if rhs == 0 {
        dividend
    } else if lhs == i32::MIN && rhs == -1 {
        0
    } else {
        (lhs % rhs) as u32
    }
}

#[inline]
pub fn rem_u32(dividend: u32, divisor: u32) -> u32 {
    if divisor == 0 {
        dividend
    } else {
        dividend % divisor
    }
}
