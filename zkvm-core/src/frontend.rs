use crate::error::ZkvmError;
use crate::elf_loader::{load_elf, LoadSegment};

#[derive(Debug, Clone)]
pub struct ElfProgram {
    pub entry: u32,
    pub segments: Vec<LoadSegment>,
}

impl ElfProgram {
    pub fn parse(bytes: &[u8]) -> Result<Self, ZkvmError> {
        let loaded = load_elf(bytes).map_err(|e| ZkvmError::Elf(format!("{:?}", e)))?;
        Ok(Self {
            entry: loaded.entry,
            segments: loaded.segments,
        })
    }
}
