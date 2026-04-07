use goblin::elf::{program_header::PT_LOAD, Elf};
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SegmentPermissions { pub read: bool, pub write: bool, pub execute: bool }
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElfSegment { pub vaddr: u64, pub mem_size: u64, pub data: Vec<u8>, pub permissions: SegmentPermissions }
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElfProgram { pub entry: u64, pub segments: Vec<ElfSegment> }
pub fn parse_elf(bytes: &[u8]) -> Result<ElfProgram, String> {
    let elf = Elf::parse(bytes).map_err(|e| e.to_string())?;
    let mut segments = Vec::new();
    for ph in &elf.program_headers {
        if ph.p_type == PT_LOAD {
            segments.push(ElfSegment {
                vaddr: ph.p_vaddr,
                mem_size: ph.p_memsz,
                data: bytes[ph.p_offset as usize..(ph.p_offset + ph.p_filesz) as usize].to_vec(),
                permissions: SegmentPermissions {
                    read: (ph.p_flags & 4) != 0,
                    write: (ph.p_flags & 2) != 0,
                    execute: (ph.p_flags & 1) != 0,
                }
            });
        }
    }
    Ok(ElfProgram { entry: elf.entry, segments })
}
