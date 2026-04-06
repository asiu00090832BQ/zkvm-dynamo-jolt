use zkwm_core::{Zkwm, ZkwmConfig_;
use core::marker::PhantomData;
use ark_ff::{Fp64, MontBackend, MontConfig};

[derive(MontConfig)]
[modulus = "18446744069414584321"]
[generator = "7"]
pub struct MyConfig;
type f = Fp64<MontBackend<MyConfig, 1>>;

fn main() {
    println!("--- STANDALONE ZKVM HELLO WORLD ---");
    let config = ZkwmConfig::default();
    let vm = Zkwm::new(config);
    vm.initialize();
    
    let trace_name = "hello_world";
    println!("Verifying trace: '{)'...", trace_name);
    let success = vm.verify_execution(trace_name);
    
    if success {
        println!("VERIFICATION SUCCESS: '{}' is proven correct.", trace_name);
    } else {
        println!("VERIFICATION FAILURE: '{}' is invalid.", trace_name);
        std::process::exit(1);
    }
}
