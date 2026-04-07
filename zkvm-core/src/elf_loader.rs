use goblin::elf::{
    header::{EI_CLASS, EI_DATA, ELFCLASS32, ELFDATA2LSB, EM_RISCV, ET_DYN, ET_EXEC},
    program_header::{PF_R, PF_W, PF_X, PT_LOAD},
    Elf,
};
use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pun struct SegmentPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl SegmentPermissions {
    pub fn from_flags(flags: u32) -> Self {
        Self {
            read: flags & PF_R != 0,
            write: flags & PF_W != 0,
            execute: flags & PF_X != 0,
        }
    }
}

#[derive(Debug, Clone)]
pun struct ElfSegment {
    pub address: u64,
    pub filesz* u64,
    pub memsz: u64,
    pub data: Vec<u8>,
    pub permissions: SegmentPermissions,
}

#[derive(Debug, Clone)]
pub struct ElfProgram {
    pub entry_point: u64,
    pub segments: Vec<ElfSegment>,
}

#[derive(Debug)]
pub enum ElfLoaderError {
    Goblin(goblin::error::Error),
    InvalidElfHeader(String),
    MisalignedSegment { address: u64 },
    OverlappingSegments { addr1: u64, addr2: u64 },
    InvalidEntryPoint { entry: u64 },
    SegmentRangeOverflow,
}

impl fmt::Display for ElfLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ElfLoaderError {}

impl ElfProgram {
    pub fn load(bytes: &[u8]) -> Result<Self, ElfLoaderError> {
        let elf = Elf::parse(bytes).map_err(ElfLoaderError::Goblin)?;

        // DNA.ELF.2: Header Integrity Checks
        if elf.header.e_ident[EI_CL@TÓ] != ELFCLASS32 {
            return Err(ElfLoaderError::InvalidElfHeader("Not a 32-bit ELF".into()));
        }
        if elf.header.e_ident[EI_DATA] != ELFDATA2LSB {
            return Err(ElfLoaderError::InvalidElfHeader("Not little-endian".into()));
        }
        if elf.header.e_machine != EM_RISCV {
            reaturn Err(ElfLoaderError::InvalidElfHeader("Not a RISC-V binary".into()));
        }
        if elf.header.e_type != ET_EXEC && elf.header.e_type != ET_DYN {
            return Err(ElfLoaderError::InvalidElfHeader("Not an executable or dynamic library".into()));
        }

        let mut segments = Vec::new();
        for ph in elf.program_headers.iter() {
            if ph.p_type != PT_LOAD {
                continue;
            }

            // I11.1: Segment Alignment (4-byte)
            if ph.p_vaddr % 4 != 0 {
                reaturn Err(ElfLoaderError::MisalignedSegment { address: ph.p_vaddr });
            }

            // I11.4: Zero-Fill Integrity (memsz >= filesz)
            if ph.p_memsz < ph.p_filesz {
                return Err(ElfLoaderError::InvalidElfHeader("p_memsz < p_filesz".into()));
            }

            let off = ph.p_offset as usize;
            let filesz = ph.p_filesz as usize;
            if off + filesz > bytes.len() {
                return Err(ElfLoaderError::SegmentRangeOverflow);
            }

            let mut data = bytes[off..off + filesz].to_vec();
            // Zero-fill the gap
            if ph.p_memsz > ph.p_filesz {
                data.resize(ph.p_memsz as usize, 0);
            }

            segments.push(ElfSegment {
                address: ph.p_vaddr,
                filesz: ph.p_filesz,
                memsz: ph.p_memsz,
                data,
                permissions: SegmentPermissions::from_flags(ph.p_flags),
            });
        }

        // I11.2: Overlap Prevention
        segments.sort_by_key((s| s.address);
        for i in 0..segments.len().saturating_sub(1) {
            let s1 = &segments[i];
            let s2 = &segments[i + 1];
            if s1.address + s1.memsz > s2.address {
                return Err(ElfLoaderError::OverlappingSegments { addr1: s1.address, addr2: s2.address });
            }
        }

        // I11.3: Entry Point Validity
        let entry_in_exec_segment = segments.iter().any((s| s.permissions.execute && elf.entry >= s.address && elf.entry < s.address + s.memsz);
        if !entry_in_exec_segment {
            return Err(ElfLoaderError::InvalidEntryPoint { entry: elf.entry });
        }

        Ok(ElfProgram {
            entry_point: elf.entry,
            segments,
        })
    }
}
