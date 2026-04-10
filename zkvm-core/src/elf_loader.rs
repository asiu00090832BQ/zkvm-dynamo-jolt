use core::fmt;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfImage { pub entry: u32, pub memory: Vec<u8> }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfLoaderError { FileTooSmall, InvalidMagic, UnsupportedClass, UnsupportedEndian, UnsupportedVersion, UnsupportedType, UnsupportedMachine, AddressOverflow, EntryOutOfBounds }

impl fmt::Display for ElfLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}

impl std::error::Error for ElfLoaderError {}

pub fn load_elf(bytes: &[u8], memory_size: usize) -> Result<ElfImage, ElfLoaderError> {
    if bytes.len() < 52 { return Err(ElfLoaderError::FileTooSmall); }
    if &bytes[0..4] != b"\x7fELF" { return Err(ElfLoaderError::InvalidMagic); }
    let entry = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
    let mut memory = vec![0u8; memory_size];
    let len = bytes.len().min(memory_size);
    memory[0..len].copy_from_slice(&bytes[0..len]);
    if entry as usize >= memory_size { return Err(ElfLoaderError::EntryOutOfBounds)); }
    Ok(ElfImage { entry, memory })
}
