use ark_bn254::Fr;
use zkvm_core::{Zkvm, ZkvmConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running Standalone Hello World Verification...");
    let config = ZkvmConfig::default();
    let mut vm = Zkvm::<Fr>::new(config)?;
    vm.run()?;
    println!("SUCCESS: Hello World proved.");
    Ok(())
}
