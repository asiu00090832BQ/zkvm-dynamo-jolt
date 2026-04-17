/// A single 16-bit limb used to decompose a 32-bit instruction word.
///
/// This crate uses two [`Limb16`] values to model the low and high halves of
/// an instruction word, matching the crate's Lemma 6.1.1 decomposition style.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Limb16(u16);

impl Limb16 {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u16 {
        self.0
    }

    pub const fn low(word: u32) -> Self {
        Self((word & 0xffff) as u16)
    }

    pub const fn high(word: u32) -> Self {
        Self(((word >> 16) & 0xffff) as u16)
    }

    pub const fn split(word: u32) -> (Self, Self) {
        (Self::low(word), Self::high(word))
    }

    pub const fn join(low: Self, high: Self) -> u32 {
        (low.0 as u32) | ((high.0 as u32) << 16)
    }
}

impl From<u16> for Limb16 {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<Limb16> for u16 {
    fn from(value: Limb16) -> Self {
        value.get()
    }
}
