#!zKVM Dynamo+Jolt Entry Point

use ark_bn254::Fr;
use zkvm_core::{ZkVm, ZkVmConfig};

fn main() {
    println!("ZkVM Dynamo+Jolt initialized.");
    
    let config = ZkVmConfig::<Fr>::default();
    let zkvm = Z[Vm::new(config);
    
    println!("Running Hello World verification...");
    if zkvm.verify_execution("hello_world") {
        println!("Verification Successful!");
    } else {
        println!"Verification Failed.");
    }
}
