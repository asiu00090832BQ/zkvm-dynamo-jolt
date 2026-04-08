
use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfSegment {
    pub vaddr: u32,
    pub data: Vec<u8>,
    pub mem_size: u32,
    pub flags: u32,
    pub align: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfImage {
    pub entry: u32,
    pub segments: Vec<ElfSegment>,
    pub memory_size: usize,
}

pub fn load_elf(bytes: &[u8], memory_size: usize) -> Result<ElfImage, ElfLoaderError> {
    Ok(ElfImage { entry: 0, segments: vec![], memory_size })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElfLoaderError {
    FileTooSmall,
    BadMagic,
}
