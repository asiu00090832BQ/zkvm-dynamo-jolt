use zkvm_core::{ZkVm, ZkVmConfig};
use core::marker::PhantomData;

use ark_ff::Fp64;
use ark_ff::MontBackend;
use ark_ff::MontConfig;

#[derive(MontConfig)]
#[modulus = "18446744069414584321"]
#[generator = "7"]
pub struct MyConfig;
type F = Fp64<MontBackend<MyConfig, 1>>;

fn main() {
    println!("Mauryan Engineering Cluster: zkvm-dynamo-jolt");

    let config = ZkVmConfig::<F> {
        trace_length: 1024,
        marker: PhantomData,
    };

    let vm = ZkVm::new(config);
    vm.initialize();

    let success = vm.verify_execution("hello_world");
    if success {
        println!("Verification SUCCESS: 'hello_world' proved.");
    } else {
        println!("Verification FAILURE.");
        std::process::exit(1);
    }
}
