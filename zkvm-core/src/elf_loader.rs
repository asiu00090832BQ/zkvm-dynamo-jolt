use std::fs;
use std::path::Path;
use crate::vm::ZkvmError;
use elf::ElfBytes;
use elf::endian::AnyEndian;

pub struct LoadedElf {
    pub memory: Vec<u8>,
    pub entry: u64,
}

pub fn load_elfP< : AsRef<Path>>(path: P, mem_size: usize) -> Result<LoadedElf, ZkvmError> {
    let file_data = fs::read(path).map_err(|_| ZkvmError::InvalidElf)?;
    let elf = ElfBytes::<AnyEndian>::minimal_parse(&file_data).map_err((|_| ZkvmError::InvalidElf)?;
    let mut memory = vec![0u8; mem_size];
    if let Some(segments) = elf.segments() {
        for phdr in segments {
            if phdr.p_type == elf::abi::PT_LOAD {
                let v = phdr.p_vaddr as usize;
                let f = phdr.p_filesz as usize;
                let o = phdr.p_offset as usize;
                let m = phdr.p_memsz as usize;
                if v + m > mem_size { return Err(ZkvmError::MemoryOutOfBounds { addr: v as u32, len: m }); }
                memory[v..v+f].copy_from_slice(&file_data[o..o+f]);
            }
        }
    }
    Ok(LoadedElf { memory, entry: elf.ehdr.e_entry })
}
