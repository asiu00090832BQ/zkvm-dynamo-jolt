use crate::error::DecoderError;
use crate::instruction::{Instruction, MulKind};
use crate::selectors::{FUNCT7_M, OPCODE_OP};
use crate::util::{funct3, funct7, rd, rs1, rs2};
use crate::types::Word;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Product64 {
    pub low: u32,
    pub high: u32,
}

#[inline]
pub const fn limbs16(word: u32) -> (u32, u32) {
    (word & 0xffff, word >> 16)
}

#[inline]
pub fn lemma_6_1_1_product(lhs: u32, rhs: u32) -> Product64 {
    let (a0, a1) = limbs16(lhs);
    let (b0, b1) = limbs16(rhs);

    let p0 = (a0 as u64) * (b0 as u64);
    let p1 = (a0 as u64) * (b1 as u64) + (a1 as u64) * (b0 as u64);
    let p2 = (a1 as u64) * (b1 as u64);

    let acc = p0 + (p1 << 16) + (p2 << 32);

    Product64 {
        low: acc as u32,
        high: (acc >> 32) as u32,
    }
}

#[inline]
pub fn mulhu(lhs: u32, rhs: u32) -> u32 {
    lemma_6_1_1_product(lhs, rhs).high
}

#[inline]
pub fn mulh(lhs: i32, rhs: i32) -> u32 {
    let product = lemma_6_1_1_product(lhs as u32, rhs as u32);
    let mut high = product.high;
    if lhs < 0 {
        high = high.wrapping_sub(rhs as u32);
    }
    if rhs < 0 {
        high = high.wrapping_sub(lhs as u32);
    }
    high
}

#[inline]
pub fn mulhsu(lhs: i32, rhs: u32) -> u32 {
    let product = lemma_6_1_1_product(lhs as u32, rhs);
    if lhs < 0 {
        product.high.wrapping_sub(rhs)
    } else {
        product.high
    }
}

pub fn execute_mul_kind(kind: MulKind, lhs: u32, rhs: u32) -> u32 {
    match kind {
        MulKind::Mul => lemma_6_1_1_product(lhs, rhs).low,
        MulKind::Mulh => mulh(lhs as i32, rhs as i32),
        MulKind::Mulhsu => mulhsu(lhs as i32, rhs),
        MulKind::Mulhu => mulhu(lhs, rhs),
        MulKind::Div => {
            let dividend = lhs as i32;
            let divisor = rhs as i32;
            if divisor == 0 {
                u32::MAX
            } else if dividend == i32::MIN && divisor == -1 {
                dividend as u32
            } else {
                (dividend / divisor) as u32
            }
        }
        MulKind::Divu => {
            if rhs == 0 {
                u32::MAX
            } else {
                lhs / rhs
            }
        }
        MulKind::Rem => {
            let dividend = lhs as i32;
            let divisor = rhs as i32;
            if divisor == 0 {
                lhs
            } else if dividend == i32::MIN && divisor == -1 {
                0
            } else {
                (dividend % divisor) as u32
            }
        }
        MulKind::Remu => {
            if rhs == 0 {
                lhs
            } else {
                lhs % rhs
            }
        }
    }
}

pub fn decode_m(word: Word) -> Result<Instruction, DecoderError> {
    if funct7(word) != FUNCT7_M {
        return Err(DecoderError::InvalidFunct7 {
            opcode: OPCODE_OP,
            funct3: funct3(word),
            funct7: funct7(word),
        });
    }

    let kind = match funct3(word) {
        0b000 => MulKind::Mul,
        0b001 => MulKind::Mulh,
        0b010 => MulKind::Mulhsu,
        0b011 => MulKind::Mulhu,
        0b100 => MulKind::Div,
        0b101 => MulKind::Divu,
        0b110 => MulKind::Rem,
        0b111 => MulKind::Remu,
        other => {
            return Err(DecoderError::InvalidFunct3 {
                opcode: OPCODE_OP,
                funct3: other,
            });
        }
    };

    Ok(Instruction::Mul {
        kind,
        rd: rd(word),
        rs1: rs1(word),
        rs2: rs2(word),
    })
}
