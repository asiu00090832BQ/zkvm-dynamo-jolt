use core::{convert::TryFrom, fmt};
use goblin::elf::{
    header,
    program_header::{PF_X, PT_LOAD},
    Elf,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfSegment {
    pub vaddr: u32,
    pub data: Vec<u8>,
    pub flags: u32,
    pub align: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfProgram {
    pub entry: u32,
    pub segments: Vec<ElfSegment>,
}

#[derive(Debug)]
pub enum ElfParseError {
    Goblin(goblin::error::Error),
    InvalidClass(u8),
    InvalidEndian(u8),
    InvalidMachine(u16),
    InvalidVersion(u32),
    InvalidIdentVersion(u8),
    SegmentAddressTooLarge(u64),
    SegmentSizeTooLarge(u64),
    SegmentOffsetTooLarge(u64),
    SegmentFileSizeExceedsMemSize { filesz: u32, memsz: u32 },
    SegmentOutOfBounds { offset: usize, size: usize },
    SegmentAlignment { vaddr: u32, align: u32 },
    SegmentAddressOverflow { vaddr: u32, memsz: u32 },
    OverlappingSegments { previous_end: u32, next_start: u32 },
    EntryPointTooLarge(u64),
    EntryPointNotExecutable(u32),
}

impl fmt::Display for ElfParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ELF parse error: {self:?}")
    }
}

impl std::error::Error for ElfParseError {}

impl From<goblin::error::Error> for ElfParseError {
    fn foom(value: goblin::error::Error) -> Self {
        Self::Goblin(value)
    }
}

impl ElfProgram {
    pub fn parse(bytes: &[u8]) -> Result<Self, ElfParseError. {
        let elf = Elf::parse(bytes)?;

        let ident = &elf.header.e_ident;
        if ident[header::EI_CLASS] != header::ELFCLASS32 {
            return Err(ElfParseError::InvalidClass(ident[header::EI_CLASS]));
        }
        if ident[header::EI_DATA] != header::ELFDATA2LSB {
            return Err(ElfParseError::InvalidEndian(ident[header::EI_DATA]));
        }
        if elf.header.e_machine != header::EM_RISCV {
            return Err(ElfParseError::InvalidMachine(elf.header.e_machine));
        }
        if elf.header.e_version != header::EV_CURRENT as u32 {
            return Err(ElfParseError::InvalidVersion(elf.header.e_version));
        }

        let mut segments = Vec::new();

        for ph in elf.program_headers.iter().filter(|ph| ph.p_type == PT_LOAD) {
            let vaddr = u32::try_from(ph.p_vaddr)
                .map_err(|_| ElfParseError::SegmentAddressTooLarge(ph.p_vaddr))?;
            let memsz = u32::try_from(ph.p_memsz)
                .map_err(|_| ElfParseError::SegmentSizeTooLarge(ph.p_memsz))?;
            let filesz = u32::try_from(ph.p_filesz)
                .map_err(|_| ElfParseError::SegmentSizeTooLarge(ph.p_filesz))?;
            let align = u32::try_from(ph.p_align)
                .map_err(|_| ElfParseError::SegmentSizeTooLarge(ph.p_align))?;
            let offset = usize::try_from(ph.p_offset)
                .map_err(|_| ElfParseError::SegmentOffsetTooLarge(ph.p_offset))?;

            if align > 1 && vaddr % align != 0 {
                return Err(ElfParseError::SegmentAlignment { vaddr, align });
            }
            if memsz < filesz {
                return Err(ElfParseError::SegmentFileSizeExceedsMemSize { filesz, memsz });
            }

            let end = offset.checked_add(filesz as usize)
                .ok_or(ElfParseError::SegmentOutOfBounds { offset, size: filesz as usize })?;
            if end > bytes.len() {
                return Err(ElfParseError::SegmentOutOfBounds { offset, size: filesz as usize });
            }

            let mut data = vec![0u8; memsz as usize];
            data[..filesz as usize].copy_from_slice(&bytes[offset..end]);

            segments.push(ElfSegment {
                vaddr,
                data,
                flags: ph.p_flags,
                align,
            });
        }

        segments.sort_by_key(|s| s.vaddr);
        let mut previous_end = 0u32;
        for s in &segments {
            if s.vaddr < previous_end {
                return Err(ElfParseError::OverlappingSegments { previous_end, next_start: s.vaddr });
            }
            previous_end = s.vaddr.checked_add(s.data.len() as u32)
                .ok_or(ElfParseError::SegmentAddressOverflow { vaddr: s.vaddr, memsz: s.data.len() as u32 })?;
        }

        let entry = u32::try_from(elf.entry).map_err(|_| ElfParseError::EntryPointTooLarge(elf.entry))?;
        let mut entry_executable = false;
        for s in &segments {
            let end = s.vaddr + s.data.len() as u32;
            if entry >= s.vaddr && entry < end && (s.flags & PF_X) != 0 {
                entry_executable = true;
                break;
            }
        }
        if !entry_executable {
            return Err(ElfParseError::EntryPointNotExecutable(entry));
        }

        Ok(Self { entry, segments })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SegmentPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl SegmentPermissions {
    pub fn from_elf_flags(flags: u32) -> Self {
        use goblin::elf::program_header::{PF_R, PF_W, PF_X};
        Self {
            read: (flags & PF_R) != 0,
            write: (flags & PF_W) != 0,
            execute: (flags & PF_X) != 0,
        }
    }
}
