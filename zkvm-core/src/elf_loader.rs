use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElfError {
    InvalidMagic,
    UnsupportedElf,
    SectionNotFound,
}

impl fmt::Display for ElfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ElfError {}

#[derive(Debug, Clone)]
pub struct ElfSegment {
    pub vaddr: u64,
    pub data: Vec<u8>,
    pub mem_size: u64,
    pub file_size: u64,
}

#[derive(Debug, Clone)]
pub struct ElfImage {
    pub entry: u64,
    pub segments: Vec<ElfSegment>,
}

pub struct ElfLoader;

impl ElfLoader {
    pub fn load(_bytes: &[u8]) -> Result<ElfImage, ElfError> {
        Ok(ElfImage { entry: 0, segments: vec![] })
    }
}

pub fn load_elf(bytes: &[u8]) -> Result<ElfImage, ElfError> {
    ElfLoader::load(bytes)
}
