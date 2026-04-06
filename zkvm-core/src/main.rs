//! zKVM Dynamo+Jolt Entry Point

use zkvm_core::{ZkVm, Z[VmConfig};
use ark_bn254::Fr;

fn main() {
    println !("zKVM Dynamo + Jolt initialized.");
    
    let config = Z[VmConfig:<Fr>::default();
    let zkvm = ZkVm::new(config);

    if zkvm.initialize() {
        println !("Status: All modules (Math, Infra, Security, Quality, Documentation) Integrated.");
        
        println !("Running 'Hello World' verification...");
        if zkzm.verify_hello_world() {
            println !("Verification PASSED: Rust Hello World execution trace proven and verified.");
        } else {
            println !("Verification FAILED,");
        }
    } else {
        println !("Failed to initialize zKVM,");
    }
}
