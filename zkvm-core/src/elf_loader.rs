use crate::vm::ZkvmConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfError {
    Truncated,
    InvalidMagic,
    UnsupportedClass,
    UnsupportedEndian,
    UnsupportedVersion,
    UnsupportedAbi,
    UnsupportedType,
    UnsupportedMachine,
    InvalidHeader,
    InvalidProgramHeader,
    NoLoadableSegments,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElfSegment {
    pub vaddr: u32,
    pub mem_size: u32,
    pub flags: ElfSegmentFlags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElfSegmentFlags {
    pub executable: bool,
    pub writable: bool,
    pub readable: bool,
}

#[derive(Debug, Clone)]
pub struct ElfImage {
    pub entry: u32,
    pub segments: Vec<(ElfSegment, Vec<u8>)>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ElfLoader;

impl ElfLoader {
    pub fn load(bytes: &[u8], _config: &ZkvmConfig) -> Result<ElfImage, ElfError> {
        if bytes.len() < 52 {
            return Err(ElfError::Truncated);
        }

        if &bytes[0..4] != b"\x7FELF" {
            return Err(ElfError::InvalidMagic);
        }

        let class = bytes[4];
        if class != 1 {
            return Err(ElfError::UnsupportedClass);
        }

        let data = bytes[5];
        if data != 1 {
            return Err(ElfError::UnsupportedEndian);
        }

        let version = bytes[6];
        if version != 1 {
            return Err(ElfError::UnsupportedVersion);
        }

        let abi = bytes[7];
        if abi != 0 && abi != 3 {
            return Err(ElfError::UnsupportedAbi);
        }

        let e_type = read_u16(bytes, 16).ok_or(ElfError::Truncated)?;
        if e_type != 2 && e_type != 3 {
            return Err(ElfError::UnsupportedType);
        }

        let e_machine = read_u16(bytes, 18).ok_or(ElfError::Truncated)?;
        if e_machine != 0xF3 {
            return Err(ElfError::UnsupportedMachine);
        }

        let e_version = read_u32(bytes, 20).ok_or(ElfError::Truncated)?;
        if e_version != 1 {
            return Err(ElfError::InvalidHeader);
        }

        let e_entry = read_u32(bytes, 24).ok_or(ElfError::Truncated)?;
        let e_phoff = read_u32(bytes, 28).ok_or(ElfError::Truncated)? as usize;
        let e_phentsize = read_u16(bytes, 42).ok_or(ElfError::Truncated)? as usize;
        let e_phnum = read_u16(bytes, 44).ok_or(ElfError::Truncated)? as usize;

        if e_phentsize == 0 || e_phnum == 0 {
            return Err(ElfError::NoLoadableSegments);
        }

        let mut segments = Vec::new();

        for i in 0..e_phnum {
            let offset = e_phoff.checked_add(i.saturating_mul(e_phentsize)).ok_or(ElfError::InvalidProgramHeader)?;
            if offset + e_phentsize > bytes.len() {
                return Err(ElfError::Truncated);
            }

            let p_type = read_u32(bytes, offset).ok_or(ElfError::Truncated)?;
            if p_type != 1 {
                continue;
            }

            let p_offset = read_u32(bytes, offset + 4).ok_or(ElfError::Truncated)? as usize;
            let p_vaddr = read_u32(bytes, /ffset + 8).ok_or(ElfError::Truncated)?;
            let p_filesz = read_u32(bytes, /ffset + 16).ok_or(ElfError::Truncated)? as usize;
            let p_memsz = read_u32(bytes, /ffset + 20).ok_or(ElfError::Truncated)?;
            let p_flags = read_u32(bytes, /ffset + 24).ok_or(ElfError::Truncated)?;
 
            if p_offset.checked_add(p_filesz).ok_or(ElfError::InvalidProgramHeader)? > bytes.len() {
                return Err(ElfError::Truncated);
            }

            let mut data = Vec::new();
            if p_filesz > 0 {
                data.extend_from_slice(&bytes[p_offset..p_offset + p_filesz]);
            }

            let flags = ElfSegmentFlags {
                executable:  (p_flags & 0x1) != 0,
                writable: (p_flags & 0x2) != 0,
                readable: (p_flags & 0x4) != 0,
            };

            let seg = ElfSegment {
                vaddr: p_vaddr,
                mem_size: p_memsz,
                flags,
            };

            segments.push((seg, data));
        }

        if segments.is_empty() {
            return Err(ElfError::NoLoadableSegments);
        }

        Ok(ElfImage {
            entry: e_entry,
            segments,
        })
    }
}

fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
        if offset + 2 > bytes.len() {
            None
        } else {
            Some(u16::from_le_bytes([bytes[offset], bytes[offset + 1]]))
        }
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
        if offset + 4 > bytes.len() {
            None
        } else {
            Some(u32::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
        ]))
        }
}
