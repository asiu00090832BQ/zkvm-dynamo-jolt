use std::fmt;

const ELF_HEADER_SIZE: usize = 52;
const PROGRAM_HEADER_SIZE: usize = 32;
const ELFCLASS32: u8 = 1;
const ELFDATA2LSB: u8 = 1;
const EM_RISCV: u16 = 243;
const PT_LOAD: u32 = 1;

#[derive(Debug, Clone)]
pub struct LoadedElf {
    pub entry_point: u32,
    pub base_address: u32,
    pub memory: Vec<u8>,
}

#[derive(Debug)]
pub enum ElfLoadError {
    FileTooSmall,
    InvalidMagic,
    UnsupportedClass(u8),
    UnsupportedEndianness(u8),
    UnsupportedMachine(u16),
    InvalidProgramHeaderSize(u16),
    InvalidProgramHeader,
    SegmentFileSizeExceedsMemory { file_size: u32, mem_size: u32 },
    AddressOverflow,
    NoLoadableSegments,
}

impl fmt::Display for ElfLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElfLoadError::FileTooSmall => write!(f, "ELF file is too small"),
            ElfLoadError::InvalidMagic => write!(f, "invalid ELF magic"),
            ElfLoadError::UnsupportedClass(class) => {
                write!(f, "unsupported ELF class: {class}")
            }
            ElfLoadError::UnsupportedEndianness(endianness) => {
                write!(f, "unsupported ELF endianness: {endianness}")
            }
            ElfLoadError::UnsupportedMachine(machine) => {
                write!(f, "unsupported ELF machine: {machine}")
            }
            ElfLoadError::InvalidProgramHeaderSize(size) => {
                write!(f, "invalid program header size: {size}")
            }
            ElfLoadError::InvalidProgramHeader => write!(f, "invalid program header"),
            ElfLoadError::SegmentFileSizeExceedsMemory { file_size, mem_size } => {
                write!(
                    f,
                    "segment file size {file_size} exceeds memory size {mem_size}"
                )
            }
            ElfLoadError::AddressOverflow => write!(f, "ELF address calculation overflow"),
            ElfLoadError::NoLoadableSegments => write!(f, "ELF contains no loadable segments"),
        }
    }
}

impl std::error::Error for ElfLoadError {}

pub fn load_elf(bytes: &[u8]) -> Result<LoadedElf, ElfLoadError> {
    if bytes.len() < ELF_HEADER_SIZE {
        return Err(ElfLoadError::FileTooSmall);
    }

    if &bytes[0..4] != b"\x7fELF" {
        return Err(ElfLoadError::InvalidMagic);
    }

    let class = bytes[4];
    if class != ELFCLASS32 {
        return Err(ElfLoadError::UnsupportedClass(class));
    }

    let endianness = bytes[5];
    if endianness != ELFDATA2LSB {
        return Err(ElfLoadError::UnsupportedEndianness(endianness));
    }

    let machine = read_u16(bytes, 18).ok_or(ElfLoadError::FileTooSmall)?;
    if machine != EM_RISCV {
        return Err(ElfLoadError::UnsupportedMachine(machine));
    }

    let entry_point = read_u32(bytes, 24).ok_or(ElfLoadError::FileTooSmall)?;
    let program_header_offset = read_u32(bytes, 28).ok_or(ElfLoadError::FileTooSmall)? as usize;
    let program_header_size = read_u16(bytes, 42).ok_or(ElfLoadError::FileTooSmall)?;
    let program_header_count = read_u16(bytes, 44).ok_or(ElfLoadError::FileTooSmall)?;

    if program_header_size < PROGRAM_HEADER_SIZE as u16 {
        return Err(ElfLoadError::InvalidProgramHeaderSize(program_header_size));
    }

    let mut base_address = u32::MAX;
    let mut end_address = 0_u32;
    let mut found_segment = false;

    for index in 0..usize::from(program_header_count) {
        let header_offset = program_header_offset
            .checked_add(index * usize::from(program_header_size))
            .ok_or(ElfLoadError::AddressOverflow)?;
        let header_end = header_offset
            .checked_add(usize::from(program_header_size))
            .ok_or(ElfLoadError::AddressOverflow)?;

        if header_end > bytes.len() {
            return Err(ElfLoadError::InvalidProgramHeader);
        }

        let program_type = read_u32(bytes, header_offset).ok_or(ElfLoadError::InvalidProgramHeader)?;
        if program_type != PT_LOAD {
            continue;
        }

        let file_offset = read_u32(bytes, header_offset + 4).ok_or(ElfLoadError::InvalidProgramHeader)? as usize;
        let virtual_address = read_u32(bytes, header_offset + 8).ok_or(ElfLoadError::InvalidProgramHeader)?;
        let file_size = read_u32(bytes, header_offset + 16).ok_or(ElfLoadError::InvalidProgramHeader)? as usize;
        let memory_size = read_u32(bytes, header_offset + 20).ok_or(ElfLoadError::InvalidProgramHeader)?;

        if file_size > memory_size {
            return Err(ElfLoadError::SegmentFileSizeExceedsMemory {
                file_size,
                mem_size: memory_size,
            });
        }

        let file_end = file_offset
            .checked_add(file_size as usize)
            .ok_or(ElfLoadError::AddressOverflow)?;
        if file_end > bytes.len() {
            return Err(ElfLoadError::InvalidProgramHeader);
        }

        let segment_end = virtual_address
            .checked_add(memory_size)
            .ok_or(ElfLoadError::AddressOverflow)?;

        base_address = base_address.min(virtual_address);
        end_address = end_address.max(segment_end);
        found_segment = true;
    }

    if !found_segment {
        return Err(ElfLoadError::NoLoadableSegments);
    }

    let image_size = end_address
        .checked_sub(base_address)
        .ok_or(ElfLoadError::AddressOverflow)? as usize;
    let mut memory = vec![0_u8; image_size];

    for index in 0..usize::from(program_header_count) {
        let header_offset = program_header_offset
            .checked_add(index * usize::from(program_header_size))
            .ok_or(ElfLoadError::AddressOverflow)?;
        let program_type = read_u32(bytes, header_offset).ok_or(ElfLoadError::InvalidProgramHeader)?;
        if program_type != PT_LOAD {
            continue;
        }

        let file_offset = read_u32(bytes, header_offset + 4).ok_or(ElfLoadError::InvalidProgramHeader)? as usize;
        let virtual_address = read_u32(bytes, header_offset + 8).ok_or(ElfLoadError::InvalidProgramHeader)?;
        let file_size = read_u32(bytes, header_offset + 16).ok_or(ElfLoadError::InvalidProgramHeader)? as usize;

        let destination_start = virtual_address
            .checked_sub(base_address)
            .ok_or(ElfLoadError::AddressOverflow)? as usize;
        let destination_end = destination_start
            .checked_add(file_size)
            .ok_or(ElfLoadError::AddressOverflow)?;
        let source_end = file_offset
            .checked_add(file_size)
            .ok_or(ElfLoadError::AddressOverflow)?;

        if destination_end > memory.len() || source_end > bytes.len() {
            return Err(ElfLoadError::InvalidProgramHeader);
        }

        memory[destination_start..destination_end].copy_from_slice(&bytes[file_offset..source_end]);
    }

    Ok(LoadedElf {
        entry_point,
        base_address,
        memory,
    })
}

fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    let end = offset.checked_add(2)?;
    let slice = bytes.get(offset..end)?;
    Some(u16::from_le_bytes([slice[0], slice[1]]))
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let slice = bytes.get(offset..end)?;
    Some(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}
