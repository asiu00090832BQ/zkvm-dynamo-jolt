use std::env;
use std::fs;
use std::path::PathBuf;

use zkvm_core::{load_elf, RunStats, VmError, Zkvm, ZkvmConfig};

fn read_program(path: &str) -> Result<Vec<u8>, String> {
    let p = PathBuf::from(path);
    fs::read(&p).map_err(|e| format!("failed to read {}: {}", path, e))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <program.elf|raw>", args[0]);
        std::process::exit(2);
    }

    let program = match read_program(&args[1]) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(2);
        }
    };

    let load = match load_elf(&program) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("ELF load error: {}", e);
            std::process::exit(2);
        }
    };

    let cfg = ZkvmConfig {
        name: "Zkvm".to_string(),
        start_pc: load.entry_pc,
        max_cycles: 1_000_000,
        max_steps: 1_000_000,
    };

    let mut vm = match Zkvm::new(cfg, load.memory) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("init error: {}", e);
            std::process::exit(2);
        }
    };

    let stats = match vm.run() {
        Ok(s) => s,
        Err(VmError::InvalidInstruction(raw)) => {
            eprintln!("invalid instruction: 0x{:08x}", raw);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("vm error: {}", e);
            std::process::exit(1);
        }
    };

    println!("steps: {}", stats.steps);
    println!("cycles: {}", stats.cycles);
    println!("halted: {}", stats.halted);
    println!("exit_code: {}", stats.exit_code);
    println!("outcome: {}", stats.outcome);
}