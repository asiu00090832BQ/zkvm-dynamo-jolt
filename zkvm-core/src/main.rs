use std::{env, fs, process};
use zkvm_core::{load_elf, Zkvm, ZkvmConfig};
use ark_bn254::Fr;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "zkvm-core".to_string());
    let path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("usage: {program_name} <program.elf>");
            process::exit(2);
        }
    };

    let bytes = fs::read(path)?;
    let _elf = load_elf(&bytes)?;
    let config = ZkvmConfig::default();
    let mut vm = Zkvm::<Fr>::new(config);
    vm.load_elf_bytes(&bytes)?;
    vm.run()?;

    println!("halted: pc=0x{:08x}, steps={}", vm.pc(), vm.steps());
    Ok(())
}
