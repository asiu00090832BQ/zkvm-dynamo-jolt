use crate::elf_loader::load_elf;
use crate::error::Result;

pub struct Frontend;
impl Frontend {
    pub fn parse(bytes: &[u8]) -> Result<crate::elf_loader::LoadedElf> { load_elf(bytes) }
}
