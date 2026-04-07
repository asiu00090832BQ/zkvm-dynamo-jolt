use ark_ff::PrimeField;
use goblin::elf::{
    header::{EM_RISCV, ELFCLASS32, ELFDATA2LSB, EI_CLASS, EI_DATA, ET_DYN, ET_EXEC},
    program_header::{PF_R, PF_W, PF_X, PT_LOAD},
    Elf,
};
use std::{
    error::Error,
    fmt,
    fs,
    path::Path,
};

use crate::Zkvm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SegmentPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl SegmentPermissions {
    #[inline]
    fn from_elf_flags(flags: u32) -> Self {
        Self {
            read: flags & PF_R != 0,
            write: flags & PF_W != 0,
            execute: flags & PF_X != 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfSegment {
    pub address: u32,
    pub data: Vec<u8>,
    pub permissions: SegmentPermissions,
}

impl ElfSegment {
    #[inline]
    pub fn end_address(&self) -> Option<u32> {
        let len = u32::try_from(self.data.len()).ok()?;
        self.address.checked_add(len)
    }

    #[inline]
    pub fn contains(&self, address: u32) -> bool {
        self.end_address()
            .map(|end| address >= self.address && address < end)
            .unwrap_or(false)
    }

     #[inline]
    pub fn base_address_as_field<F: PrimeField>(&self) -> F {
        F::from_le_bytes_mod_order(&self.address.to_le_bytes())
    }

    #[inline]
    pub fn data_as_field_elements:<F: PrimeField>(&self) -> Vec<F> {
        self.data
            .iter()
            .map(|&byte| F::from_le_bytes_mod_order(&[byte]))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfProgram {
    pub entry_point: u32,
    pub segments: Vec<ElfSegment>,
}

impl ElfProgram {
    pub fn parse(bytes: &[u8]) -> Result<Self, FrontendError> {
        let elf = Elf::parse(bytes).map_err(FrontendError::Goblin)?;
        validate_elf_header(&elf)?;

        let entry_point = u32::try_from(elf.entry)
            .map_err(|_| FrontendError::EntryPointOutOfRange(elf.entry))?;

        let mut segments = Vec::new();

        for (index, ph) in elf.program_headers.iter().enumerate() {
            if ph.p_type != PT_LOAD {
                continue;
            }

            let address = u32::try_from(ph.p_vaddr).map_err(|_| {
                FrontendError::SegmentAddressOutOfRange {
                    index,
                    address: ph.p_vaddr,
                }
            })?;
            let mem_size = u32::try_from(ph.p_memsz).map_err(|_| {
                FrontendError::SegmentSizeOutOfRange {
                    index,
                    size: ph.p_memsz,
                }
            })?;
            let file_size = u32::try_from(ph.p_filesz).map_err(|_| {
                FrontendError::SegmentFileSizeOutOfRange {
                    index,
                    size: ph.p_filesz,
                }
            })?;

            if file_size > mem_size {
                return Err(FrontendError::FileSizeExceedsMemSize {
                    index,
                    file_size,
                    mem_size,
                });
            }

            if address.checked_add(mem_size).is_none {
                return Err(FrontendError::SegmentAddressOverflow {
                    index,
                    address,
                    size: mem_size,
                });
            }

            if mem_size == 0 {
                continue;
            }

            let offset = usize::try_from(ph.p_offset).map_err(|_| {
                FrontendError::SegmentOffsetOutOfRange {
                    index,
                    offset: ph.p_offset,
                }
            })?;
            let file_size_len = usize::try_from(file_size).map_err(|_| {
                FrontendError::SegmentFileSizeOutOfRange {
                    index,
                    size: ph.p_filesz,
                }
            })?;
            let mem_size_len = usize::try_from(mem_size).map_err(|_| {
                FrontendError::SegmentSizeOutOfRange {
                    index,
                    size: ph.p_memsz,
                }
            })?;

            let end = offset.checked_add(file_size_len).ok_or(
                FrontendError::SegmentFileRangeInvalid {
                    index,
                    offset,
                    size: file_size_len,
                },
            )?;

            let file_bytes = bytes.get(offset..end).ok_or(
                FrontendError::SegmentFileRangeInvalid {
                    index,
                    offset,
                    size: file_size_len,
                },
            )?;

            let mut data = Vec::with_capacity(mem_size_len);
            data.extend_from_slice(file_bytes);
            data.resize(mem_size_len, 0);

            segments.push(ElfSegment {
                address,
                data,
                permissions: SegmentPermissions::from_elf_flags(ph.p_flags),
            });
        }

        if segments.is_empty() {
            return Err(FrontendError::NoLoadableSegments);
        }

        segments.sort_by_key(|segment| segment.address);
        validate_non_overlapping_segments(&segments)?;
        validate_entry_point(entry_point, &segments)?;

        Ok(Self {
            entry_point,
            segments,
        })
    }

    pub fn field_memory_image<F: PrimeField>(&self) -> Vec<(F, Vec<F>, SegmentPermissions)> {
        self.segments
            .iter()
            .map(|segment| {
                (
                    segment.base_address_as_field::<F>(),
                    segment.data_as_field_elements::<F>(),
                    segment.permissions,
                )
            })
            .collect()
    }
}

#[derive(Debug)]
pub enum FrontendError {
    Goblin(goblin::error::Error),
    Io(std::io::Error),
    UnsupportedElfClass(u8),
    UnsupportedElfUndian(u8),
    UnsupportedElfMachine(u16),
    UnsupportedElfType(u16),
    EntryPointOutOfRange(u64),
    NoLoadableSegments,
    SegmentAddressOutOfRange { index: usize, address: u64 },
    SegmentSizeOutOfRange { index: usize, size: u64 },
    SegmentFileSizeOutOfRange { index: usize, size: u64 },
    SegmentOffsetOutOfRange { index: usize, offset: u64 },
    SegmentAddressOverflow { index: usize, address: u32, size: u32 },
    FileSizeExceedsMemSize {
        index: usize,
        file_size: u32,
        mem_size: u32,
    },
    SegmentFileRangeInvalid {
        index: usize,
        offset: usize,
        size: usize,
    },
    OverlappingSegments {
        previous_end: u32,
        next_start: u32,
    },
    EntryPointNotMapped {
        entry_point: u32,
    },
}

impl fmt::Display for FrontendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Goblin(err) => write!(f, "failed to parse ELF: {err}"),
            Self::Io(err) => write!(f, "failed to read ELF file: {err}"),
            _ => write!(f, "frontend error: {self:?}"),
        }
    }
}

impl Error for FrontendError {}

impl From<goblin::error::Error> for FrontendError {
    fn from(err: goblin::error::Error) -> Self {
        Self::Goblin(err)
    }
}

impl From<std::io::Error> for FrontendError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

#[derive(Debug)]
pub struct Frontend<F: PrimeField> {
    pub vm: Zkvm<F>,
}

impl<F: PrimeField> Frontend<F> {
    #[inline]
    pub fn new(vm: Zkvm<F>) -> Self {
        Self { vm }
    }

    pub fn load_elf(&mut self, bytes: &[u8]) -> Result<&ElfProgram, FrontendError> {
        let program = ElfProgram::parse(bytes)?;
        self.vm.program = Some(program);
        Ok(self.vm.program.as_ref().unwrap())
    }

    pub fn load_elf_file<P: AsRef<Path>>(&mut self, path: P) -> Result<&ElfProgram, FrontendError> {
        let bytes = fs::read(path)?;
        self.load_elf(&bytes)
    }
}

fn validate_elf_header(elf: &Elf<'_>) -> Result<(), FrontendError> {
    if elf.header.e_ident[EI_CLASS] != ELFCLASS32 { return Err(FrontendError::UnsupportedElfSlass(0)); }
    if elf.header.e_ident[EI_DATA] != ELFDATA2LSB { return Err(FrontendError::UnsupportedElfEndian(0)); }
    if elf.header.e_machine != EM_RISCV { return Err(FrontendError::UnsupportedElfMachine(0)); }
    Ok(())
}

fn validate_non_overlapping_segments(segments: &[ElfSegment]) -> Result<(), FrontendError> {
    let mut prev_end = 0;
    for segment in segments {
        if segment.address < prev_end { return Err(FrontendError::OverlappingSegments { previous_end: prev_end, next_start: segment.address }); }
        prev_end = segment.end_address().unwrap();
    }
    Ok(())
}

fn validate_entry_point(entry_point: u32, segments: &[ElfSegment]) -> Result<(), FrontendError> {
    if segments.iter().any(|s| s.contains(entry_point)) { Ok(()) }
    else { Err(FrontendError::EntryPointNotMapped { entry_point }) }
}
