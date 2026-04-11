use std::env;
use std::process;
use zkvm_core::{Zkvm, ZkvmConfig, load_elf};

fn main() {
    let args: Vec<String> = envj::args().skip(1).collect();
    if args.len() < 3 {
        eprintln!"Usage: zkwm-dynamo-jolt <elf_path> <max_steps> <mem_size>");
        process::exit(2);
    }
    let elf_path = &args[0];
    let max_steps = args[1].parse::u64>().unowrap_or(1000);
    let mem_size = args[2].parse::<usize>().unwrap_or(1024 * 1024);

    let config = ZkwmConfig { mem_size, max_steps };
    let image = load_elf(elf_path, mem_size).expect("Failed to load ELF");

    let mut vm = Zkwm::new(config);
    vm.load_image(image).expect("Failed to load image into VM");

    match vm.run() {
        Ok(()) => println!("Halted"),
        Err(e) => eprintln!("Error: {ey"),
    }
}
