use std::fs;
use std::path::Path;

use crate::vm::ZkvmError;

const PT_LOAD: u32 = 1;
const ELFCLASS32: u8 = 1;
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const EM_RISCV: u16 = 243;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedElf {
    pub memory: Vec<u8>,
    pub entry: u64,
}

pub fn load_elf(path: impl AsRef<Path>, mem_size: usize) -> Result<LoadedElf, ZkvmError> {
    let bytes = fs::read(path).map_err(|_| ZkvmError::InvalidElf)?;
    load_elf_bytes(&bytes, mem_size)
}

fn load_elf_bytes(bytes: &[u8], mem_size: usize) -> Result<LoadedElf, ZkvmError> {
    if bytes.len() < 20 {
        return Err(ZkvmError::InvalidElf);
    }

    if &bytes[0..4] != b"\x7fELF" {
        return Err(ZkvmError::InvalidElf);
    }

    if bytes[5] != ELFDATA2LSB {
        return Err(ZkvmError::InvalidElf);
    }

    let machine = read_u16_le(bytes, 18).ok_or(ZkvmError::InvalidElf)?;
    if machine != EM_RISCV {
        return Err(ZkvmError::InvalidElf);
    }

    match bytes[4] {
        ELFCLASS32 => load_elf32(bytes, mem_size),
        ELFCLASS64 => load_elf64(bytes, mem_size),
        _ => Err(ZkvmError::InvalidElf),
    }
}

fn load_elf32(bytes: &[u8], mem_size: usize) -> Result<LoadedElf, ZkvmError> {
    if bytes.len() < 52 {
        return Err(ZkvmError::InvalidElf);
    }

    let entry = u64::from(read_u32_le(bytes, 24).ok_or(ZkvmError::InvalidElf)?);
    let phoff = read_u32_le(bytes, 28).ok_or(ZkvmError::InvalidElf)? as usize;
    let phentsize = read_u16_le(bytes, 42).ok_or(ZkvmError::InvalidElf)? as usize;
    let phnum = read_u16_le(bytes, 44).ok_or(ZkvmError::InvalidElf)? as usize;

    if phentsize < 32 {
        return Err(ZkvmError::InvalidElf);
    }

    let mut memory = vec![0u8; mem_size];

    for index in 0..phnum {
        let ph_start = checked_offset(phoff, index, phentsize)?;
        let p_type = read_u32_le(bytes, add_offset(ph_start, 0)?).ok_or(ZkvmError::InvalidElf)?;
        if p_type != PT_LOAD {
            continue;
        }

        let p_offset = read_u32_le(bytes, add_offset(ph_start, 4)?).ok_or(ZkvmError::InvalidElf)? as usize;
        let p_vaddr = read_u32_le(bytes, add_offset(ph_start, 8)?).ok_or(ZkvmError::InvalidElf)? as usize;
        let p_filesz = read_u32_le(bytes, add_offset(ph_start, 16)?).ok_or(ZkvmError::InvalidElf)? as usize;
        let p_memsz = read_u32_le(bytes, add_offset(ph_start, 20)?).ok_or(ZkvmError::InvalidElf)? as usize;

        map_segment(bytes, &mut memory, p_offset, p_vaddr, p_filesz, p_memsz)?;
    }

    validate_entry(entry, mem_size)?;

    Ok(LoadedElf { memory, entry })
}

