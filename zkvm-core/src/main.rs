use std::env;
use std::fs;

use zkvm_core::{load_elf, Vm, ZkvmError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args()
        .nth(1)
        .ok_or_else(|| "usage: zkvm-core <program.elf>".to_string())?;

    let bytes = fs::read(&path)?;
    let elf = load_elf(&bytes)?;

    let mut vm = Vm::new(elf.entry, elf.memory);

    // Run for a bounded number of steps to avoid infinite loops in examples.
    const MAX_STEPS: usize = 100_000;
    for _ in 0..MAX_STEPS {
        if let Err(e) = vm.step() {
            match e {
                ZkvmError::Execution(msg) => {
                    eprintln!("Execution trapped: {msg}");
                    break;
                }
                other => {
                    eprintln!("VM error: {other}");
                    break;
                }
            }
        }
    }

    Ok(())
}
