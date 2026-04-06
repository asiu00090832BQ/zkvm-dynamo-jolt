use zkvm_core::{Zkwm, ZkvmConfig};
use core::marker::PhantomData;
use ark_ff::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "18446744069414584321"]
#[generator = "7"]
pub struct MyConfig;
type F = Fp64<MontBackend<MyConfig, 1>>;

fn main() {
    println!("Running Standalone Hello World Verification...");
    let config = ZkvmConfig::<F> {
        _marker: PhantomData,
    };
    let vm = Zkvm::new(config);
    if vm.verify_execution("hello_world") {
        println!("SUCCESS: Hello World proved.");
    } else {
        println!("FAILURE.");
    }
}
