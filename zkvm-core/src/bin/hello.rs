use ark_ff::{Fp64, MontBackend, MontConfig};
use zkvm_core::{Zkvm, ZkvmConfig};

#[derive(MontConfig)]
#[modulus = "18446744069414584321"]
#[generator = "7"]
pub struct MyConfig;

type Fr = Fp64<MontBackend<MyConfig, 1>>;

fn main() {
    println!("--- STANDALONE ZKVM HELLO WORLD ---");

    let config = ZkvmConfig::<Fr>::default();
    let vm = Zkvm::new(config);
    let _ = vm.initialize();

    let trace_name = "hello_world";
    println!("Verifying trace: '{trace_name}'...");
    let success = vm.verify_execution(trace_name);

    if success {
        println!("VERIFICATION SUCCESS: '{trace_name}' is proven correct.");
    } else {
        eprintln!("VERIFICATION FAILURE: '{trace_name}' is invalid.");
        std::process::exit(1);
    }
}