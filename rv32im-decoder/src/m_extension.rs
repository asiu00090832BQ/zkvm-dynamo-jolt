use crate::error::ZkvmError;

#[inline]
pub fn decompose_32bit_limbs(a: u32, b: u32) -> (u16, u16, u16, u16) {
    let a0 = (a & 0xffff) as u16;
    let a1 = (a >> 16) as u16;
    let b0  = (b & 0xffff) as u16;
    let b1 = (b >> 16) as u16;
    (a0, a1, b0, b1)
}

#[inline]
pub fn decompose_u32(x: u32) -> (u16, u16) {
    let lo = (x & 0xffff) as u16;
    let hi = (x >> 16) as u16;
    (lo, hi)
}

#[inline]
pub fn reconstruct_from_limbs(lo: u16, hi: u16) -> u32 {
    (lo as u32) | ((hi as u32) << 16)
}

#[inline]
pub fn limb_products(a: u32, b: u32) -> (u64, u64, u64, u64) {
    let (a0, a1, b0, b1) = decompose_32bit_limbs(a, b);
    let a0 = a0 as u64;
    let a1 = a1 as u64;
    let b0 = b0 as u64;
    let b1 = b1 as u64;

    let p0 = a0 * b0;
    let p1 = a0 * b1;
    let p2 = a1 * b0;
    let p3 = a1 * b1;
    (p0, p1, p2, p3)
}

#[inline]
pub fn mul_via_limbs(a: u32, b: u32) -> u64 {
    let (p0, p1, p2, p3) = limb_products(a, b);
    let mid = p1 + p2;
    p0 + (mid << 16) + (p3 << 32)
}

pub fn verify_limb_decomposition(a: u32, b: u32) -> crate::error::Result<()> {
    let direct = (a as u64) * (b as u64);
    let via_limbs = mul_via_limbs(a, b);

    if direct == via_limbs {
        Ok(())
    } else {
        Err(ZkvmError::LimbOverflow(direct))
    }
}
