use crate::elf_loader::load_elf;
use crate::error::ZkvmError;

#[derive(Debug, Clone)]
pub struct ElfProgram {
    pub entry: u32,
    pub segments: Vec<crate::elf_loader::LoadSegment>,
}

impl ElfProgram {
    pub fn parse(bytes: &[u8]) -> Result<Self, ZkvmError> {
        let loaded = load_elf(bytes)
            .map_err(|e| ZkvmError::InvalidConfiguration(format!("{:?}", e)))?;
        Ok(ElfProgram {
            entry: loaded.entry as u32,
            segments: loaded.segments,
        })
    }
}
