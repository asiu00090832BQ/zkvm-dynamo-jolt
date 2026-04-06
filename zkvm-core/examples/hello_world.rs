use zkvm_core::{ZkVm, ZkVmConfig};
use core::marker::PhantomData;
use ark_ff::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "18446744069414584321"]
#[generator = "7"]
pub struct MyConfig;
type F = Fp64<MontBackend<MyConfig, 1>>;

fn main() {
    println!("Running Standalone Hello World Verification...");
    let config = ZkVmConfig::<F> {
        trace_length: 1024,
        marker: PhantomData,
    };
    let vm = ZkVm::new(config);
    if vm.verify_execution("hello_world") {
        println!("SUCCESS: Hello World proved.");
    } else {
        println!("FAILURE.");
    }
}
