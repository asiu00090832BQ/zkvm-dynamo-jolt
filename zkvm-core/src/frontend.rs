use crate::elf_loader::{load_elf, LoadedProgram};
use crate::ZkvmError;

#[derive(Debug, Clone)]
pub struct ElfProgram {
    pub entry: u32,
    pub memory: Vec<u8>,
}

impl ElfProgram {
    pub fn parse(bytes: &[u8]) -> Result<Self, ZkvmError> {
        let p = load_elf(bytes)?;
        Ok(Self {
            entry: p.entry,
            memory: p.memory,
        })
    }
}
