use ark_bn254::Fr;
use zkvm_core::{Zkvm, ZkvmConfig};

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("zkvm Dynamo+Jolt initialized.");

    let config = ZkvmConfig::default();
    config.validate()?;

    let mut vm = Zkvm::<Fr>::new(config);

    // Simulation: load placeholder bytes
    let elf_bytes = vec![0u8; 100];
    vm.load_elf_bytes(&elf_bytes)?;

    println!("Running simulation step...");
    vm.step()?;

    println!("Execution Successful!");
    Ok(())
}