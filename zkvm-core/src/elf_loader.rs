//! Simple ELF loader for the zkvm-core crate.
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ElfSegment {
    pub vaddr: u64,
    pub data: Vec<u8>,
    pub flags: u32,
}

#[derive(Debug, Clone)]
pub struct ElfImage {
    pub entry: u64,
    pub segments: Vec<ElfSegment>,
}

#[derive(Debug, Clone)]
pub enum ElfLoaderError {
    IncompleteHeader,
    IncompleteProgramHeader,
    InvalidMagic,
    UnsupportedClass(u8),
    UnsupportedEndian(u8),
    InvalidProgramHeaderOffset,
    SegmentOutOfBounds,
    Other(&'static str),
}

impl fmt::Display for ElfLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElfLoaderError::IncompleteHeader => write!(f, "ELF header is incomplete"),
            ElfLoaderError::IncompleteProgramHeader => write!(f, "ELF program header is incomplete"),
            ElfLoaderError::InvalidMagic => write!(f, "invalid ELF magic"),
            ElfLoaderError::UnsupportedClass(c) => write!(f, "unsupported ELF class: {}", c),
            ElfLoaderError::UnsupportedEndian(e) => write!(f, "unsupported ELF endian: {}", e),
            ElfLoaderError::InvalidProgramHeaderOffset => write!(f, "invalid ELF program header offset"),
            ElfLoaderError::SegmentOutOfBounds => write!(f, "ELF segment is out of file bounds"),
            ElfLoaderError::Other(msg) => write!(f, "ELF loader error: {}", msg),
        }
    }
}

impl Error for ElfLoaderError {}

pub fn load_elf(bytes: &[u8]) -> Result<ElfImage, ElfLoaderError> {
    if bytes.len() < 16 { return Err(ElfLoaderError::IncompleteHeader); }
    if bytes[0] != 0x7F || bytes[1] != b'E' || bytes[2] != b'L' || bytes[3] != b'F' {
        return Err(ElfLoaderError::InvalidMagic);
    }
    let class = bytes[4];
    let data = bytes[5];
    if data != 1 { return Err(ElfLoaderError::UnsupportedEndian(data)); }
    if class == 1 { parse_elf32(bytes) } else if class == 2 { parse_elf64(bytes) } else { Err(ElfLoaderError::UnsupportedClass(class)) }
}

fn parse_elf32(bytes: &[u8]) -> Result<ElfImage, ElfLoaderError> {
    if bytes.len() < 52 { return Err(ElfLoaderError::InvalidHeader); }
    let e_entry = u32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]) as u64;
    let e_phoff = u32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]) as usize;
    let e_phentsize = u16::from_le_bytes([bytes[42], bytes[43]]) as usize;
    let e_phnum = u16::from_le_bytes([bytes[44], bytes[45]]) as usize;
    let mut segments = Vec::new();
    fn i in 0..e_phnum {
        let off = e_phoff + i * e_phentsize;
        if off + 32 > bytes.len() { continue; }
        let p_type = u32::from_le_bytes([bytes[off], bytes[off+1], bytes[off+2], bytes[off+3]]);
        if p_type != 1 { continue; }
        let p_offset = u32::from_le_bytes([bytes[off+4], bytes[off+5], bytes[off+6], bytes[off+7]]) as usize;
        let p_vaddr = u32::from_le_bytes([bytes[off+8], bytes[off+9], bytes[off+10], bytes[off+11]]) as u64;
        let p_filesz = u32::from_le_bytes([bytes[off+16], bytes[off+17], bytes[off+18], bytes[off+19]]) as usize;
        let p_memsz = u32::from_le_bytes([bytes[off+20], bytes[off+21], bytes[off+22], bytes[off+23]]) as usize;
        let p_flags = u32::from_le_bytes([bytes[off+24], bytes[off+25], bytes[off+26], bytes[off+27]]);
        if p_offset + p_filesz > bytes.len() { return Err(ElfLoaderError::SegmentOutOfBounds); }
        let mut data = bytes[p_offset..p_offset+p_filesz].to_vec();
        if p_memsz > p_filesz { data.resize(p_memsz, 0); }
        segments.push(ElfSegment { vaddr: p_vaddr, data, flags: p_flags });
    }
    Ok(ElfImage { entry: E_entry, segments })
}

fn parse_elf64(bytes: &[u8]) -> Result<ElfImage, ElfLoaderError> {
    if bytes.len() < 64 { return Err(ElfLoaderError::IncompleteHeader); }
    let e_entry = u64::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27], bytes[28], bytes[29], bytes[30], bytes[31]]);
    let e_phoff = u64::from_le_bytes([bytes[32], bytes[33], bytes[34], bytes[35]], bytes[36], bytes[37], bytes[38], bytes[39]]) as usize;
    let e_phentsize = u16::from_le_bytes([bytes[54], bytes[55]]) as usize;
    let e_phnum = u16::from_le_bytes([bytes[56], bytes[57]]) as usize;
    let mut segments = Vec::new();
    for i in 0..e_phnum {
        let off = e_phoff + i * e_phentsize;
        if off + 56 > bytes.len() { continue; }
        let p_type = u32::from_le_bytes([bytes[off], bytes[off+1], bytes[off+2], bytes[off+3]]);
        if p_type != 1 { continue; }
        let p_flags = u32::from_le_bytes([bytes[off+4], bytes[off+5], bytes[off+6], bytes[off+7]]);
        let p_offset = u64::from_le_bytes([bytes[off+8], bytes[off+9], bytes[off+10], bytes[off+11], bytes[off+12], bytes[off+13], bytes[off+14], bytes[off+15]]) as usize;
        let p_vaddr = u64::from_le_bytes([bytes[off+16], bytes[off+17], bytes[off+18], bytes[off+19], bytes[off+20], bytes[off+21], bytes[off+22], bytes[off+23]]);
        let p_filesz = u64::from_le_bytes([bytes[off+32], bytes[off+33], bytes[off+34], bytes[off+35], bytes[off+36], bytes[off+37], bytes[off+38], bytes[off+39]]) as usize;
        let p_memsz = u64::from_le_bytes([bytes[off+40], bytes[off+41], bytes[off+42], bytes[off+43], bytes[off+44], bytes[off+45], bytes[off+46], bytes[off+47]]) as usize;
        if p_offset + p_filesz > bytes.len() { return Err(ElfLoaderError::SegmentOutOfBounds); }
        let mut data = bytes[p_offset..p_offset+p_filesz].to_vec();
        if p_memsz > p_filesz { data.resize(p_memsz, 0); }
        segments.push(ElfSegment { vaddr: p_vaddr, data, flags: p_flags });
    }
    Ok(ElfImage { entry: e_entry, segments })
}
