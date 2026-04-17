use crate::limb16::Limb16;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Word(u32);

impl Word {
    pub const fn new(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub const fn bit(self, index: u8) -> bool {
        ((self.0 >> index) & 1) != 0
    }

    pub const fn bits(self, start: u8, width: u8) -> u32 {
        if width == 0 {
            0
        } else if width >= 32 {
            self.0 >> start
        } else {
            (self.0 >> start) & ((1u32 << width) - 1)
        }
    }

    pub const fn low_limb(self) -> Limb16 {
        Limb16::low(self.0)
    }

    pub const fn high_limb(self) -> Limb16 {
        Limb16::high(self.0)
    }

    pub const fn limbs(self) -> (Limb16, Limb16) {
        Limb16::split(self.0)
    }

    pub const fn from_limbs(low: Limb16, high: Limb16) -> Self {
        Self(Limb16::join(low, high))
    }

    pub const fn is_standard_32(self) -> bool {
        (self.0 & 0b11) == 0b11
    }

    pub const fn opcode(self) -> u8 {
        self.bits(0, 7) as u8
    }

    pub const fn rd(self) -> u8 {
        self.bits(7, 5) as u8
    }

    pub const fn funct3(self) -> u8 {
        self.bits(12, 3) as u8
    }

    pub const fn rs1(self) -> u8 {
        self.bits(15, 5) as u8
    }

    pub const fn rs2(self) -> u8 {
        self.bits(20, 5) as u8
    }

    pub const fn funct7(self) -> u8 {
        self.bits(25, 7) as u8
    }
}

impl From<u32> for Word {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<Word> for u32 {
    fn from(value: Word) -> Self {
        value.raw()
    }
}
