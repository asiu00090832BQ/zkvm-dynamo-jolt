use crate::error::DecodeError;
use crate::instruction::Instruction;
use crate::selectors::{funct3, rd, rs1, rs2};

pub const M_FUNCT7: u32 = 0b0000001;

pub const fn decompose_u32_16(value: u32) -> [u16; 2] {
    [(value & 0xffff) as u16, (value >> 16) as u16]
}

pub const fn recompose_u32_16(limbs: [u16; 2]) -> u32 {
    (limbs[0] as u32) | ((limbs[1] as u32) << 16)
}

pub const fn decompose_i32_16(value: i32) -> [u16; 2] {
    decompose_u32_16(value as u32)
}

pub const fn recompose_i32_16(limbs: [u16; 2]) -> i32 {
    recompose_u32_16(limbs) as i32
}

pub fn mul_u32_wide_16(lhs: u32, rhs: u32) -> u64 {
    let [a0, a1] = decompose_u32_16(lhs);
    let [b0, b1] = decompose_u32_16(rhs);

    let a0 = u64::from(a0);
    let a1 = u64::from(a1);
    let b0 = u64::from(b0);
    let b1 = u64::from(b1);

    let lo = a0 * b0;
    let mid = a0 * b1 + a1 * b0;
    let hi = a1 * b1;

    lo + (mid << 16) + (hi << 32)
}

pub fn mul_i32_wide_16(lhs: i32, rhs: i32) -> i64 {
    let negative = (lhs < 0) ^ (rhs < 0);
    let lhs_abs = if lhs < 0 { lhs.wrapping_neg() as u32 } else { lhs as u32 };
    let rhs_abs = if rhs < 0 { rhs.wrapping_neg() as u32 } else { rhs as u32 };
    let product = i128::from(mul_u32_wide_16(lhs_abs, rhs_abs));

    if negative {
        (-product) as i64
    } else {
        product as i64
    }
}

pub fn mul_i32_u32_wide_16(lhs: i32, rhs: u32) -> i64 {
    let negative = lhs < 0;
    let lhs_abs = if lhs < 0 { lhs.wrapping_neg() as u32 } else { lhs as u32 };
    let product = i128::from(mul_u32_wide_16(lhs_abs, rhs));

    if negative {
        (-product) as i64
    } else {
        product as i64
    }
}

pub fn decode_m_instruction(word: u32) -> Result<Instruction, DecodeError> {
    let rd = rd(word);
    let rs1 = rs1(word);
    let rs2 = rs2(word);

    Ok(match funct3(word) {
        0b000 => Instruction::Mul { rd, rs1, rs2 },
        0b001 => Instruction::Mulh { rd, rs1, rs2 },
        0b010 => Instruction::Mulhsu { rd, rs1, rs2 },
        0b011 => Instruction::Mulhu { rd, rs1, rs2 },
        0b100 => Instruction::Div { rd, rs1, rs2 },
        0b101 => Instruction::Divu { rd, rs1, rs2 },
        0b110 => Instruction::Rem { rd, rs1, rs2 },
        0b111 => Instruction::Remu { rd, rs1, rs2 },
        _ => unreachable!(),
    })
}
