use std::fs;
use std::path::Path;
use crate::vm::ZkvmError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedElf {
    pub memory: Vec<u8>,
    pub entry: u64,
}

pub fn load_elf<P: AsRef<Path>>(path: P, mem_size: usize) -> Result<LoadedElf, ZkvmError> {
    let data = fs::read(path).map_err(|_| ZkvmError::InvalidElf)?;
    if data.len() < 52 || &data[0..4] != b"\x7fELF" {
        return Err(ZkvmError::InvalidElf);
    }
    let mut memory = vec![0u8; mem_size];
    let len = data.len().min(mem_size);
    memory[..len].copy_from_slice(&data[..len]);
    Ok(LoadedElf { memory, entry: 0x10000 })
}
