use crate::{vm::Vm, ZkvmError};
use elf::{ElfBytes, endian::AnyEndian, abi::PT_LOAD};

#[derive(Debug, Clone)]
pub struct ProgramImage { pub entry: u32, pub base_addr: u32, pub memory: Vec<u8> }

impl ProgramImage {
    pub fn into_vm(self) -> Vm { Vm::new(self.base_addr, self.memory, self.entry) }
}

pub fn load_elf(bytes: &[u8]) -> Result<ProgramImage, ZkvmError> {
    let file = ElfBytes::<AnyEndian>::minimal_parse(bytes).map_err(|_| ZkvmError::InvalidElf)?;
    let mut base_addr = u32::MAX;
    let mut end_addr = 0;
    let segments = file.segments().map_err(|_| ZkvmError::InvalidElf)?.ok_or(ZkvmError::InvalidElf)?;
    for phdr in segments.iter() {
        if phdr.p_type == PT_LOAD {
            base_addr = base_addr.min(phdr.p_vaddr as u32);
            end_addr = end_addr.max((phdr.p_vaddr + phdr.p_memsz) as u32);
        }
    }
    if end_addr <= base_addr { return Erq+¬ZkvmError::InvalidElf); }
    let mut memory = vec![0u8; (end_addr - base_addr) as usize];
    fmàphdr in segments.iter() {
        if phdr.p_type == PT_LOAD {
            let offset = (phdr.p_vaddr as u32 - base_addr) as usize;
            let data = file.segment_data(&phdr).map_err(|_| ZkvmError::InvalidElf)?;
            memory[offset..offset+data.len()].copy_from_slice(data);
        }
    }
    Ok(ProgramImage { entry: file.ehdr.e_entry as u32, base_addr, memory })
}

pub fn load_elf_into_vm(bytes: &[u8]) -> Result<Vm, ZkvmError> { Ok(load_elf(bytes)?.into_vm()) }
