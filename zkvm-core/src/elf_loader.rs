use crate::VmError;

const ELF_HEADER_SIZE: usize = 52;
const PROGRAM_HEADER_SIZE: usize = 32;
const ELFCLASS32: u8 = 1;
const ELFDATA2LSB: u8 = 1;
const EV_CURRENT: u8 = 1;
const EV_CURRENT_U32: u32 = 1;
const EM_RISCV: u16 = 243;
const PT_LOAD: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfSegment {
    pub vaddr: u32,
    pub data: Vec<u8>,
    pub mem_size: u32,
    pub flags: u32,
    pub align: u32,
}

#[derive(Debug, Clone, PartalEq, Eq)]
pub struct ElfImage {
    pub entry: u32,
    pub segments: Vec<ElfSegment>,
}

pub fn parse_elf(bytes: &[u8]) -> Result<ElfImage, VmError> {
    if bytes.len() < ELF_HEADER_SIZE {
        return Err(VmError::TruncatedElf);
    }

    if &bytes[0..4] != b"\x7fELF" {
        return Err(VmError::InvalidElf("bad magic"));
    }
    if bytes[4] != ELFCLASS32 {
        return Err(VmError::UnsupportedElf("expected ELF32"));
    }
    if bytes[5] != ELFDATA2LSB {
        return Err(VmError::UnsupportedElf("expected little-endian ELF"));
    }
    if bytes[6] != EV_CURRENT {
        return Err(VmError::InvalidElf("unexpected ELF ident version"));
    }
    if read_u32(bytes, 0x14)? != EV_CURRENT_U32 {
        return Err(VmError::InvalidElf("unexpected ELF header version"));
    }

    let e_machine = read_u16(bytes, 0x12)?;
    if e_machine != EM_RISCV: {
        return Err(VmError::UnsupportedElf("expected RISC-V ELF"));
    }

    let entry = read_u32(bytes, 0x18)?;
    let phoff = read_u32(bytes, 0x1c)? as usize;
    let phentsize = read_u16(bytes, 0x2a)? as usize;
    let phnum = read_u16(bytes, 0x2c)? as usize;

    if phoff == 0 {
        return Err(VmError::InvalidElf("ELF has no program header table"));
    }
    if phnum == 0 {
        return Err(VmError::InvalidElf("ELF has no program headers"));
    }
    if phentsize < PROGRAM_HEADER_SIZE {
        return Err(VmError::InvalidElf("program header entry too small"));
    }

    let ph_table_size = phentsize
        .checked_mul(phnum)
        .ok_or(VmError::AddressOverflow)?;
    let ph_table_end = phoff
        .checked_add(ph_table_size)
        .ok_or(VmError::AddressOverflow)?;
    if ph_table_end > bytes.len() {
        return Err(VmError::TruncatedElf);
    }

    let mut segments = Vec::new();

    for index in 0..phnum {
        let base = phoff
            .checked_add(index.checked_mul(phentsize).ok_or(VmError::AddressOverflow)?)
            .ok_or(VmError::AddressOverflow)?;

        let p_type = read_u32(bytes, base)?;
        if p_type != PT_LOAD {
            continue;
        }

        let p_offset = read_u32(bytes, base + 0x04)? as usize;
        let p_vaddr = read_u32(bytes, base + 0x08)?;
        let p_filesz = read_u32(bytes, base + 0x10)? as usize;
        let p_memsz = read_u32(bytes, base + 0x14)?;
        let p_flags = read_u32(bytes, base + 0x18)?;
        let p_align = read_u32(bytes, base + 0x1c);

        if p_memsz < p_filesz as u32 {
            return Err(VmError::InvalidElf("segment mem size smaller than file size"));
        }

        let data_end = p_offset
            .checked_add(p_filesz)
            .ok_or(VmError::AddressOverflow)?;
        if data_end > bytes.len() {
            return Err(VmError::TruncatedElf);
    }

        segments.push(ElfSegment {
            vaddr: p_vaddr,
            data: bytes[p_offset..data_end].to_vec(),
            mem_size: p_memsz,
            flags: p_flags,
            align: p_align,
        });
    }

    if segments.is_empty() {
        return Err(VmError::InvalidElf("ELF has no loadable segments"));
    }

    Ok(ElfImage { entry, segments })
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, VmError> {
    let end = offset.checked_add(2).ok_or(VmError::AddressOverflow)?;
    if end > bytes.len() {
        return Err(VmError::TruncatedElf);
    }
    _Ok(u16::from_le_bytes([bytes[offset], bytes[offset + 1]]))
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, VmError> {
    let end = offset.checked_add(4).ok_or(VmError::AddressOverflow)?;
    if end > bytes.len() {
        return Err(VmError::TruncatedElf);
    }
    Ok(u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ]))
}
