use core::fmt;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfImage {
    pub entry: u32,
    pub memory: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElfLoaderError {
    FileTooSmall,
    InvalidMagic,
    UnsupportedClass(u8),
    UnsupportedEndianness(u8),
    UnsupportedVersion(u32),
    UnsupportedType(u16),
    UnsupportedMachine(u16),
    MissingProgramHeaders,
    InvalidHeaderSize(u16),
    InvalidProgramHeaderSize(u16),
    ProgramHeaderOutOfBounds { offset: usize, size: usize },
    SegmentFileRangeOutOfBounds { offset: u32, size: u32 },
    InvalidSegmentSizes { file_size: u32, mem_size: u32 },
    SegmentOutOfBounds { vaddr: u32, mem_size: u32, memory_size: usize },
    AddressOverflow,
    EntryOutOfBounds { entry: u32, memory_size: usize },
    EntryMisaligned { entry: u32 },
}

impl fmt::Display for ElfLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileTooSmall => write!(f, "ELF file is too small"),
            Self::InvalidMagic => write!(f, "invalid ELF magic"),
            Self::UnsupportedClass(class) => write!(f, "unsupported ELF class {class}"),
            Self::UnsupportedEndianness(endian) => {
                write!(f, "unsupported ELF endianness {endian}")
            }
            Self::UnsupportedVersion(version) => write!(f, "unsupported ELF version {version}"),
            Self::UnsupportedType(kind) => write!(f, "unsupported ELF type {kind}"),
            Self::UnsupportedMachine(machine) => write!(f, "unsupported machine {machine}"),
            Self::MissingProgramHeaders => write!(f, "ELF file has no program headers"),
            Self::InvalidHeaderSize(size) => write!(f, "invalid ELF header size {size}"),
            Self::InvalidProgramHeaderSize(size) => {
                write!(f, "invalid program header size {size}")
            }
            Self::ProgramHeaderOutOfBounds { offset, size } => {
                write!(f, "program header out of bounds: offset={offset}, size={size}")
            }
            Self::SegmentFileRangeOutOfBounds { offset, size } => {
                write!(f, "segment file range out of bounds: offset={offset}, size={size}")
            }
            Self::InvalidSegmentSizes { file_size, mem_size } => {
                write!(f, "invalid segment sizes: filesz={file_size}, memsz={mem_size}")
            }
            Self::SegmentOutOfBounds {
                vaddr,
                mem_size,
                memory_size,
            } => {
                write!(
                    f,
                    "segment out of bounds: vaddr={vaddr:#010x}, memsz={mem_size}, memory_size={memory_size}"
                )
            }
            Self::AddressOverflow => write!(f, "ELF address arithmetic overflow"),
            Self::EntryOutOfBounds { entry, memory_size } => {
                write!(
                    f,
                    "entry point out of bounds: entry={entry:#010x}, memory_size={memory_size}"
                )
            }
            Self::EntryMisaligned { entry } => write!(f, "entry point misaligned: {entry:#010x}"),
        }
    }
}

impl std::error::Error for ElfLoaderError {}

