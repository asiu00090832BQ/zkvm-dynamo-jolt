use ark_bn254::Fr;
use zkvm_core::{Zkvm, ZkvmConfig, Result};

fn main() -> Result<()> {
    // Initialize configuration with security-hardened defaults
    let config = ZkvmConfig::default();

    // Instantiate the Zkvm with the verified PrimeField baseline
    let mut vm = Zkvm::<Fr>::new(config)?;

    // Load the ELF image using the hardened ingestion path
    // This triggers the alignment and overlap validation logic in elf_loader.rs
    let elf_bytes = include_bytes!("../examples/hello_world.elf");
    vm.load_elf(elf_bytes)?;

    // Execute the program until halt or trap
    vm.run()?;

    println!("Execution verified. Total cycles: {}", vm.cycle_count());
    Ok(())
}
