use crate::elf_loader::{self, ElfProgram, ElfLoaderError};
pub type Program = ElfProgram;
pub struct Frontend;

impl Frontend {
    pub fn load_elf_bytes(&self, bytes: &[u8]) -> Result<Program, ElfLoaderError> {
        elf_loader::parse_elf(bytes)
    }

}
