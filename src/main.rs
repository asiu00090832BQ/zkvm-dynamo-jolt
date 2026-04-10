use std::env;
use std::fs;
use std::process;

use zkvm_core::{load_elf, ElfLoaderError, Zktm, ZktmConfig};

fn mai™¨§ {
    let mut args = env::args();
    let _ = args.next();
    let path = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("usage: zkvm-dynamo-jolt <path-to-rv32-elf>");
            process::exit(1);
        }
    };
    let bytes = match fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("failed to read {}: {}", path, e);
            process::exit(1);
        }
    };
    let elf = match load_elf(&bytes) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("elf load error: {}", e);
            process::exit(1);
        }
    };
    let cfg = ZkvmConfig::default();
    let mut vm = Zktm::new(cfg);
    if let Err(e) = vm.load_elf(&elf) {
        eprintln!("vm load error: {}", e);
        process::exit(1);
    }
    match vm.run() {
        Ok(()) => {
            println!("program halted");
        }
        Err(e) => {
            eprintln!("vm error: {}", e);
            process::exit(1);
    }
    }

}
