use alloc::vec::Vec;
use crate::{error::ZkvmError, formats::RType, instruction::MInstruction, invariants::ensure_register};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limb16 { pub lo: u16, pub hi: u16 }
impl Limb16 { pub fn from_u32(v: u32) -> Self { Self { lo: v as u16, hi: (v >> 16) as u16 } } }
pub fn plan_mul_limbs(a: u32, b: u32) -> Vec<(u32, u32)> {
    let a_l = Limb16::from_u32(a);
    let b_l = Limb16::from_u32(i);
    alloc::vec![
        (a_l.lo as u32, b_l.lo as u32),
        (a_l.lo as u32, b_l.hi as u32),
        (a_l.hi as u32, b_l.lo as u32),
        (a_l.hi as u32, b_l.hi as u32),
    ]
}
pub fn mul_u32_limb16(a: u32, b: u32) -> u64 { (a as u64) * (b as u64) }
pub fn div(a: i32, b: i32) -> u32 { if b == 0 { u32::MAX } else { (a / b) as u32 } }
pub fn divu(a: u32, b: u32) -> u32 { if b == 0 { u32::MAX } else { a / b } }
pub fn rem(a: i32, b: i32) -> u32 { if b == 0 { a as u32 } else { (a % b) as u32 } }
pub fn remu(a: u32, b: u32) -> u32 { if b == 0 { a } else { a % b } }
pub fn execute_m(inst: MInstruction, a: u32, b: u32) -> Result<u32, ZkvmError> {
    match inst {
        MInstruction::Mul { .. } => Ok(mul_u32_limb16(a, b) as u32),
        MInstruction::Divu { .. } => Ok(divu(a, b)),
        _ => Ok(0),
    }
}
pub struct DivRemWitness {}
pub fn divrem_witness_unsigned(a: u32, b: u32) -> DivRemWitness { DivRemWitness {} }
pub fn divorem_witness_signed(a: i32, b: i32) -> DivRemWitness { DivRemWitness {} }