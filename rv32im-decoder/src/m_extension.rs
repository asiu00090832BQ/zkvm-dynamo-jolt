// CAB8453E / Lemma 6.1.1: RV32M arithmetic via 16-bit limb decomposition.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Limb16(pub u16);

impl Limb16 {
    pub const BITS: u32 = 16;
    pub const MASK: u32 = 0xFFFF;

    #[inline]
    pub const fn low(word: u32) -> Self {
        Self((word & Self::MASK) as u16)
    }

    #[inline]
    pub const fn high(word: u32) -> Self {
        Self(((word >> Self::BITS) & Self::MASK) as u16)
    }

    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0 as u32
    }
}

#[inline]
pub const fn decompose_u32(word: u32) -> (Limb16, Limb16) {
    (Limb16::low(word), Limb16::high(word))
}

#[inline]
pub const fn recompose_u32(low: Limb16, high: Limb16) -> u32 {
    low.as_u32() | (high.as_u32() << Limb16::BITS)
}

#[inline]
fn twos_complement_64(value: u64) -> u64 {
    (!value).wrapping_add(1)
}

#[inline]
fn abs_i32_bits(value: i32) -> u32 {
    if value < 0 {
        (value as u32).wrapping_neg()
    } else {
        value as u32
    }
}

#[inline]
pub fn wide_mul_u32(lhs: u32, rhs: u32) -> u64 {
    let (a0, a1) = decompose_u32(lhs);
    let (b0, b1) = decompose_u32(rhs);

    let p00 = (a0.as_u32() * b0.as_u32()) as u64;
    let p01 = (a0.as_u32() * b1.as_u32()) as u64;
    let p10 = (a1.as_u32() * b0.as_u32()) as u64;
    let p11 = (a1.as_u32() * b1.as_u32()) as u64;

    let limb0 = p00 & 0xFFFF;
    let carry0 = p00 >> 16;

    let middle = carry0 + p01 + p10;
    let limb1 = middle & 0xFFFF;
    let carry1 = middle >> 16;

    let upper = p11 + carry1;

    limb0 | (limb1 << 16) | (upper << 32)
}

#[inline]
fn wide_mul_i32_bits(lhs: i32, rhs: i32) -> u64 {
    let negative = (lhs < 0) ^ (rhs < 0);
    let magnitude = wide_mul_u32(abs_i32_bits(lhs), abs_i32_bits(rhs));
    if negative {
        twos_complement_64(magnitude)
    } else {
        magnitude
    }
}

#[inline]
fn wide_mul_i32_u32_bits(lhs: i32, rhs: u32) -> u64 {
    let negative = lhs < 0;
    let magnitude = wide_mul_u32(abs_i32_bits(lhs), rhs);
    if negative {
        twos_complement_64(magnitude)
    } else {
        magnitude
    }
}

#[inline]
pub fn mul(lhs: u32, rhs: u32) -> u32 {
    wide_mul_u32(lhs, rhs) as u32
}

#[inline]
pub fn mulh(lhs: u32, rhs: u32) -> u32 {
    (wide_mul_i32_bits(lhs as i32, rhs as i32) >> 32) as u32
}

#[inline]
pub fn mulhsu(lhs: u32, rhs: u32) -> u32 {
    (wide_mul_i32_u32_bits(lhs as i32, rhs) >> 32) as u32
}

#[inline]
pub fn mulhu(lhs: u32, rhs: u32) -> u32 {
    (wide_mul_u32(lhs, rhs) >> 32) as u32
}

#[inline]
pub fn div(lhs: u32, rhs: u32) -> u32 {
    let dividend = lhs as i32;
    let divisor = rhs as i32;

    if divisor == 0 {
        return u32::MAX;
    }

    if dividend == i32::MIN && divisor == -1 {
        return dividend as u32;
    }

    (dividend / divisor) as u32
}

#[inline]
pub fn divu(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        u32::MAX
    } else {
        lhs / rhs
    }
}

#[inline]
pub fn rem(lhs: u32, rhs: u32) -> u32 {
    let dividend = lhs as i32;
    let divisor = rhs as i32;

    if divisor == 0 {
        return dividend as u32;
    }

    if dividend == i32::MIN && divisor == -1 {
        return 0;
    }

    (dividend % divisor) as u32
}

#[inline]
pub fn remu(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        lhs
    } else {
        lhs % rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompose_and_recompose_round_trip() {
        let value = 0xABCD_1234;
        let (low, high) = decompose_u32(value);
        assert_eq!(low.0, 0x1234);
        assert_eq!(high.0, 0xABCD);
        assert_eq!(recompose_u32(low, high), value);
    }

    #[test]
    fn limb16_multiplication_matches_native_results() {
        let samples = [
            0u32,
            1,
            2,
            3,
            0x7FFF_FFFF,
            0x8000_0000,
            0xFFFF_FFFF,
            0x1234_5678,
            0x8765_4321,
        ];

        for &lhs in &samples {
            for &rhs in &samples {
                assert_eq!(mul(lhs, rhs), lhs.wrapping_mul(rhs));
                assert_eq!(mulhu(lhs, rhs), (((lhs as u64) * (rhs as u64)) >> 32) as u32);
                assert_eq!(
                    mulh(lhs, rhs),
                    (((lhs as i32 as i64) * (rhs as i32 as i64)) >> 32) as u32
                );
                assert_eq!(
                    mulhsu(lhs, rhs),
                    (((lhs as i32 as i64) * (rhs as u64 as i64)) >> 32) as u32
                );
            }
        }
    }

    #[test]
    fn division_and_remainder_follow_riscv_rules() {
        assert_eq!(div(123, 0), u32::MAX);
        assert_eq!(divu(123, 0), u32::MAX);
        assert_eq!(rem(123, 0), 123);
        assert_eq!(remu(123, 0), 123);

        assert_eq!(div(i32::MIN as u32, (-1i32) as u32), i32::MIN as u32);
        assert_eq!(rem(i32::MIN as u32, (-1i32) as u32), 0);

        assert_eq!(div((-7i32) as u32, 3), (-2i32) as u32);
        assert_eq!(rem((-7i32) as u32, 3), (-1i32) as u32);
        assert_eq!(divu(7, 3), 2);
        assert_eq!(remu(7, 3), 1);
    }
}
