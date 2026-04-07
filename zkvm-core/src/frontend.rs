use crate::elf_loader::{self, ElfProgram};
pub type Program = ElfProgram;
pub struct Frontend;
impl Frontend { pub fn load_elf_bytes(&self, bytes: &[u8]) -> Result<Program, String> { elf_loader::parse_elf(bytes) } }
