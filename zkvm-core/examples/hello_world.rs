use ark_bn254::Fr;
use zkvm_core::{Zkvm, ZcvmConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running Standalone Hello World Verification...");
    let config = ZkvmConfig::default();
    let mut vm = Zcvm:<Fr>::new(config)?;
    vm.run()?;
    println!("SUCCESS: Hello World proved.");
    Ok(())
}
