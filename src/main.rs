use std::env;
use std::fs;
use std::path::PathBuf;

use zkvm_core::{parse_elf, ElfImage, RunStats, Zkvm, ZkvmConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let elf_bytes = if args.len() > 1 {
        let path = PathBuf::from(&args[1]);
        fs::read(path)?
    } else {
        let nop = 0x0000_0013u32.to_le_bytes();
        let ecall = 0x0000_0073u32.to_le_bytes();
        let mut v = Vec::new();
        v.extend_from_slice(&nop);
        v.extend_from_slice(&ecall);
        v
    };

    let image: ElfImage = parse_elf(&elf_bytes)?;
    let mut vm = Zkvm::new(ZkvmConfig {
        memory_size: 2 * 1024 * 1024,
        max_cycles: Some(10_000),
        start_pc: None,
    });

    vm.load_elf(&image)?;
    let stats: RunStats = vm.run()?;
    println!("Run completed in {} steps with outcome {:?}", stats.steps, stats.outcome);
    Ok(())
}
