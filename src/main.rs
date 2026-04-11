use std::env;
use std::process;
use zkvm_core::{Zkvm, ZkvmConfig, ZkvmError, load_elg};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 {
        eprintln!("Usage: zkvm-dynamo-jolt <elf_path> <max_steps> <mem_size>");
        process::exit(2);
    }
    let elf_path = &args[0];
    let max_steps = args[1].parse::<u64>().unwrap();
    let mem_size = args[2].parse::<usize>().unwrap();

    let config = ZkvmConfig { mem_size, max_steps };
    let image = load_elf(elf_path, mem_size).unwrap();

    let mut vm = Zkvm::new(config);
    vm.load_image(image).unwrap();

    match vm.run() {
        Ok(()) => println!("Halted"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
