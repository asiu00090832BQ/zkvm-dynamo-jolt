#[inline] fn mul_u32_wide_limbs(lhs: u32, rhs: u32) -> u64 { let a0 = (lhs & 0xffff) as u64; let a1 = (lhs >> 16) as u64; let b0 = (rhs & 0xffff) as u64; let b1 = (rhs >> 16) as u64; let p0 = a0 * b0; let p1 = a0 * b1; let p2 = a1 * b0; let p3 = a1 * b1; p0 + ((p1 + p2) << 16) + (p3 << 32) }
#[inline] fn is_negative(bits: u32) -> bool { (bits & 0x8000_0000) != 0 }
#[inline] fn abs_i32_bits(bits: u32) -> u32 { if is_negative(bits) { (!bits).wrapping_add(1) } else { bits } }
#[inline] fn signed_high_from_magnitude(magnitude: u64, negative: bool) -> u32 { let product = if negative { (!magnitude).wrapping_add(1) } else { magnitude }; (product >> 32) as u32 }
#[inline] pub fn mul(lhs: u32, rhs: u32) -> u32 { mul_u32_wide_limbs(lhs, rhs) as u32 }
#[inline] pub fn mulh(lhs: u32, rhs: u32) -> u32 { let magnitude = mul_u32_wide_limbs(abs_i32_bits(lhs), abs_i32_bits(rhs)); signed_high_from_magnitude(magnitude, is_negative(lhs) ^ is_negative(rhs)) }
#[inline] pub fn mulhsu(lhs: u32, rhs: u32) -> u32 { let magnitude = mul_u32_wide_limbs(abs_i32_bits(lhs), rhs); signed_high_from_magnitude(magnitude, is_negative(lhs)) }
#[inline] pub fn mulhu(lhs: u32, rhs: u32) -> u32 { (mul_u32_wide_limbs(lhs, rhs) >> 32) as u32 }
#[inline] pub fn div(lhs: u32, rhs: u32) -> u32 { if rhs == 0 { return u32::MAX; } let dividend = lhs as i32; let divisor = rhs as i32; if dividend == i32::MIN && divisor == -1 { lhs } else { (dividend / divisor) as u32 } }
#[inline] pub fn divu(lhs: u32, rhs: u32) -> u32 { if rhs == 0 { u32::MAX } else { lhs / rhs } }
#[inline] pub fn rem(lhs: u32, rhs: u32) -> u32 { if rhs == 0 { return lhs; } let dividend = lhs as i32; let divisor = rhs as i32; if dividend == i32::MIN && divisor == -1 { 0 } else { (dividend % divisor) as u32 } }
#[inline] pub fn remu(lhs: u32, rhs: u32) -> u32 { if rhs == 0 { lhs } else { lhs % rhs } }
