use std::env;
use std::error::Error;

use zkvm_core::{load_elf, Zkvm, ZkvmConfig};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| String::from("zkvm"));

    let elf_path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("usage: {program} <elf-path>");
            std::process::exit(1);
        }
    };

    let mem_size = 1024 * 1024;
    let image = load_elf(&elf_path, mem_size)?;
    let mut vm = Zkvm::new(ZkvmConfig {
        mem_size,
        max_steps: 1_000_000,
    });

    vm.load_image(image.memory.as_ref())?;
    let halt_reason = vm.run()?;
    println!("halted: {:?}", halt_reason);

    Ok(())
}