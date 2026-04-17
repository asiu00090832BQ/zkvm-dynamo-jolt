use crate::error::ZkvmError;
use crate::formats::RType;
use crate::instruction::{AluRegKind, Instruction};

#[inline(always)]
pub const fn split_u32_to_u16_limbs(value: u32) -> (u16, u16) {
    ((value & 0xffff) as u16, (value >> 16) as u16)
}

/// Lemma 6.1.1:
/// P = (a1 * b1) << 32 + (a1 * b0 + a0 * b1) << 16 + a0 * b0
#[inline]
pub fn mul_u32_limb(a: u32, b: u32) -> u64 {
    let (a0, a1) = split_u32_to_u16_limbs(a);
    let (b0, b1) = split_u32_to_u16_limbs(b);

    let a0 = a0 as u64;
    let a1 = a1 as u64;
    let b0 = b0 as u64;
    let b1 = b1 as u64;

    ((a1 * b1) << 32) + ((a1 * b0 + a0 * b1) << 16) + (a0 * b0)
}

#[inline]
fn mul_i32_full(a: i32, b: i32) -> i64 {
    let a_negative = a < 0;
    let b_negative = b < 0;
    let a_mag = a.wrapping_abs() as u32;
    let b_mag = b.wrapping_abs() as u32;
    let magnitude = mul_u32_limb(a_mag, b_mag) as i64;

    if a_negative ^ b_negative {
        -magnitude
    } else {
        magnitude
    }
}

#[inline]
fn mul_i32_u32_full(a: i32, b: u32) -> i64 {
    let a_negative = a < 0;
    let a_mag = a.wrapping_abs() as u32;
    let magnitude = mul_u32_limb(a_mag, b) as i64;

    if a_negative {
        -magnitude
    } else {
        magnitude
    }
}

#[inline]
pub fn execute_m(kind: AluRegKind, lhs: u32, rhs: u32) -> Option<u32> {
    let value = match kind {
        AluRegKind::Mul => mul_u32_limb(lhs, rhs) as u32,
        AluRegKind::Mulh => ((mul_i32_full(lhs as i32, rhs as i32) as u64) >> 32) as u32,
        AluRegKind::Mulhsu => ((mul_i32_u32_full(lhs as i32, rhs) as u64) >> 32) as u32,
        AluRegKind::Mulhu => (mul_u32_limb(lhs, rhs) >> 32) as u32,
        AluRegKind::Div => {
            let lhs = lhs as i32;
            let rhs = rhs as i32;
            if rhs == 0 {
                u32::MAX
            } else if lhs == i32::MIN && rhs == -1 {
                lhs as u32
            } else {
                (lhs / rhs) as u32
            }
        }
        AluRegKind::Divu => {
            if rhs == 0 {
                u32::MAX
            } else {
                lhs / rhs
            }
        }
        AluRegKind::Rem => {
            let lhs = lhs as i32;
            let rhs = rhs as i32;
            if rhs == 0 {
                lhs as u32
            } else if lhs == i32::MIN && rhs == -1 {
                0
            } else {
                (lhs % rhs) as u32
            }
        }
        AluRegKind::Remu => {
            if rhs == 0 {
                lhs
            } else {
                lhs % rhs
            }
        }
        _ => return None,
    };

    Some(value)
}

pub fn decode_m_extension(word: u32) -> Result<Instruction, ZkvmError> {
    let r = RType::decode(word);
    if r.funct7 != 0x01 {
        return Err(ZkvmError::InvalidInstruction(word));
    }

    let kind = match r.funct3 {
        0x0 => AluRegKind::Mul,
        0x1 => AluRegKind::Mulh,
        0x2 => AluRegKind::Mulhsu,
        0x3 => AluRegKind::Mulhu,
        0x4 => AluRegKind::Div,
        0x5 => AluRegKind::Divu,
        0x6 => AluRegKind::Rem,
        0x7 => AluRegKind::Remu,
        _ => return Err(ZkvmError::InvalidInstruction(word)),
    };

    Ok(Instruction::Op {
        kind,
        rd: r.rd,
        rs1: r.rs1,
        rs2: r.rs2,
    })
}
