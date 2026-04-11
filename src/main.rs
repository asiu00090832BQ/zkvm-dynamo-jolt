use std::env;
use std::error::Error;
use std::fs;
use zkvm_core::{load_elf, Zkvm, ZkvmConfig};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let _ = args.next();
    let elf_path = args.next().expect("usage: zkvm <elf-path>");
    let mem_size = 1024 * 1024;
    let image = load_elf(elf_path, mem_size)?;
    let mut vm = Zkvm::new(ZkvmConfig {
        memory_size: mem_size,
        max_cycles: Some(1_000_000),
        start_pc: None,
    });
    vm.load_elf_image(image);
    Ok(())
}
