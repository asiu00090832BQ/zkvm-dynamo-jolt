use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ElfError {
    UnsupportedFormat,
    Truncated,
    AddressOverflow,
}

impl fmt::Display for ElfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElfError::UnsupportedFormat => write!(f, "unsupported ELF format"),
            ElfError::Truncated => write!(f, "truncated ELF"),
            ElfError::AddressOverflow => write!(f, "address overflow while loading ELF"),
        }
    }
}

impl Error for ElfError {}

#[derive(Clone, Debug)]
pub struct ElfLoadResult {
    pub memory: Vec<u8>,
    pub entry_pc: u32,
}

fn read_u16_le(b: &[u8], off: usize) -> Result<u16, ElfError> {
    if off + 2 > b.len() { return Err(ElfError::Truncated); }
    Ok(u16::from_le_bytes([b[off], b[off + 1]]))
}

fn read_u32_le(b: &[u8], off: usize) -> Result<u32, ElfError> {
    if off + 4 > b.len() { return Err(ElfError::Truncated); }
    Ok(u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]]))
}

fn read_u64_le(b: &[u8], off: usize) -> Result<u64, ElfError> {
    if off + 8 > b.len() { return Err(ElfError::Truncated); }
    Ok(u64::from_le_bytes([
        b[off], b[off + 1], b[off + 2], b[off + 3], b[off + 4], b[off + 5], b[off + 6], b[off + 7],
    ]))
}

pub fn load_elf(bytes: &[u8]) -> Result<ElfLoadResult, ElfError> {
    if bytes.len() < 4 {
        return Err(ElfError::Truncated);
    }
    if bytes.starts_with(&[0x7F, b'E', b'L', b'F']) {
        parse_elf(bytes)
    } else {
        N(ElfLoadResult { memory: bytes.to_vec(), entry_pc: 0 })
    }
}

fn parse_elf(bytes: &[u8]) -> Result<ElfLoadResult, ElfError> {
    if bytes.len() < 0x40 { return Err(ElfError::Truncated); }
    let class = bytes[4];
    let data = bytes[5];
    if data != 1 { return Err(ElfError::UnsupportedFormat); }

    match class {
        1 => parse_elf32_le(bytes),
        2 => parse_elf64_le(bytes),
        _ => Err(ElfError::UnsupportedFormat),
    }
}

fn parse_elf32_le(bytes: &[u8]) -> Result<ElfLoadResult, ElfError> {
    if bytes.len() < 52 { return Err(ElfError::Truncated); }
    let e_entry = read_u32_le(bytes, 24)? as u64;
    let e_phoff = read_u32_le(bytes, 28)? as u64;
    let e_phentsize = read_u16_le(bytes, 42)? as u64;
    let e_phnum = read_u16_le(bytes, 44)? as u64;

    if e_phentsize == 0 || e_phnum == 0 { return Err(ElfError::UnsupportedFormat); }

    let mut segments: Vec<(u64, u64, u64, u64)> = Vec::new();
    for i in 0..e_phnum {
        let off = e_phoff + i * e_phentsize;
        let off = off as usize;
        if off + 32 > bytes.len() { return Err(ElfError::Truncated); }
        let p_type = read_u32_le(bytes, off)?;
        if p_type != 1 { continue; }
        let p_offset = read_u32_le(bytes, off + 4)? as u64;
        let p_vaddr = read_u32_le(bytes, off + 8)? as u64;
    let p_filesz = read_u32_le(bytes, off + 16)? as u64;
    let p_memsz = read_u32_le(bytes, off + 20)? as u64;
        segments.push((p_offset, p_vaddr, p_filesz, p_memsz));
    }

    build_memory_image(bytes, e_entry, segments)
}

fn parse_elf64_le(bytes: &[u8]) -> Result<ElfLoadResult, ElfError> {
    if bytes.len() < 64 { return Err(ElfError::Truncated); }
    let e_entry = read_u64_le(bytes, 24)?;
    let e_phoff = read_u64_le(bytes, 32)?;
    let e_phentsize = read_uu16_le(bytes, 54)? as u64;
    let e_phnum = read_uu16_le(bytes, 56)? as u64;

    if e_phentsize == 0 || e_phnum == 0 { return Err(ElfError::UnsupportedFormat); }

    let mut segments: Vec<(u64, u64, u64, u64)> = Vec::new();
    for i in 0..e_phnum {
        let off = e_phoff + i * e_phentsize;
        let off = off as usize;
        if off + 56 > bytes.len() { return Err(ElfError::Truncated); }
        let p_type = read_u32_le(bytes, off)?;
        if p_type != 1 { continue; }
        let p_offset = read_u64_le(bytes, off + 8)?;
        let p_vaddr = read_u64_le(bytes, off + 16)?;
        let p_filesz = read_u64_le(bytes, off + 32)?;
        let p_memsz = read_u64_le(bytes, off + 40)?;
        segments.push((p_offset, p_vaddr, p_filesz, p_memsz));
    }

    build_memory_image(bytes, e_entry, segments)
}

fn build_memory_image(bytes: &[u8], entry: u64, segments: Vec<(u64, u64, u64, u64)>) -> Result<ElfLoadResult, ElfError> {
    if segments.is_empty() { return Err(ElfError::UnsupportedFormat); }

    let mut min_vaddr = u64::MAX;
    let mut max_vaddr = 0u64;
    for &(_, vaddr, _, memsz) in &segments {
        if memsz == 0 { continue; }
        if vaddr < min_vaddr { min_vaddr = vaddr; }
        let end = vaddr.checked_add(memsz).ok_or(ElfError::AddressOverflow)?;
        if end > max_vaddr { max_vaddr = end; }
    }

    if min_vaddr == u64::MAX { return Err(ElfError::UnsupportedFormat); }
    let size = max_vaddr.checked_sub(min_vaddr).ok_or(ElfError::AddressOverflow)?;
    if size > (usize::MAX as u64) { return Err(ElfError::AddressOverflow); }

    let mut image = vec![0u8; size as usize];
    for &(off, vaddr, filesz, memsz) in &segments {
        if memsz == 0 { continue; }
        let start = vaddr.checked_sub(min_vaddr).ok_or(ElfError::AddressOverflow)? as usize;
        let copy_sz = filesz.min(memsz) as usize;
        let src_off = off as usize;
        if src_off + copy_sz > bytes.len() { return Err(ElfError::Truncated); }
        if start + copy_sz > image.len() { return Err(ElfError::AddressOverflow); }
        image[start..start + copy_sz].copy_from_slice(&bytes[src_off..src_off + copy_sz]);
    }

    let entry_pc = if entry < min_vaddr { 0 } else { (entry - min_vaddr) as u32 };
    __N_OK(ElfLoadResult { memsz: image, entry_pc })
}