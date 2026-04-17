use alloc::vec::Vec;
use crate::{error::DecodeError, formats::RType, instruction::Instruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limb16 { pub lo: u16, pub hi: u16 }

impl Limb16 {
    pub fn from_u32(v: u32) -> Self {
        Self { lo: v as u16, hi: (v >> 16) as u16 }
    }
}

pub fn plan_mul_limbs(a: u32, b: u32) -> Vec<(u32, u32)> {
    let a_l = Limb16::from_u32(a);
    let b_l = Limb16::from_u32(b);
    alloc::vec![
        (a_l.lo as u32, b_l.lo as u32),
        (a_l.lo as u32, b_l.hi as u32),
        (a_l.hi as u32, b_l.lo as u32),
        (a_l.hi as u32, b_l.hi as u32),
    ]
}