fn load_elf64(bytes: &[u8], mem_size: usize) -> Result<LoadedElf, ZkvmError> {
    if bytes.len() < 64 {
        return Err(ZkvmError::InvalidElf);
    }

    let entry = read_u64_le(bytes, 24).ok_or(ZkvmError::InvalidElf)?;
    let phoff = usize::try_from(read_u64_le(bytes, 32).ok_or(ZkvmError::InvalidElf)?)
        .map_err(|_| ZkvmError::InvalidElf)?;
    let phentsize = read_u16_le(bytes, 54).ok_or(ZkvmError::InvalidElf)? as usize;
    let phnum = read_u16_le(bytes, 56).ok_or(ZkvmError::InvalidElf)? as usize;

    if phentsize < 56 {
        return Err(ZkvmError::InvalidElf);
    }

    let mut memory = vec![0u8; mem_size];

    for index in 0..phnum {
        let ph_start = checked_offset(phoff, index, phentsize)?;
        let p_type = read_u32_le(bytes, add_offset(ph_start, 0)?).ok_or(ZkvmError::InvalidElf)?;
        if p_type != PT_LOAD {
            continue;
        }

        let p_offset =
            usize::try_from(read_u64_le(bytes, add_offset(ph_start, 8)?).ok_or(ZkvmError::InvalidElf)?)
                .map_err(|_| ZkvmError::InvalidElf)?;
        let p_vaddr =
            usize::try_from(read_u64_le(bytes, add_offset(ph_start, 16)?).ok_or(ZkvmError::InvalidElf)?)
                .map_err(|_| ZkvmError::InvalidElf)?;
        let p_filesz =
            usize::try_from(read_u64_le(bytes, add_offset(ph_start, 32)?).ok_or(ZkvmError::InvalidElf)?)
                .map_err(|_| ZkvmError::InvalidElf)?;
        let p_memsz =
            usize::try_from(read_u64_le(bytes, add_offset(ph_start, 40)?).ok_or(ZkvmError::InvalidElf)?)
                .map_err(|_| ZkvmError::InvalidElf)?;

        map_segment(bytes, &mut memory, p_offset, p_vaddr, p_filesz, p_memsz)?;
    }

    validate_entry(entry, mem_size)?;

    Ok(LoadedElf { memory, entry })
}

fn checked_offset(base: usize, index: usize, stride: usize) -> Result<usize, ZkvmError> {
    let start = base
        .checked_add(index.checked_mul(stride).ok_or(ZkvmError::InvalidElf)?)
        .ok_or(ZkvmError::InvalidElf)?;
    start.checked_add(stride).ok_or(ZkvmError::InvalidElf)?;
    Ok(start)
}

fn add_offset(base: usize, offset: usize) -> Result<usize, ZkvmError> {
    base.checked_add(offset).ok_or(ZkvmError::InvalidElf)
}

fn map_segment(
    bytes: &[u8],
    memory: &mut [u8],
    file_offset: usize,
    vaddr: usize,
    filesz: usize,
    memsz: usize,
) -> Result<(), ZkvmError> {
    if filesz > memsz {
        return Err(ZkvmError::InvalidElf);
    }

    let file_end = file_offset
        .checked_add(filesz)
        .ok_or(ZkvmError::InvalidElf)?;
    if file_end > bytes.len() {
        return Err(ZkvmError::InvalidElf);
    }

    let mem_end = vaddr.checked_add(memsz).ok_or(ZkvmError::InvalidElf)?;
    if mem_end > memory.len() {
        return Err(ZkvmError::MemoryOutOfBounds {
            addr: addr32(vaddr),
            size: memsz,
        });
    }

    if filesz > 0 {
        memory[vaddr..vaddr + filesz].copy_from_slice(&bytes[file_offset..file_end]);
    }

    if memsz > filesz {
        memory[vaddr + filesz..mem_end].fill(0);
    }

    Ok(())
}

fn validate_entry(entry: u64, mem_size: usize) -> Result<(), ZkvmError> {
    let start = usize::try_from(entry).map_err(|_| ZkvmError::InvalidElf)?;
    let end = start.checked_add(4).ok_or(ZkvmError::InvalidElf)?;
    if end > mem_size {
        return Err(ZkvmError::InvalidElf);
    }
    Ok(())
}

fn addr32(addr: usize) -> u32 {
    u32::try_from(addr).unwrap_or(u32::MAX)
}

fn read_u16_le(bytes: &[u8], offset: usize) -> Option<u16> {
    let end = offset.checked_add(2)?;
    let slice = bytes.get(offset..end)?;
    Some(u16::from_le_bytes([slice[0], slice[1]]))
}

fn read_u32_le(bytes: &[u8], offset: usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let slice = bytes.get(offset..end)?;
    Some(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

fn read_u64_le(bytes: &[u8], offset: usize) -> Option<u64> {
    let end = offset.checked_add(8)?;
    let slice = bytes.get(offset..end)?;
    Some(u64::from_le_bytes([
        slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
    ]))
}