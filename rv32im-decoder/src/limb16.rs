/// Lemma 6.1.1 (16-bit limb decomposition):
/// every `u32` can be written uniquely as `lo + (hi << 16)`
/// with `lo, hi ∈ [0, 2^16)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Limb16 {
    pub lo: u16,
    pub hi: u16,
}

impl Limb16 {
    pub const fn new(lo: u16, hi: u16) -> Self {
        Self { lo, hi }
    }

    pub const fn from_u32(value: u32) -> Self {
        Self {
            lo: (value & 0xffff) as u16,
            hi: (value >> 16) as u16,
        }
    }

    pub const fn to_u32(self) -> u32 {
        (self.lo as u32) | ((self.hi as u32) << 16)
    }

    pub const fn widening_mul(self, rhs: Self) -> u64 {
        let a0 = self.lo as u64;
        let a1 = self.hi as u64;
        let b0 = rhs.lo as u64;
        let b1 = rhs.hi as u64;

        let cross = a0 * b1 + a1 * b0;
        a0 * b0 + (cross << 16) + ((a1 * b1) << 32)
    }
}
