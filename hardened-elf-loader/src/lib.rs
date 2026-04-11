use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElfLoaderError {
    FileTooSmall,
    InvalidMagic,
    UnsupportedClass,
    UnsupportedEndianness,
    UnsupportedVersion,
    UnsupportedType,
    UnsupportedMachine,
    SegmentOutOfRange,
    AddressOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfImage {
    pub entry: u32,
    pub memory: Vec<u8>,
}

pub fn load_elf(bytes: &[u8], memory_size: usize) -> Result<ElfImage, ElfLoaderError> {
    if bytes.len() < 52 { return Err(ElfLoaderError::FileTooSmall); }
    if &bytes[0..4] != b"\x7fELF" { return Err(ElfLoaderError::InvalidMagic); }
    if bytes[4] != 1 { return Err(ElfLoaderError::UnsupportedClass); }
    if bytes[5] != 1 { return Err(ElfLoaderError::UnsupportedEndianness); }

    let entry = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
    let phoff = u32::from_le_bytes(bytes[28..32].try_into().unwrap()) as usize;
    let phnum = u16::from_le_bytes(bytes[44..46].try_into().unwrap()) as usize;
    let phentsize = u16::from_le_bytes(bytes[42..44].try_into().unwrap()) as usize;

    let mut memory = vec![0u8; memory_size];
    for i in 0..phnum {
        let offset = phoff.checked_add(i.checked_mul(phentsize).ok_or(ElfLoaderError::AddressOverflow)?).ok_or(ElfLoaderError::AddressOverflow)?;
        if offset.checked_add(32).map_or(true, |v| v > bytes.len()) { return Err(ElfLoaderError::FileTooSmall); }

        let p_type = u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());
        if p_type == 1 { // PT_LOAD
            let p_offset = u32::from_le_bytes(bytes[offset+4..offset+8].try_into().unwrap()) as usize;
            let p_vaddr = u32::from_le_bytes(bytes[offset+8..offset+12].try_into().unwrap()) as usize;
            let p_filesz = u32::from_le_bytes(bytes[offset+16..offset+20].try_into().unwrap()) as usize;
            let p_memsz = u32::from_le_bytes(bytes[offset+20..offset+24].try_into().unwrap()) as usize;

            if p_vaddr.checked_add(p_memsz).map_or(true, |v| v > memory_size) { return Err(ElfLoaderError::SegmentOutOfRange); }
            if p_offset.checked_add(p_filesz).map_or(true, |v| v > bytes.len()) { return Err(ElfLoaderError::FileTooSmall); }

            memory[p_vaddr..p_vaddr+p_filesz].copy_from_slice(&bytes[p_offset..p_offset+p_filesz]);
        }
    }
    Ok(ElfImage { entry, memory })
}
