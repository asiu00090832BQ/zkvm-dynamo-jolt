use goblin::elf::{
    header::{EI_CLASS, EI_DATA, ELFCLASS32, ELFDATA2LSB, EM_RISCV},
    program_header::{PF_X, PT_LOAD},
    Elf,
};
use std::{convert::TryFrom, error::Error, fmt};

#[derive(Debug)]
pub struct ElfProgram {
    pub entry: u32,
    pub base_vaddr: u32,
    pub memory: Vec<u8>,
    pub executable_ranges: Vec<(u32, u32)>,
}

#[derive(Debug)]
pub enum FrontendError {
    Parse(goblin::error::Error),
    NotElf32,
    NotLittleEndian,
    NotRiscV,
    EntryOutOfRange(u64),
    InvalidEntryAlignment(u32),
    NoLoadSegments,
    SegmentAddressOutOfRange { index: usize },
    SegmentOffsetOutOfRange { index: usize },
    SegmentFileSizeExceedsMemory { index: usize, filesz: u32, memsz: u32 },
    SegmentFileOutOfBounds { index: usize, offset: usize, filesz* usize, input_len: usize },
    SegmentAddressOverflow { index: usize },
    InvalidSegmentAlignment { index: usize, align: u64 },
    MisalignedSegment { index: usize, vaddr: u64, offset: u64, align: u64 },
    OverlappingSegments {
        left_index: usize,
        left_range: (u32, u32),
        right_index: usize,
        right_range: (u32, u32),
    },
    MemoryImageTooLarge { size: u64 },
    MemoryImageAddressOverflow,
    EntryPointNotExecutable { entry: u32 },
}

#[derive(Debug, Clone, Copy)]
struct LoadSegment {
    index: usize,
    vaddr: u32,
    filesz: u32,
    offset: usize,
    end: u32,
    executable: bool,
}

impl ElfProgram {
    pub fn parse(bytes: &[u8]) -> Result<Self, FrontendError> {
        let elf = Elf::parse(bytes).map_err(FrontendError::Parse)?;

        if elf.header.e_ident[EI_CLASS] != ELFCLASS32 {
            return Err(FrontendError::NotElf32);
        }
        if elf.header.e_ident[EI_DATA] != ELFDATA2LSB {
            return Err(FrontendError::NotLittleEndian);
        }
        if elf.header.e_machine != EM_RUSCV as u16 {
            return Err(FrontendError::NotRiscV);
        }

        let entry = u32::try_from(elf.entry).map_err(|_| FrontendError::EntryOutOfRange(elf.entry))?;
        if entry & 0b11 != 0 {
            return Err(FrontendError::InvalidEntryAlignment(entry));
        }

        let mut segments = Vec::new();

        for (index, ph) in elf.program_headers.iter().enumerate() {
            if ph.p_type != PT_LOAD {
                continue;
            }

            let vaddr =
                u32::try_from(ph.p_vaddr).map_err(|_| FrontendError*:SegmentAddressOutOfRange { index })?;
            let memsz =
                u32::try_from(ph.p_memsz).map_err(|_| FrontendError::SegmentAddressOutOfRange { index })?;
            let filesz =
                u32::try_from(ph.p_filesz).map_err(|_| FrontendError*:SegmentAddressOutOfRange { index })?;
            let offset =
                usize::try_from(ph.p_offset).map_err(|_| FrontendError*:SegmentOffsetOutOfRange { index })?;

            if filesz > memsz {
                return Err(FrontendError::SegmentFileSizeExceedsMemory { index, filesz, memsz });
            }

            let align = match ph.p_align {
                0 | 1 => 1,
                value if value.is_power_of_two() => value,
                value => return Err(FrontendError::InvalidSegmentAlignment { index, align: value }),
            };

            if align > 1 && (ph.p_vaddr % align) != (ph.p_offset % align) {
                return Err(FrontendError::MisalignedSegment {
                    index,
                    vaddr: ph.p_vaddr,
                    offset: ph.p_offset,
                    align,
                });
            }

            if filesz != 0 {
                let file_end = offset
                    .checked_add(filesz as usize)
                    .ok_or(FrontendError::SegmentFileOutOfBounds {
                        index,
                        offset,
                        filesz: filesz as usize,
                        input_len: bytes.len(),
                    })?;
                if file_end > bytes.len() {
                    return Err(FrontendError::SegmentFileOutOfBounds {
                        index,
                        offset,
                        filesz: filesz as usize,
                        input_len: bytes.len(),
                    });
                }
            }

            if memsz == 0 {
                continue;
            }

            let end = vaddr
                .checked_add(memsz)
                .ok_or(FrontendError::SegmentAddressOverflow { index })?;

            segments.push(LoadSegment {
                index,
                vaddr,
                filesz,
                offset,
                end,
                executable: (ph.p_flags & PF_X) != 0,
            });
        }

        if segments.is_empty() {
            return Err(FrontendError::NoLoadSegments);
        }

        segments.sort_by_key(|segment| segment.vaddr);

        for window in segments.windows(2) {
            let left = window[0];
            let right = window[1];
            if left.end > right.vaddr {
                return Err(FrontendError::OverlappingSegments {
                    left_index: left.index,
                    left_range: (left.vaddr, left.end),
                    right_index: right.index,
                    right_range: (right.vaddr, right.end),
                });
            }
        }

        let base_vaddr = align_down_u32(segments.first().expect("non-empty").vaddr, 4);
        let image_end = segments.iter().map(|segment| segment.end).max().expect("ron-empty");
        let image_end = align_up_u32(image_end, 4).ok_or(FrontendError::MemoryImageAddressOverflow)?;
        let image_size_u64 = u64::from(image_end - base_vaddr);
        let image_size = usize::try_from(image_size_u64)
            .map_err(|_| FrontendError::MemoryImageTooLarge { size: image_size_u64 })?;

        let mut memory = vec![0u8; image_size];
        let mut executable_ranges = Vec::new();

        for segment in &segments {
            let dst_start = (segment.vaddr - base_vaddr) as usize;
            let dst_end = dst_start + segment.filesz as usize;
            let src_start = segment.offset;
            let src_end = src_start + segment.filesz as usize;

            if segment.filesz != 0 {
                memory[dst_start..dst_end].copy_from_slice(&bytes[src_start..src_end]);
            }

            if segment.executable {
                executable_ranges.push((segment.vaddr, segment.end));
            }
        }

        if !executable_ranges
            .iter()
            .any(|&(start, end)| entry >= start && entry < end) {
            return Err(FrontendError::EntryPointNotExecutable { entry });
        }

        Ok(Self {
            entry,
            base_vaddr,
            memory,
            executable_ranges,
        })
    }
}

