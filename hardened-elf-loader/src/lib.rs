pub struct Segment { pub vaddr: u32, pub mem_size: u32, pub file_size: u32, pub data: Vec<u8>, pub executable: bool }
pub struct LoadedProgram { pub entry: u32, pub base: u32, pub memory: Vec<u8> }
pub fn load_elf(_bytes: &[u8], _memory_size: usize) -> Result<LoadedProgram, String> { Err("not implemented".into()) }