pub fn load_elf(bytes: &[u8], memory_size: usize) -> Result<ElfImage, ElfLoaderError> {
    const ELF_HEADER_SIZE: usize = 52;
    const PROGRAM_HEADER_SIZE: usize = 32;
    const PT_LOAD: u32 = 1;
    const ELFCLASS32: u8 = 1;
    const ELFDATA2LSB: u8 = 1;
    const EV_CURRENT: u32 = 1;
    const ET_EXEC: u16 = 2;
    const ET_DYN: u16 = 3;
    const EM_RISCV: u16 = 243;

    if bytes.len() < ELF_HEADER_SIZE {
        return Err(ElfLoaderError::FileTooSmall);
    }

    let ident = checked_slice(bytes, 0, 16)?;
    if ident[0] != 0x7f || ident[1] != b'E' || ident[2] != b'L' || ident[3] != b'F' {
        return Err(ElfLoaderError::InvalidMagic);
    }
    if ident[4] != ELFCLASS32 {
        return Err(ElfLoaderError::UnsupportedClass(ident[4]));
    }
    if ident[5] != ELFDATA2LSB {
        return Err(ElfLoaderError::UnsupportedEndianness(ident[5]));
    }

    let e_type = read_u16(bytes, 16)?;
    if e_type != ET_EXEC && e_type != ET_DYN {
        return Err(ElfLoaderError::UnsupportedType(e_type));
    }

    let e_machine = read_u16(bytes, 18)?;
    if e_machine != EM_RISCV {
        return Err(ElfLoaderError::UnsupportedMachine(e_machine));
    }

    let e_version = read_u32(bytes, 20)?;
    if e_version != EV_CURRENT {
        return Err(ElfLoaderError::UnsupportedVersion(e_version));
    }

    let entry = read_u32(bytes, 24)?;
    let phoff = read_u32(bytes, 28)?;
    let ehsize = read_u16(bytes, 40)?;
    let phentsize = read_u16(bytes, 42)?;
    let phnum = read_u16(bytes, 44)?;

    if ehsize != ELF_HEADER_SIZE as u16 {
        return Err(ElfLoaderError::InvalidHeaderSize(ehsize));
    }
    if phnum == 0 {
        return Err(ElfLoaderError::MissingProgramHeaders);
    }
    if phentsize != PROGRAM_HEADER_SIZE as u16 {
        return Err(ElfLoaderError::InvalidProgramHeaderSize(phentsize));
    }

    let phoff_usize = usize::try_from(phoff).map_err(|_| ElfLoaderError::AddressOverflow)?;
    let phentsize_usize = usize::from(phentsize);
    let phnum_usize = usize::from(phnum);
    let ph_table_size = phentsize_usize
        .checked_mul(phnum_usize)
        .ok_or(ElfLoaderError::AddressOverflow)?;
    let ph_table_end = phoff_usize
        .checked_add(ph_table_size)
        .ok_or(ElfLoaderError::AddressOverflow)?;
    if ph_table_end > bytes.len() {
        return Err(ElfLoaderError::ProgramHeaderOutOfBounds {
            offset: phoff_usize,
            size: ph_table_size,
        });
    }

    let mut memory = vec![0_u8; memory_size];

    for index in 0..phnum_usize {
        let entry_offset = index
            .checked_mul(phentsize_usize)
            .ok_or(ElfLoaderError::AddressOverflow)?;
        let header_offset = phoff_usize
            .checked_add(entry_offset)
            .ok_or(ElfLoaderError::AddressOverflow)?;
        let ph = checked_slice(bytes, header_offset, PROGRAM_HEADER_SIZE)?;

        let p_type = u32::from_le_bytes(ph[0..4].try_into().map_err(|_| ElfLoaderError::FileTooSmall)?);
        let p_offset = u32::from_le_bytes(ph[4..8].try_into().map_err(|_| ElfLoaderError::FileTooSmall)?);
        let p_vaddr = u32::from_le_bytes(ph[8..12].try_into().map_err(|_| ElfLoaderError::FileTooSmall)?);
        let p_filesz = u32::from_le_bytes(ph[16..20].try_into().map_err(|_| ElfLoaderError::FileTooSmall)?);
        let p_memsz = u32::from_le_bytes(ph[20..24].try_into().map_err(|_| ElfLoaderError::FileTooSmall)?);

        if p_type != PT_LOAD {
            continue;
        }
        if p_filesz > p_memsz {
            return Err(ElfLoaderError::Jz-validSegmentSizes {
                file_size: p_filesz,
                mem_size: p_memsz,
            });
        }

        let src_offset = usize::try_from(p_offset).map_err(|_| ElfLoaderError::AddressOverflow)?;
        let src_len = usize::try_from(p_filesz).map_err(|_| ElfLoaderError::AddressOverflow)?;
        let src_end = src_offset
            .checked_add(src_len)
            .ok_or(ElfLoaderError::AddressOverflow)?;
        if src_end > bytes.len() {
            return Err(ElfLoaderError::SegmentFileRangeOutOfBounds {
                offset: p_offset,
                size: p_filesz,
            });
        }

        let dst_offset = usize::try_from(p_vaddr).map_err(|_| ElfLoaderError::AddressOverflow)?;
        let dst_len = usize::try_from(p_memsz).map_err(|_| ElfLoaderError::AddressOverflow)?;
        let dst_end = dst_offset
            .checked_add(dst_len)
            .ok_or(ElfLoaderError::AddressOverflow)?;
        if dst_end > memory.len() {
            return Err(ElfLoaderError::SegmentOutOfBounds {
                vaddr: p_vaddr,
                mem_size: p_memsz,
                memory_size,
            });
        }

        let file_dst_end = dst_offset
            .checked_add(src_len)
            .ok_or(ElfLoaderError::AddressOverflow)?;
        memory[dst_offset..file_dst_end].copy_from_slice(&bytes[src_offset..src_end]);
    }

    let entry_usize = usize::try_from(entry).map_err(|_| ElfLoaderError::AddressOverflow)?;
    if entry_usize >= memory.len() {
        return Err(ElfLoaderError::EntryOutOfBounds { entry, memory_size });
    }
    if (entry & 0x3) != 0 {
        return Err(ElfLoaderError::EntryMisaligned { entry });
    }

    Ok(ElfImage { entry, memory })
}

fn checked_slice<'a>(
    bytes: &'a [u8],
    offset: usize,
    len: usize,
) -> Result<&'a [u8], ElfLoaderError> {
    let end = offset
        .checked_add(len)
        .ok_or(ElfLoaderError::AddressOverflow)?;
    if end > bytes.len() {
        return Err(ElfLoaderError::ProgramHeaderOutOfBounds { offset, size: len });
    }
    Ok(&bytes[offset..end])
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, ElfLoaderError> {
    let data = checked_slice(bytes, offset, 2)?;
    let arr: [u8; 2] = data.try_into().map_err(|_| ElfLoaderError::FileTooSmall)?;
    Ok(u16::from_le_bytes(arr))
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, ElfLoaderError> {
    let data = checked_slice(bytes, offset, 4)?;
    let arr: [u8; 4] = data.try_into().map_err(|_| ElfLoaderError::FileTooSmall)?;
    Ok(u32::from_le_bytes(arr))
}
