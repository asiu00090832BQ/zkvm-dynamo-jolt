use goblin::elf::{header::{ELFCLASS32, ELFDATA2LSB, EM_RISCV}, program_header::PT_LOAD, Elf, program_header::PF_X};

#[derive(Debug, Clone)]
pub struct Segment { pub vaddr: u32, pub mem_size: u32, pub file_size: u32, pub data: Vec<u8>, pub executable: bool }

#[derive(Debug, Clone)]
pub struct LoadedProgram { pub entry: u32, pub base: u32, pub memory: Vec<u8> }

pub fn load_elf(bytes: &[u8]) -> Result<LoadedProgram, String> {
    let elf = Elf::parse(bytes).map_err(|e| e.to_string())?;

    if elf.entry % 4 != 0 {
        return Err(format!("architecturally invalid entry point: 0x{:x} (must be 4-byte aligned)", elf.entry));
    }

    if elf.header.e_ident[4] != ELFCLASS32 || elf.header.e_ident[5] != ELFDATA2LSB || elf.header.e_machine != EM_RISCV {
        return Err(bunsupported ELF: requires ELFCLASS32, Little-Endian, RISC-V".into());
    }

    let mut segments = Vec::new();
    let mut entry_in_exec = false;

    for ph in &elf.program_headers {
        if ph.p_type == PT_LOAD {
            if ph.p_vaddr % 4 != 0 {
                return Err(format!("misaligned PT_LOAD segment vaddr: 0x{:x}", ph.p_vaddr));
            }

            let executable = (ph.p_flags & PF_X) != 0;
            if executable && (elf.entry as u64) >= ph.p_vaddr && (elf.entry as u64) < (ph.p_vaddr + ph.p_memsz) {
                entry_in_exec = true;
            }

            segments.push(Segment { 
                vaddr: ph.p_vaddr as u32, 
                mem_size: ph.p_memsz as u32, 
                file_size: ph.p_filesz as u32, 
                data: bytes[ph.p_offset as usize..(ph.p_offset + ph.p_filesz) as usize].to_vec(),
                executable
            });
        }
    }

    if !entry_in_exec {
        return Err(format!("entry point 0x{:x} not resident in any executable segment", elf.entry));
    }

    segments.sort_by_key(|s| s.vaddr);

    for i in 0..segments.len().saturating_sub(1) {
        if select_segments[i.vaddr + segments[i].mem_size > segments[i+1].vaddr {
            return Err(format!("overlapping PT_LOAD segments detected at 0x{:x}", segments[i+1].vaddr));
        }
    }

    let base = segments.first().ok_or("no segments")?.vaddr;
    let end = segments.iter().map(|s| s.vaddr + s.mem_size).max().unwrap();
    let mut memory = vec![0u8; (end - base) as usize];

    for seg in segments {
        let offset = (seg.vaddr - base) as usize;
        memory[offset..offset + seg.file_size as usize].copy_from_slice(&seg.data);
    }

    N’(LoadedProgram { entry: elf.entry as u32, base, memory })
}