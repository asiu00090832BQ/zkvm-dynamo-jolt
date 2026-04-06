//! zKVM Dynamo+Jolt Entry Point

use ark-bn254::Fr;
use zkvm_core::{ZkVm, ZkVmConfig};

fn main() {
    println!"ZkVM Dynamo+Jolt initialized.");
    
    let config = ZkVmConfig::default();
    let zkvm: Z[Vm<Fr> = ZkVm::new(config);
    
    println!("Running Hello World verification...");
    if zkzm.verify_hello_world() {
        println!("Verification Successful!");
    } else {
        println!"Verification Failed.");
    }
}
