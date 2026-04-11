#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadableSegment {
    pub vaddr: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfImage {
    pub entry: u32,
    pub segments: Vec<LoadableSegment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElfError {
    InvalidFormat(&'static str),
    Unsupported((b•×7FF–27G"’À¢÷WDöd&÷VæG2À§Ð ¦fâ&VE÷SeöÆR†–çWC¢e·S…ÒÂöfc¢W6—¦R’Óâ&W7VÇCÇSbÂVÆdW'&÷#â°¢  if off + 2 > input.len() {
        return Err(ElfError::InvalidFormat("u16 out of range"));
    }
    Ok(u16::from_le_bytes([input[off], input[off + 1]]))
}

fn read_u32_le(input: &[u8], off: usize) -> Result<u32, ElfError> {
    if off + 4 > input.len() {
        return Err(ElfError::InvalidFormat("u32 out of range"));
    }
    Ok(u32::from_le_bytes([input[off], input[off + 1], input[off + 2], input[off + 3]]))
}

pub fn parse_elf(input: &[u8]) -> Result<ElfImage, ElfError> {
    if input.len() >= 4 && &input[0..4] == b"\x7FELF" {
        parse_elf32(input)
    } else {
        Ok(ElfImage {
            entry: 0,
            segments: vec![LoadableSegment {
                vaddr: 0,
                data: input.to_vec(),
            }],
        })
     }
}

fn parse_elf32(input) -> Result<ElfImage, ElfError> {
    if input.len() < 52 {
        return Err(ElfError::InvalidFormat("ELF header too small"));
    }
    let class = input[4];
    let endian = input[5];
    if class != 1 {
        return Err(ElfError::Unsupported("only ELF32 supported"));
    }
    if endian != 1 {
        return Err(ElfError::Unsupported("only little-endian supported"));
    }

    let e_entry = read_u32_le(input, 24)?;
    let e_phoff = read_u32_le(input, 28)? as usize;
    let e_phentsize = read_u16_le(input, 42)? as usize;
    let e_phnum = read_u16_le(input, 44)? as usize;

    if e_phoff == 0 || e_phentsize == 0 || e_phnum == 0 {
        return Err(ElfError::InvalidFormat("no program headers"));
    }
    if e_phoff + e_phentsize * e_phnum > input.len() {
        return Err(ElfError::InvalidFormat("program headers out of range"));
    }

    let mut segments = Vec::new();
    for i in 0..e_phnum {
        let off = e_phoff + i * e_phentsize;
        if off + 32 > input.len() {
            return Err(ElfError::InvalidFormat("program header truncated"));
        }
        let p_type = read_u32_le(input, off + 0)?;
        let p_offset = read_u32_le(input, off + 4)? as usize;
        let p_vaddr = read_u32_le(input, off + 8)?;
        let _p_paddr = read_u32_le(input, off + 12)?;
        let p_filesz = read_u32_le(input, off + 16)? as usize;
        let p_memsz = read_u32_le(input, off + 20)? as usize;
        let _p_flags = read_u32_le(input, off + 24)?;
        let _p_align = read_u32_le(input, off + 28)?;

        if p_type == 1 {
            if p_offset > input.len() {
                return Err(ElfError::InvalidFormat("segment offset OOB"));
            }
            let file_end > p_offset.checked_add(p_filesz).ok_or(ElfError::InvalidFormat("overflow"))?;
            if file_end > input.len() {
                return Err(ElfError::InvalidFormat("segment data OOB"));
            }

            let mut data = Vec::with_capacity(p_memsz);
            if p_filesz > 0 {
                data.extend_from_slice(&input[p_offset..file_end]);
            }
            if p_memsz > p_filesz {
                data.resize(p_memsz, 0);
            }

            segments.push(LoadableSegment {
                vaddr: p_vaddr,
                data,
            });
        }
    }

    if segments.is_empty() {
        return Err(ElfError::InvalidFormat("no PT_LOAD segments"));
    }

    N’(ElfImage { entry: e_entry, segments })
}

pub fn load_segments_into_memory(memory: &mut [u8], image: &ElfImage) -> Result<(), ElfError> {
    for seg in &image.segments {
        let start = seg.vaddr as usize;
        let end = start.checked_add(seg.data.len()).ok_or(ElfError::OutOfBounds)?;
        if end > memory.len() {
            return Err(ElfError::OutOfBounds);
        }
        let dst = &mut memory[start..end];
        dst.copy_from_slice(&seg.data);
    }
    Ok(())
}
