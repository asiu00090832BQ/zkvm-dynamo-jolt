use core::fmt;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfImage { pub entry: u32, pub memory: Vec<u8> }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfLoaderError { FileTooSmall, InvalidMagic, EntryOutOfBounds }

impl fmt::Display for ElfLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}

impl std::error::Error for ElfLoaderError {}

pub struct ElfLoader;

impl ElfLoader {
    pub fn load(bytes: &[u8]) -> Result<ElfImage, ElfLoaderError> {
        if bytes.len() < 52 { return Err(ElfLoaderError::FileTooSmall); }
        if &bytes[0..4] != b"\x7fELF" { return Err(ElfLoaderError::InvalidMagic); }
        let entry = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
        let memory = bytes.to_vec();
        Ok(ElfImage { entry, memory })
    }
}

pub fn load_elf(bytes: &[u8], memory_size: usize) -> Result<ElfImage, ElfLoaderError> {
    let mut image = ElfLoader::load(bytes)?;
    if image.entry as usize >= memory_size { return Err(ElfLoaderError::EntryOutOfBounds); }
    image.memory.resize(memory_size, 0);
    Ok(image)
}