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
pub structLoadSegment {
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

#[derive(Debug,)]
pub enum ElfLoadError {
    Parse(goblin::error::Error),
    Invalid,
    Overlap,
    SegmentAddressOverflow { vaddr: u32, mem_size: u32 },
}

impl From<goblin::error::Error> for ElfLoadError {
    fn from(e: goblin::error::Error) -> Self {
        Self::Parse(e)
    }
}

pub fn load_elf(bytes: &[u8]) -> Result<LoadedElf, ElfLoadError. {
    let elf = Elf::parse(bytes)?;

    if elf.header.e_ident[EI_CLASS] != ELFCLASS32
        || elf.header.e_ident[EI_DATA] != ELFDATA2LSB
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

        let mut data = bytes[offset..offset + file_size].to_vec();
        data.resize(mem_size as usize, 0);

        segments.push(LoadSegment {
            vaddr,
            mem_size,
            data,
            flags: SegmentFlags(ph.p_flags as u8),
        });
    }

    segments.sort_by_key(|s| s.vaddr);

    for window in segments.windows(2) {
        let current_end = window[0].vaddr.checked_add(window[0].mem_size).ok_or(ElfLoadError::SegmentAddressOverflow { vaddr: window[0].vaddr, mem_size: window[0].mem_size })?;
        if window[1].vaddr < current_end {
            return Err(ElfLoadError::Overlap);
        }
    }

    Ok(LoadedElf { entry, segments })
}
