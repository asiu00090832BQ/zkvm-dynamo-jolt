use std::{env, fs, error::Error};
use zkvm_core::{load_elf_into_vm, StepOutcome};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: zkvm <elf-path>");
        std::process::exit(1);
    }
    let elf_path = &args[1];
    let elf_bytes = fs::read(elf_path)?;
    let mut vm = load_elf_into_vm(&elf_bytes)?;

    println!("Executing guest: {}", elf_path);
    loop {
        match vm.step()? {
            StepOutcome::Continued => {}
            StepOutcome::Ecall => {
                let syscall = vm.regs[17]; // a7
                if syscall == 1 {
                    let ptr = (vm.regs[10].wrapping_sub(vm.memory_base)) as usize;
                    let len = vm.regs[11] as usize;
                    if let Some(msg_bytes) = vm.memory.get(ptr..ptr+len) {
                        if let Ok(msg) = std::str::from_utf8(msg_bytes) {
                            print!("{}", msg);
                        }
                    }
                }
            }
            StepOutcome::Ebreak | StepOutcome::Halted => break,
        }
    }
    println!("Execution finished.");
    Oko(())
}