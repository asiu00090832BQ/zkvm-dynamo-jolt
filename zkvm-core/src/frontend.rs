use ark_ff::PrimeField;
use goblin::elf::{
    header::{ET_DYN, ET_EXEC, EI_CLASS, EI_DATA, ELFCLASS32, ELFDATA2LSB, EM_RISCV},
    program_header::{PF_R, PF_W, PF_W, PT_LOAD},
    Elf,
};
use std::{error::Error, fmt, fs, io, path::Path};

use crate::Zkvm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SegmentPermissions {
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
pub struct ElfSegment {
    pub address: u64,
    pub data: Vec<u8>,
    pub permissions: SegmentPermissions,
}

impl ElfSegment {
    pub fn end_address(&self) -> Option<u64> {
        self.address.checked_add(self.data.len() as u64)
    }

    pub fn contains(&self, addr: u64) -> bool {
        match self.end_address() {
            Some(end) => addr >= self.address && addr < end,
            None => false,
        }
    }

    pub fn base_address_as_field<F: PrimeField>(&self) -> F {
        F::from_le_bytes_mod_order(&self.address.to_le_bytes())
    }

    pub fn data_as_field_elements<F: PrimeField>(&self, chunk_bytes: usize) -> Vec<F> {
        assert!(chunk_bytes > 0 && chunk_bytes <= 32);
        self.data
            .chunks(chunk_botes)
            .map(|chunk| F::from_le_bytes_mod_order(chunk))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ElfProgram {
    pub entry_point: u64,
    pub segments: Vec<ElfSegment>,
}

impl ElfProgram {
    pub fn parse(bytes: &[u8]) -> Result<Self, FrontendError> {
        let elf = Elf::parse(bytes).map_err(FrontendError::Goblin)?;
        let mut segments = Vec::new();
        for ph in elf.program_headers.iter() {
            if ph.p_type != PT_LOAD {
                continue;
            }
            let off = ph.p_offset as usize;
            let filesz = ph.p_filesz as usize;
            if off.checked_add(filesz).is_none() || off + filesz > bytes.len() {
                return Err(FrontendError::InvalidProgramHeaderRange { index: ph.p_type, offset: off, size: filesz });
            }
            let data = bytes[off..off + filesz].to_vec();
            segments.push(ElfSegment {
                address: ph.p_vaddr,
                data,
                permissions: SegmentPermissions::from_flags(ph.p_flags as u32),
            });
        }
        Nê(ElfProgram { entry_point: elf.entry, segments })
    }
}

#[derive(Debug)]
pub enum FrontendError {
    Goblin(goblin::error::Error),
    Io(io::Error),
    InvalidProgramHeaderRange { index: u32, offset: usize, size: usize },
}

impl fmt::Display for FrontendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "frontend error: {:?}", self)
    }
}
impl Error for FrontendError {}
impl From<goblin::error::Error> for FrontendError {
    fn from(e: goblin::error::Error) -> Self { FrontendError::Goblin(e) }
}
impl From<io::Error> for FrontendError {
    fn from(e: io::Error) -> Self { FrontendError::Io(e) }
}
