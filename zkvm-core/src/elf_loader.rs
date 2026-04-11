use std::fs;
use std::path::Path;
use crate::ZkvmError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedElf {
    pub memory: Vec<u8>,
    pub entry: u64,
}

pub fn load_elf<P: AsRef<Path>>(path: P, mem_size: usize) -> Result<LoadedElf, ZkvmError> {
    let data = fs::read(path)?;
    if data.len() < 64 { return Err(ZkvmError::InvalidElf); }
    if data[0] != 0x7f || data[1] != b'E' || data[2] != b'L' || data[3] != b'F' {
        return Err(ZkvmError::InvalidElf);
    }
    let mut memory = vec![0u8; mem_size];
    // ... LOAD logic here ...
    Ok(LoadedElf { memory, entry: 0x10000 })
}
