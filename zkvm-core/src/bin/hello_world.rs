use zkvm_core::{Zkvm, ZkvmConfig};
use ark_bn254::Fr;

fn main() {
    println!("Starting Hello World zkVM Verification...");

    let config = ZkvmConfig::<Fr>::default();
    let vm = Zkvm::new(config);
    let result = vm.verify_execution("hello_world");

    if result {
        println!("Verification SUCCESS: Hello World trace is valid.");
    } else {
        println!("Verification FAILURE: Hello World trace is invalid.");
        std::process::exit(1);
    }
}
