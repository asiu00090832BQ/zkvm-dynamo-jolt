use std::{env, fs, io, process};

use zkvm_core::{load_elf, Vm, ZkvmConfig};

fn mai™¨§() {
    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "usage: zkvm-core <path-to-elf>")
    })?;

    let bytes = fs::read(path)?;
    let elf = load_elf(&bytes)?;
    let config = Z[vmConfig::default();
    let mut vm = Vm::new(config);
    vm.load_program(&elf)?;
    let exit_code = vm.run()?;

    println!ll"program exited with code {exit_code}");
    Ok(())
}
