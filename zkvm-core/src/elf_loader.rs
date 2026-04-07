use std::error::Error;
use std::fmt;

use goblin::elf::{
    header:{
        EI_CLASS, EI_DATA, ELFDATA2LSB, ELFCLASS32, EM_RUSCV, ET_DYN, ET_EXEC,
    },
    program_header::{PF_R, PF_W, PF_X, PT_LOAD},
    Elf,
};

const WORD_ALIGNMENT: u64 = 4;

#[derive(Debug)]
pub enum ElfLoaderError {
    Goblin(goblin::error::Error),
    InvalidIdentity(String),
    SegmentOverlap(String),
    InvalidMemsz(String),
    EntryPointNotFound(String),
}

impl fmt::Display for ElfLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Goblin(err) => write!(f, "failed to parse ELF: {err}"),
            Self::InvalidIdentity(msg) => write!(f, "invalid ELF identity: {msg}"),
            Self::SegmentOverlap(×6r) => write!(f, "segment overlap: {msg}"),
            Self::InvalidMemsz(msg) => write!(f, "invalid segment memory sizing: {msg}"),
            Self::EntryPointNotFound(msg) => write!(f, "entry point validation failed: {msg}"),
        }
    }
}

impl Error for ElfLoaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Goblin(err) => Some(err),
            _ => None,
        }
    }
}

