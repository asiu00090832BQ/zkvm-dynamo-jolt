use crate::error::{Result, ZkvmError};
use crate::fields::{RType, RawInstruction};
use crate::instruction::{ArithmeticOp, Instruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limb16 {
    pub low: u16,
    pub high: u16,
}

pub fn decompose(val: u32) -> Limb16 {
    Limb16 {
        low: val as u16,
        high: (val >> 16) as u16,
    }
}

pub fn mul_u32_wide(lhs: u32, rhs: u32) -> u64 {
    let left = decompose(lhs);
    let right = decompose(rhs);

    let low = (left.low as u64) * (right.low as u64);
    let cross = (left.low as u64) * (right.high as u64)
        + (left.high as u64) * (right.low as u64);
    let high = (left.high as u64) * (right.high as u64);

    low + (cross << 16) + (high << 32)
}

pub fn lemma_6_1_1_parity(lhs: u32, rhs: u32) -> bool {
    mul_u32_wide(lhs, rhs) == (lhs as u64) * (rhs as u64)
}

pub fn mulhu(lhs: u32, rhs: u32) -> u32 {
    (mul_u32_wide(lhs, rhs) >> 32) as u32
}

pub fn mulh(lhs: i32, rhs: i32) -> i32 {
    (((lhs as i64) * (rhs as i64)) >> 32) as i32
}

pub fn mulhsu(lhs: i32, rhs: u32) -> i32 {
    (((lhs as i64) * (rhs as i64)) >> 32) as i32
}

pub(crate) fn decode_m_extension(raw: RawInstruction) -> Result<Instruction> {
    if raw.opcode() != 0b0110011 || raw.funct7() != 0b0000001 {
        return Err(ZkvmError::InvalidEncoding {
            word: raw.word(),
            reason: "M-extension decode requires opcode 0b0110011 and funct7 0b0000001",
        });
    }

    let op = match raw.funct3() {
        0b000 => ArithmeticOp::Mul,
        0b001 => ArithmeticOp::Mulh,
        0b010 => ArithmeticOp::Mulhsu,
        0b011 => ArithmeticOp::Mulhu,
        0b100 => ArithmeticOp::Div,
        0b101 => ArithmeticOp::Divu,
        0b110 => ArithmeticOp::Rem,
        0b111 => ArithmeticOp::Remu,
        _ => {
            return Err(ZkvmError::UnsupportedFunct3 {
                funct3: raw.funct3(),
                opcode: raw.opcode(),
                word: raw.word(),
            })
        }
    };

    Ok(Instruction::Op(op, RType::from(raw)))
}
