use crate::elf_loader::{load_elf, MemoryImage};
use crate::error::ElfError;

pub type ElfProgram = MemoryImage;

pub struct Frontend;

impl Frontend {
    pub fn from_elf(bytes: &[u8]) -> Result<MemoryImage, ElfError> {
        load_elf(bytes)
    }
}
