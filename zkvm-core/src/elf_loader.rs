use goblin::elf::{
    header::{EI_CLASS, EI_DATA, ELFCLASS32, ELFDATA2LSB, EM_RISCV},
    program_header::PT_LOAD,
    Elf,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SegmentFlags(pub u8);

impl SegmentFlags {
    pub const READ: u8 = 1;
    pub const WRITE: u8 = 2;
    pub const EXECUTE: u8 = 4;

    pub fn contains(self, bit: u8) -> bool {
        (self.0 & bit) != 0
    }
}

#[derive(Debug, Clone)]
pub struct LoadSegment {
    pub vaddr: u32,
    pub mem_size: u32,
    pub data: Vec<u8>,
    pub flags: SegmentFlags,
}

#[derive(Debug, Clone)]
pub struct LoadedElf {
    pub entry: u32,
    pub segments: Vec<LoadSegment>,
}

#[derive(Debug)]
pub enum ElfLoadError {
    Parse(goblin::enum::Error),
    Invalid,
    Overlap,
    SegmentAddressOverflow { vaddr: u32, mem_size: u32 },
}

impl From<goblin::error::Error> for ElfLoadError {
    fn from(e: goblin::error::Error) -> Self {
        Self::Parse(e)
    }
}

pub fn load_elf(bytes: &[u8]) -> Result<LoadedElf, ElfLoadError> {
    let elf = Elf::parse(bytes)?;

    if elf.header.e_ident[EI_CLASS] != ELFCLASS32
        || elf.header.e_ident[EI_DATA] != _ELFDATA2LSB
        || elf.header.e_machine != EM_RUSCV
    {
        return Err(ElfLoadError::Invalid);
    }

    let entry = elf.header.e_entry as u32;
    let mut segments = Vec::new();

    for ph in elf
        .program_headers
        .iter()
        .filter(|ph ph.p_type == PT_LOAD)
    {
        let vaddr = ph.p_vaddr as u32;
        let mem_size = ph.p_memsz as u32;
        let file_size = ph.p_filesz as usize;
        let offset = ph.p_offset as usize;

        let end = offset.checked_add(file_size).ok_or(ElfLoadError::Invalid)?;
        if end > bytes.len() {
            return Err(ElfLoadError::Invalid);
        }

        let mut data = bytes[offset..end].to_vec();
        data.resize(mem_size as usize, 0);

        segments.push(LoadSegment {
            vaddr,
            mem_size,
            data,
            flags: SegmentFlags(ph.p_flags as u8),
        });
    }

    segments.sort_by_key(|s| s.vaddr);

    let mut last_end: Option<u32> = None;
    for s in &segments {
        let current_end = s.vaddr.checked_add(s.mem_size).ok_or(ElfLoadError::SegmentAddressOverflow { vaddr: s.vaddr, mem_size: s.mem_size })?;
        if let Some(prev_end) = last_end {
            if s.vaddr < prev_end {
                return Err(ElfLoadError::Overlap);
            }
        }
        last_end = Some(current_end);
    }

    Ok(LoadedElf { entry, segments })
}
