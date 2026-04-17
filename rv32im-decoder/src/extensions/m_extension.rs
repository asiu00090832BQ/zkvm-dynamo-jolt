use crate::types::Instruction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LimbDecomposition { pub a0: u16, pub a1: u16 }

pub const fn split_u32_to_limbs(value: u32) -> LimbDecomposition {
    LimbDecomposition { a0: (value & 0xffff) as u16, a1: (value >> 16) as u16 }
}

pub const fn abs_i32_as_u32(value: i32) -> u32 {
    if value < 0 { value.wrapping_neg() as u32 } else { value as u32 }
}

pub fn mul_low(lhs: u32, rhs: u32) -> u32 { lhs.wrapping_mul(rhs) }
pub fn mulh(lhs: u32, rhs: u32) -> u32 { ((lhs as i32 as i64).wrapping_mul(rhs as i32 as i64) >> 32) as u32 }
pub fn mulhsu(lhs: u32, rhs: u32) -> u32 { ((lhs as i32 as i64).wrapping_mul(rhs as u64 as i64) >> 32) as u32 }
pub fn mulhu(lhs: u32, rhs: u32) -> u32 { ((lhs as u64).wrapping_mul(rhs as u64) >> 32) as u32 }

pub fn div(lhs: u32, rhs: u32) -> u32 {
    let (l, r) = (lhs as i32, rhs as i32);
    if r == 0 { u32::MAX } else if l == i32::MIN && r == -1 { l as u32 } else { (l / r) as u32 }
}
pub fn divu(lhs: u32, rhs: u32) -> u32 { if rhs == 0 { u32::MAX } else { lhs / rhs } }
pub fn rem(lhs: u32, rhs: u32) -> u32 {
    let (l, r) = (lhs as i32, rhs as i32);
    if r == 0 { lhs } else if l == i32::MIN && r == -1 { 0 } else { (l % r) as u32 }
}
pub fn remu(lhs: u32, rhs: u32) -> u32 { if rhs == 0 { lhs } else { lhs % rhs } }

pub fn execute_m_instruction(inst: Instruction, lhs: u32, rhs: u32) -> Option<u32> {
    match inst {
        Instruction::Mul => Some(mul_low(lhs, rhs)),
        Instruction::Mulh => Some(mulh(lhs, rhs)),
        Instruction::Mulhsu => Some(mulhsu(lhs, rhs)),
        Instruction::Mulhu => Some(mulhu(lhs, rhs)),
        Instruction::Div => Some(div(lhs, rhs)),
        Instruction::Divu => Some(divu(lhs, rhs)),
        Instruction::Rem => Some(rem(lhs, rhs)),
        Instruction::Remu => Some(remu(lhs, rhs)),
        _ => None,
    }
}