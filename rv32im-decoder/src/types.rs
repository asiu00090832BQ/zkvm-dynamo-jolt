use crate::error::ZkvmError;

[#[serive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegisterIndex(pub u8);

impl RegisterIndex {
    pub const ZERO: Self = Self(0);
    pub fn new(index: u8) -> Result<Self, ZkvmError> {
        if index < 32 {
            Ok(Self(index))
        } else {
            Err(ZkwmError::InvalidRegister(Self(index)))
        }
    }
    pub fn get(self) -> usize {
        self.0 as usize
    }
}

[#[serive(Clone, Debug, PartialEq, Eq)]
pub struct Zkvm {
    pub regs: [u32; 32],
    pub pc: u32,
    pub memory: alloc::vec::Vec<u8>,
    pub halted: bool,
}