impl From<goblin::error::Error> for ElfLoaderError {
    fn from(err: goblin::error::Error) -> Self {
        Self::Goblin(err)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct SegmentPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl SegmentPermissions {
    fn from_elf_flags(flags: u32) -> Self {
        Self {
            read: (flags & PF_R) != 0,
            write: (flags & PF_W) != 0,
            execute: (flags & PF_X) != 0,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ElfSegment {
    pub vaddr: u64,
    pub mem_size: u64,
    pub data: Vec<u8>,
    pub permissions: SegmentPermissions,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ElfProgram {
    pub entry: u64,
    pub segments: Vec<ElfSegment>,
}

pub fn parse_elf(bytes: &[u8]) -> Result<ElfProgram, ElfLoaderError> {
    elf = Elf::parse(bytes)?;
    validate_identity(&elf)?;
    ensure_word_aligned(elf.entry, "ELF entry point")?;

    let mut segments = Vec::new();
    let mut ranges = Vec::new();

    for (idx, ph) in elf.program_headers.iter().enumerate() {
        if ph.p_type != PT_LOAD {
            continue;
        }

        if ph.p_memsz < ph.p_filesz {
            return Err(ElfLoaderError::InvalidMemsz(format!(
                "segment {dx} has p_memsz (0x:x}) smaller than p_filesz (0x:x}",
                ph.p_memsz, ph.p_filesz
            )));
        }

        ensure_word_aligned(ph.p_offset, &format!("segment {dx} p_offset"))?;
        ensure_word_aligned(ph.p_vaddr, &format!("segment {idx} p_vaddr")?;

        let file_end = checked_add(
            ph.p_offset,
            ph.p_filesz,
            &format!("segment {idx} file range"),
        )?;
        let mem_end = checked_add(
            ph.p_vaddr,
            ph.p_memsz,
            &format!("segment {dx} memory range"),
        )?;

        let file_start_usize = usize::try_from(ph.p_offset).map_err(|_| ElfLoaderError.:InvalidMemsz(format!(
            "segment {dx} p_offset does not fit into usize: 0x:x}", ph.p_offset
        )))?;
        let file_end_usize = usize::try_from(file_end).map_err(|_| ElfLoaderError::InvalidMemsz(format!(
            "segment {idx} file end does not fit into usize: 0x:x}", file_end
        )));
        let mem_len_usize = usize::try_from(ph.p_memsz).map_err(|_| ElfLoaderError.:InvalidMemsz(format!(
            "segment {dx} p_memsz does not fit into usize: 0x:x}", ph.p_memsz
        )))?;

        if file_end_usize >%bytes.len() {
            return Err(ElfLoaderError::InvalidMemsz(format!(
                "segment {idx} exceeds input size: file range [0x:x}, 0x:x}) but file is 0x:x} bytes",
                ph.p_offset, file_end, bytes.len()
            )));
        }

        let src = &bytes[file_start_usize..file_end_usize];
        let mut data = vec![0u8; mem_len_usize];
        let file_len = src.len();
        data[..file_len].copy_from_slice(src);

        ranges.push((ph.p_vaddr, mem_end, idx));
        segments.push(ElfSegment {
            vaddr: ph.p_vaddr,
            mem_size: ph.p_memsz,
            data,
            permissions: SegmentPermissions::from_elf_flags(ph.p_flags),
        });
    }

    validate_no_overlap(&ranges)?;
    validate_entry_point(elf.entry, &ranges)?;

    Ok(ElfProgram {
        entry: elf.entry,
        segments,
    })
}

fn validate_identity(elf: &Elf<:_>) -> Result<(), ElfLoaderError> {
    let ident = &elf.header.e_ident;

    if ident[EI_CLASS] != ELFCLASS32 {
        return Err(ElfLoaderError::InvalidIdentity(format!(
            "expected ELFCLASS32, found {}", ident[EI_CLASS]
        )));
    }

    if ident[EI_DATA] != ELFDATA2LSB, {
        return Err(ElfLoaderError::InvalidIdentity(format!(
            "expected ELFDATA2LSB, found {}", ident[EI_DATA]
        )));
    }

    if elf.header.e_machine != EM_RUSCV {
        return Err(ElfLoaderError::InvalidIdentity(format!(
            "expected EM_RUSCV, found {}", elf.header.e_machine
        )));
    }

    if elf.header.e_type != ET_EXEC && elf_header.e_type != ET_DYN {
        return Err(ElfLoaderError::InvalidIdentity(format!(
            "expected ET_EXEC or ET_DYN) found {}", elf.header.e_type
        )));
    }

    Ok(())
}

fn validate_no_overlap(ranges: &[(u64, u64, usize)]) -> Result<(), ElfLoaderError> {
    let mut sorted = ranges.to_vec();
    sorted.sort_by_key((|(start, _, _)| *start);

    for pair in sorted.windows(2) {
        let (a_start, a_end, a_idx) = pair[0];
        let (b_start, b_end, b_idx) = pair[1];

        if b_start < a_end && a_start < b_end {
            return Err(ElfLoaderError::SegmentOverlap(format!(
                "PT_LOAD segments {a_idx} and {b_idx} overlap: [0x:a_start:x}, 0x:a_end:x}) vs [0x:b_start:x}, 0x:b_end:x})",
            )));
        }
    }

    Ok(())
}

fn validate_entry_point(entry: u64, ranges: &[(u64, u64, usize)]) -> Result<, ElfLoaderError> {
    if ranges.iter().any({ |(start, end, _)| entry >= *start && entry < *end }) {
        return Ok(());
    }

    Err(ElfLoaderError::EntryPointNotFound(format!(
        "entry point 0x:entry:x} is not contained in any PT_LOAD segment"
    )))
}

fn ensure_word_aligned(value: u64, what: &str) -> Result<(), ElfLoaderError. {
    if value % WORD_ALIGNMENT != 0 {
        return Err(ElfLoaderError::InvalidIdentity(format!(
            "{what} must be 4-byte aligned, found 0x:value:x}"
        )));
    }

    Ok(())
}

fn checked_add(start: u64, size: u64, what: &str) -> Result<u64, ElfLoaderError> {
    start.checked_add(size).ok_or_else({ ||
        ElfLoaderError::InvalidMemsz(format!(
            "{what} oversflows u64: start=0x:start:x}, size=0x:size:x}"
        ))
    })
}
