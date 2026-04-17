/// Unsigned 16-bit limb decomposition for a 32-bit word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limbs16 {
    pub low: u16,
    pub high: u16,
}

impl Limbs16 {
    #[inline]
    pub fn recompose(self) -> u32 {
        ((self.high as u32) << 16) | self.low as u32
    }
}

/// Witness for Lemma 6.1.1:
///
/// A = a1 * 2^16 + a0
/// B = b1 * 2^16 + b0
/// P = (a1*b1)*2^32 + (a1*b0 + a0*b1)*2^16 + a0*b0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MulWitness {
    pub a: u32,
    pub b: u32,
    pub a0: u32,
    pub a1: u32,
    pub b0: u32,
    pub b1: u32,
    pub p0: u64,
    pub p1: u64,
    pub p2: u64,
    pub product: u64,
}

impl MulWitness {
    #[inline]
    pub fn new(a: u32, b: u32) -> Self {
        let a_limbs = decompose_u32(a);
        let b_limbs = decompose_u32(b);

        let a0 = a_limbs.low as u32;
        let a1 = a_limbs.high as u32;
        let b0 = b_limbs.low as u32;
        let b1 = b_limbs.high as u32;

        let p0 = (a0 as u64) * (b0 as u64);
        let p1 = (((a1 as u64) * (b0 as u64)) + ((a0 as u64) * (b1 as u64))) << 16;
        let p2 = ((a1 as u64) * (b1 as u64)) << 32;
        let product = p2 + p1 + p0;

        Self {
            a,
            b,
            a0,
            a1,
            b0,
            b1,
            p0,
            p1,
            p2,
            product,
        }
    }

    #[inline]
    pub fn low(self) -> u32 {
        self.product as u32
    }

    #[inline]
    pub fn high(self) -> u32 {
        (self.product >> 32) as u32
    }
}

#[inline]
pub fn decompose_u32(value: u32) -> Limbs16 {
    Limbs16 {
        low: (value & 0xffff) as u16,
        high: (value >> 16) as u16,
    }
}

#[inline]
pub fn mul_witness(a: u32, b: u32) -> MulWitness {
    MulWitness::new(a, b)
}

#[inline]
pub fn verify_mul_witness(witness: &MulWitness) -> bool {
    let a_ok = witness.a == ((witness.a1 << 16) | witness.a0);
    let b_ok = witness.b == ((witness.b1 << 16) | witness.b0);
    let limb_bounds_ok = witness.a0 <= 0xffff
        && witness.a1 <= 0xffff
        && witness.b0 <= 0xffff
        && witness.b1 <= 0xffff;

    let p0_ok = witness.p0 == (witness.a0 as u64) * (witness.b0 as u64);
    let p1_ok = witness.p1
        == ((((witness.a1 as u64) * (witness.b0 as u64))
            + ((witness.a0 as u64) * (witness.b1 as u64)))
            << 16);
    let p2_ok = witness.p2 == (((witness.a1 as u64) * (witness.b1 as u64)) << 32);
    let recomposed = witness.p2 + witness.p1 + witness.p0;

    a_ok
        && b_ok
        && limb_bounds_ok
        && p0_ok
        && p1_ok
        && p2_ok
        && witness.product == recomposed
        && witness.product == (witness.a as u64) * (witness.b as u64)
}

#[inline]
fn signed_high_word(product: i64) -> u32 {
    ((product as u64) >> 32) as u32
}

#[inline]
pub fn mul(a: u32, b: u32) -> u32 {
    mul_witness(a, b).low()
}

#[inline]
pub fn mulhu(a: u32, b: u32) -> u32 {
    mul_witness(a, b).high()
}

#[inline]
pub fn mulh(a: u32, b: u32) -> u32 {
    signed_high_word((a as i32 as i64) * (b as i32 as i64))
}

#[inline]
pub fn mulhsu(a: u32, b: u32) -> u32 {
    signed_high_word((a as i32 as i64) * (b as i64))
}

#[inline]
pub fn div(a: u32, b: u32) -> u32 {
    let lhs = a as i32;
    let rhs = b as i32;

    if b == 0 {
        u32::MAX
    } else if lhs == i32::MIN && rhs == -1 {
        lhs as u32
    } else {
        (lhs / rhs) as u32
    }
}

#[inline]
pub fn divu(a: u32, b: u32) -> u32 {
    if b == 0 {
        u32::MAX
    } else {
        a / b
    }
}

#[inline]
pub fn rem(a: u32, b: u32) -> u32 {
    let lhs = a as i32;
    let rhs = b as i32;

    if b == 0 {
        a
    } else if lhs == i32::MIN && rhs == -1 {
        0
    } else {
        (lhs % rhs) as u32
    }
}

#[inline]
pub fn remu(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        a % b
    }
}
