#![forbid(unsafe_code)]

use std::path::PathBuf;
use zkvm_core::{Zkvm, ZkvmConfig};

fn main() {
    let mut args = std::env::args_os();
    let _exe = args.next();
    let elf_path: PathBuf = match args.next() {
        Some(p) => p.into(),
        None => {
            eprintln!(\"usage: zkvm <path-to-riscv32-elf>\");
            std::process::exit(2);
        }
    };

    let elf_bytes = match std::fs::read(&elf_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!(\"failed to read {}: {}\", elf_path.display(), e);
            std::process::exit(2);
        }
    };

    let cfg = ZkvmConfig::default();
    let mut vm: Zkvm<ark_bn254::Fr> = Zkvm::new(cfg);

    if let Err(e) = vm.load_elf(&elf_bytes) {
        eprintln!(\"failed to load image: {:?}\", e);
        std::process::exit(2);
    }

    if let Err(e) = vm.run() {
        eprintln!(\"vm error: {:?}\", e);
        std::process::exit(1);
    }

    println!(\"halted at pc={:#x} after {} steps\", vm.pc(), vm.steps());
}