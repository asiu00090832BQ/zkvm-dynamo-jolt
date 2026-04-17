#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limb16 { pub lo: u16, pub hi: u16 }

impl Limb16 {
    pub const WIDTH: u32 = 16;
    #[inline] pub fn decompose(value: u32) -> Self { Self { lo: (value & 0xffff) as u16, hi: (value >> 16) as u16 } }
    #[inline] pub fn reconstruct(self) -> u32 { (self.lo as u32) | ((self.hi as u32) << Self::WIDTH) }
    #[inline] pub fn multiply(self, rhs: Self) -> WideMul16 {
        WideMul16 {
            lhs: self, rhs, 
            lo_lo: (self.lo as u64) * (rhs.lo as u64),
            lo_hi: (self.lo as u64) * (rhs.hi as u64),
            hi_lo: (self.hi as u64) * (rhs.lo as u64),
            hi_hi: (self.hi as u64) * (rhs.hi as u64),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WideMul16 { pub lhs: Limb16, pub rhs: Limb16, pub lo_lo: u64, pub lo_hi: u64, pub hi_lo: u64, pub hi_hi: u64 }

impl WideMul16 {
    #[inline] pub fn full64(self) -> u64 { self.lo_lo + ((self.lo_hi + self.hi_lo) << Limb16::WIDTH) + (self.hi_hi << (2 * Limb16::WIDTH)) }
    #[inline] pub fn low32(self) -> u32 { self.full64() as u32 }
    #[inline] pub fn high32(self) -> u32 { (self.full64() >> 32) as u32 }
}
