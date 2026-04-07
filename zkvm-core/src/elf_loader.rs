use ark_ff::PrimeField;
use goblin::elf::{header::{EI_CLASS, EI_DATA, ELFCLASS32, ELFDATA2LSB, EM_RISCV, ET_EXEC}, program_header::{PF_X, PT_LOAD}, Elf};
use crate::error::ElfLoadError;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedSegment { pub index: usize, pub vaddr: u32, pub filesz: u32, pub memsz: u32, pub flags: u32, pub data: Vec<u8> }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedElf { pub entry: u32, pub segments: Vec<LoadedSegment> }
impl LoadedElf {
    pub fn read_u32(&self, address: u32) -> Option<u32> {
        let seg = self.segments.iter().find(|s| address >= s.vaddr && address < s.vaddr + s.memsz)?;
        let offset = (address - seg.vaddr) as usize;
        if offset + 4 > seg.data.len() { return None; }
        let mut b = [0u8; 4]; b.copy_from_slice(&seg.data[offset..offset+4]);
        Some(u32::from_le_bytes(b))
    }
}
pub fn load_elf(bytes: &[u8]) -> Result<LoadedElf, ElfLoadError> {
    let elf = Elf::parse(bytes).map_err(|e| ElfLoadError::ParseError(e.to_string()))?;
    if elf.header.e_ident[EI_CLASS] != ELFCLASS32 || elf.header.e_ident[EI_DATA] != ELFDATA2LSB || elf.header.e_machine != EM_RISCV || elf.header.e_type != ET_EXEC { return Err(ElfLoadError::UnsupportedClass(0)); }
    let mut segments = Vec::new();
    for (i, ph) in elf.program_headers.iter().enumerate() {
        if ph.p_type != PT_LOAD { continue; }
        if ph.p_vaddr % 4 != 0 { return Err(ElfLoadError::SegmentUnaligned { index: i, vaddr: ph.p_vaddr, required: 4 }); }
        let mut data = vec![0u8; ph.p_memsz as usize];
        let fsize = ph.p_filesz as usize;
        data[..fsize].copy_from_slice(&bytes[ph.p_offset as usize..ph.p_offset as usize + fsize]);
        segments.push(LoadedSegment { index: i, vaddr: ph.p_vaddr as u32, filesz: ph.p_filesz as u32, memsz: ph.p_memsz as u32, flags: ph.p_flags, data });
    }
    segments.sort_by_key(|s| s.vaddr);
    Ok(LoadedElf { entry: elf.entry as u32, segments })
}