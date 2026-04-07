use crate::error::{Result, ZkvmError};
use goblin::elf::program_header::PT_LOAD;
use goblin::elf::{header::EM_RISCV, Elf};

#[derive(Debug, Clone)]
pub struct LoadSegment {
    pub vaddr: u64,
    pub data: Vec<u8>,
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

#[derive(Debug, Clone)]
pub struct LoadedElf {
    pub entry: u64,
    pub segments: Vec<LoadSegment>,
}

pub fn load_elf(bytes: &[u8]) -> Result<LoadedElf> {
    let elf = Elf::parse(bytes).map_err(|e| ZkvmError::InvalidElf(format!("failed to parse ELF: {e}")))?;
    if elf.header.e_machine != EM_RISCV {
        return Err(ZkvmError::InvalidElf(format!("unsupported machine {:#x}", elf.header.e_machine)));
    }
    let mut segments = Vec::new();
    for ph in &elf.program_headers {
        if ph.p_type != PT_LOAD { continue; }
        let offset = ph.p_offset as usize;
        let file_size = ph.p_filesz as usize;
        let mem_size = ph.p_memsz as usize;
        let end = offset.checked_add(file_size).ok_or_else(|| ZkvmError::ElfLoad("offset overflow".to_string()))?;
        if end > bytes.len() { return Err(ZkvmError::ElfLoad("segment out of bounds".to_string())); }
        let mut data = bytes[offset..end].to_vec();
        if mem_size > file_size {
            data.resize(mem_size, 0);
        }
        segments.push(LoadSegment { vaddr: ph.p_vaddr, data, read: ph.is_read(), write: ph.is_write(), execute: ph.is_executable() });
    }
    Ok(LoadedElf { entry: elf.entry, segments })
}
