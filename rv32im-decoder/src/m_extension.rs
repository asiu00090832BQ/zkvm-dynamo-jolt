use crate::{
    error::{DecodeResult, DecoderError},
    formats::RType,
    instruction::{DecodedInstruction, MInstruction},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Limb16 {
    pub lo: u16,
    pub hi: u16,
}

pub fn decompose_u32(value: u32) -> Limb16 {
    Limb16 {
        lo: value as u16,
        hi: (value >> 16) as u16,
    }
}

pub fn plan_mul_limbs(lhs: u32, rhs: u32) -> [(u32, u32); 4] {
    let l = decompose_u32(lhs);
    let r = decompose_u32(rhs);
    [
        (l.lo as u32, r.lo as u32),
        (l.lo as u32, r.hi as u32),
        (l.hi as u32, r.lo as u32),
        (l.hi as u32, r.hi as u32),
    ]
}

__helper_mul_wide_unsigned__
pub fn mul_wide_unsigned(a: u32, b: u32) -> u64 {
    let pairs = plan_mul_limbs(a, b);
    let (a0, b0) = pairs[0];
    let (a01, b11) = pairs[1];
    let (a11, b01) = pairs[2];
    let (a1, b1) = pairs[3];
    let p00 = (a0 as u64) * (b0 as u64);
    let p01 = (a01 as u64) * (b11 as u64);
    let p10 = (a11 as u64) * (b01 as u64);
    let p11 = (a1 as u64) * (b1 as u64);
    p00 + ((p01 + p10) << 16) + (p11 << 32)
}

pub fn mul(a: u32, b: u32) -> u32 { mul_wide_unsigned(a, b) as u32 }
pub fn mulhu(a: u32, b: u32) -> u32 { (mul_wide_unsigned(a, b) >> 32) as u32 }
pub fn mulh(a: u32, b: u32) -> u32 {
    let p = (a as i32 as i128) * (b as i32 as i128);
    (p >> 32) as u32
}
pub fn mulhsu(a: u32, b: u32) -> u32 {
    let p = (a as i32 as i128) * (b as i128);
    (p >> 32) as u32
}

pub fn div(a: u32, b: u32) -> u32 {
    let lhs = a as i32; let rhs = b as i32;
    if rhs == 0 { u32::MAX }
    else if lhs == i32::MIN && rhs == -1 { lhs as u32 }
    else { (lhs / rhs) as u32 }
}
pub fn divu(a: u32, b: u32) -> u32 { if b == 0 { u32::MAX } else { a / b } }
pub fn rem(a: u32, b: u32) -> u32 {
    let lhs = a as i32; let rhs = b as i32;
    if rhs == 0 { a }
    else if lhs == i32::MIN && rhs == -1 { 0 }
    else { (lhs % rhs) as u32 }
}
pub fn remu(a: u32, b: u32) -> u32 { if b == 0 { a } else { a % b } }
