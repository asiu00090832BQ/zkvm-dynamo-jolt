//! zkvm Dynamo+Jolt Entry Point

use ark_bn254::Fr;
use zkvm_core::{Zkvm, ZkvmConfig};

fn main() {
    println!("zkvm Dynamo+Jolt initialized.");

    let config = ZkvmConfig::default();
    let mut vm: Zkvm<Fr> = Zkvm::new(config);

    println!("Running verification pass...");
    // Simulating ELF bytes for demonstration
    let mock_elf = vec![0u8; 64];
    if let Err(e) = vm.load_elf_bytes(&mock_elf) {
        println!("Load failed: {:?}", e);
    }

    match vm.step() {
        Ok(_) => println!("Step Successful!"),
        Err(e) => println!("Step Failed: {:?}", e),
    }
}