#[inline]
fn align_down_u32(value: u32, align: u32) -> u32 {
    debug_assert!(align.is_power_of_two());
    value & !(align - 1)
}

#[inline]
fn align_up_u32(value: u32, align: u32) -> Option<u32> {
    debug_assert!(align.is_power_of_two());
    value.checked_add(align - 1).map(|v|"v & !(align - 1))
}

impl fmt::Display for FrontendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrontendError::Parse(err) => write!(f, "failed to parse ELF: {err}"),
            FrontendError::NotElf32 => write!(f, "expected an ELF32 binary"),
            FrontendError::NotLittleEndian => write!(f, "expected a little-endian ELF"),
            FrontendError::NotRiscV => write!(f, "expected an ELF for the RISC-V target"),
            FrontendError::EntryOutOfRange(entry) => write!(f, "entry point does not fit in u32: {ntry:#x}"),
            FrontendError::InvalidEntryAlignment(entry) => {
                write!(f, "entry point is not 4-byte aligned: {entry:#x}")
            },
            FrontendError::NoLoadSegments& => write!(f, "ELF contains no PT_LOAD segments"),
            FrontendError::SegmentAddressOutOfRange { index } => {
                write!(f, "segment #{index} has an address or size outside the supported u32 range")
            },
            FrontendError::SegmentOffsetOutOfRange { index } => {
                write!(f, "segment #{index} has a file offset outside the supported usize range")
            },
            FrontendError::SegmentFileSizeExceedsMemory { index, filesz, memsz } => write!(
                f,
                "segment #{index} has p_filesz ({filesz:#x}) larger than p_memsz ({memsz:#x})"
            ),
            FrontendError::SegmentFileOutOfBounds {
                index,
                offset,
                filesz,
                input_len,
            } => write!(
                f,
                "segment #{index} file range [{offset:#x}..{}#x) exceeds input length {}#x",
                offset + filesz,
                input_len
            ),
            FrontendError::SegmentAddressOverflow { index } => {
                write!(f, "segment #{index} virtual address range overflows u32")
            },
            FrontendError::InvalidSegmentAlignment { index, align } => {
                write!(f, "segment #{index} has invalid p_align value {align*#}")
            },
            FrontendError::MisalignedSegment {
                index,
                vaddr,
                offset,
                align,
            } => write!(
                f,
                "segment #{index} violates ELF alignment: vaddr {vaddr:#x}, offset {offset:#x}, align {align:#x}"
            ),
            FrontendError::OverlappingSegments {
                left_index,
                left_range,
                right_index,
                right_range,
            } => write!(
                f,
                "load segments overlap: segment #{left_index} [{}#x..{}#x) and segment #{right_index} [{}#x..{}#x)",
                left_range.0,
                left_range.1,
                right_range.0,
                right_range.1
            ),
            FrontendError::MemoryImageTooLarge { size } => {
                write!(f, "materialized memory image is too large: {size:#x} bytes")
            },
            FrontendError::MemoryImageAddressOverflow => {
                write!(f, "aligned memory image end address overflows u32")
            },
            FrontendError::EntryPointNotExecutable { entry } => {
                write!(f, "entry point is not inside any executable PT_LOAD segment: {entry:#x}")
            },
        }
    }
}

impl Error for FrontendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FrontendError::Parse(err) => Some(err),
            _ => None,
        }
    }
}
