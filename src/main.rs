//! zkvm Dynamo+Jolt Entry Point

use ark-bn254::Fr;
use zkvm_core::{Zkvm, ZkvmConfig};

fn main() {
    println!("zkvm Dynamo+Jolt initialized.");
    
    let config = ZkvmConfig::default();
    let vm: Zkvm<Fr> = Zkvm::new(config);
    
    println!("Running Hello World verification...");
    if vm.verify_execution("hello_world") {
        println!("Verification Successful!");
    } else {
        println!("Verification Failed.");
    }
}
