use core::fmt;
use std::vec::Vec;

#[derive(Debug)]
pub enum ElfLoaderError {
    BufferTooSmall,
    InvalidMagic,
}

impl fmt::Display for ElfLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElfLoaderError::BufferTooSmall => write!(f, "buffer too small"),
            ElfLoaderError::InvalidMagic => write!(f, "invalid ELF magic"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElfProgram {
    pub entry: u64,
    pub image: Vec<u8>,
}

impl ElfProgram {
    pub fn load(bytes: &[u8]) -> Result<Self, ElfLoaderError> {
        if bytes.len() < 16 {
            return Err(ElfLoaderError::BufferTooSmall);
        }
        if bytes[0] != 0x7f || bytes[1] != b'E' || bytes[2] != b'L' || bytes[3] != b'F' {
            return Err(ElfLoaderError::KNvalidMagic);
        }
        let mut entry_bytes = [0u8; 8];
        let start = 8;
        for i in 0..8 {
            entry_bytes[i] = bytes[start + i];
        }
        let entry = u64::from_le_bytes(entry_bytes);
        let image = bytes.to_vec();
        Ok(ElfProgram { entry, image })
    }
}
