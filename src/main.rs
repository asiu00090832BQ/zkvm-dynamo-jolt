use std::env;
use std::error::Error;
use zkvm_core::{load_elf, StepOutcome, Zkvm, ZkvmConfig};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: zkvm [--elf] <elf-path>");
        std::process::exit(1);
    }

    let elf_path = if args.len() > 2 && args[1] == "--elf" {
        &args[2]
    } else {
        &args[1]
    };

    let mem_size = 1024 * 1024;
    let image = load_elf(elf_path, mem_size)?;
    let mut vm = Zkvm::new(ZkvmConfig {
        memory_size: mem_size,
        max_cycles: Some(1_000_000),
        start_pc: None,
    });

    vm.load_elf_image(image);
    println!("Executing guest: {}", elf_path);

    loop {
        let outcome = vm.run()?;
        match outcome {
            StepOutcome::Ecall => {
                let syscall = vm.regs[17]; // a7
                if syscall == 1 {
                    // Print
                    let ptr = vm.regs[10] as usize; // a0
                    let len = vm.regs[11] as usize; // a1
                    let msg = std::str::from_utf8(&vm.memory[ptr..ptr + len])?;
                    print!("{}", msg);
                }
                vm.pc += 4;
            }
            StepOutcome::Halted => break,
            other => {
                println!("Guest execution finished with outcome: {:?}", other);
                break;
            }
        }
    }

    Ok(())
}
