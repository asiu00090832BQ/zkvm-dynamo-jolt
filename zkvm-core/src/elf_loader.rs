use std::fs;
use std::path::Path;
use crate::vm::ZkvmError;
use elf::ElfBytes;
use elf::endian::AnyEndian;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedElf {
    pub memory: Vec<u8>,
    pub entry: u64,
}

pub fn load_elf<P: AsRef<Path>>(path: P, mem_size: usize) -> Result<LoadedElf, ZkvmError> {
    let file_data = fs::read(path).map_err(|_| ZkvmError::InvalidElf)?;
    let elf = ElfBytes::<AnyEndian>::minimal_parse(&file_data).map_err(|_| ZkvmError::InvalidElf)?;

    let mut memory = vec![0u8; mem_size];

    if let Ok(Some(segments)) = elf.segments() {
        for phdr in segments {
            if phdr.p_type == elf::abi::PT_LOAD {
                let vaddr = phdr.p_vaddr as usize;
                let filesz = phdr.p_filesz as usize;
                let memsz = phdr.p_memsz as usize;
                let offset = phdr.p_offset as usize;

                if vaddr + memsz > mem_size {
                    return Err(ZkvmError::MemoryOutOfBounds { addr: vaddr as u32, len: memsz });
                }

                let data = &file_data[offset..offset + filesz];
                memory[vaddr..vaddr + filesz].copy_from_slice(data);
            }
        }
    }

    Ok(LoadedElf { memory, entry: elf.ehdr.e_entry })
}
