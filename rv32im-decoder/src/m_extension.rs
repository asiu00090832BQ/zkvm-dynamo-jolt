#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limb16Decomposition {
    pub low: u16,
    pub high: u16,
}

pub fn decompose_limb16(value: u32) -> Limb16Decomposition {
    Limb16Decomposition {
        low: value as u16,
        high: (value >> 16) as u16,
    }
}

pub fn combine_limb16(parts: Limb16Decomposition) -> u32 {
    (parts.low as u32) | ((parts.high as u32) << 16)
}

pub fn mul_u32_wide(lhs: u32, rhs: u32) -> u64 {
    let lhs_parts = decompose_limb16(lhs);
    let rhs_parts = decompose_limb16(rhs);

    let p0 = lhs_parts.low as u64 * rhs_parts.low as u64;
    let p1 = lhs_parts.low as u64 * rhs_parts.high as u64;
    let p2 = lhs_parts.high as u64 * rhs_parts.low as u64;
    let p3 = lhs_parts.high as u64 * rhs_parts.high as u64;

    p0.wrapping_add((p1.wrapping_add(p2)) << 16)
        .wrapping_add(p3 << 32)
}

pub fn mul_i32_wide(lhs: i32, rhs: i32) -> i64 {
    let lhs_neg = lhs < 0;
    let rhs_neg = rhs < 0;
    let magnitude = mul_u32_wide(abs_i32_bits(lhs), abs_i32_bits(rhs));
    if lhs_neg ^ rhs_neg {
        0u64.wrapping_sub(magnitude) as i64
    } else {
        magnitude as i64
    }
}

pub fn mul_i32_u32_wide(lhs: i32, rhs: u32) -> i64 {
    let magnitude = mul_u32_wide(abs_i32_bits(lhs), rhs);
    if lhs < 0 {
        0u64.wrapping_sub(magnitude) as i64
    } else {
        magnitude as i64
    }
}

fn abs_i32_bits(value: i32) -> u32 {
    if value < 0 {
        0u32.wrapping_sub(value as u32)
    } else {
        value as u32
    }
}
