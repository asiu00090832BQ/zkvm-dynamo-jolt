pub fn decompose_u32_to_u16_limbs(value: u32) -> (u16, u16) {
    ((value & 0xffff) as u16, (value >> 16) as u16)
}

pub fn mul_u32_via_u16_limbs(lhs: u32, rhs: u32) -> u64 {
    let (a0, a1) = decompose_u32_to_u16_limbs(lhs);
    let (b0, b1) = decompose_u32_to_u16_limbs(rhs);

    let p0 = (a0 as u64) * (b0 as u64);
    let p1 = (a0 as u64) * (b1 as u64);
    let p2 = (a1 as u64) * (b0 as u64);
    let p3 = (a1 as u64) * (b1 as u64);

    p0 + ((p1 + p2) << 16) + (p3 << 32)
}

pub fn unsigned_mulh(lhs: u32, rhs: u32) -> u32 {
    (mul_u32_via_u16_limbs(lhs, rhs) >> 32) as u32
}

pub fn signed_mulh(lhs: i32, rhs: i32) -> u32 {
    (((lhs as i64) * (rhs as i64)) >> 32) as u32
}

pub fn signed_unsigned_mulh(lhs: i32, rhs: u32) -> u32 {
    (((lhs as i64) * (rhs as u64 as i64)) >> 32) as u32
}
