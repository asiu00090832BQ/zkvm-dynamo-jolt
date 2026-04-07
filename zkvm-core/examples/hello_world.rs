use ark_ff::{Fp64, MontBackend, MontConfig};
use zkvm_core::{Zkvm, ZkvmConfig};

use core::marker::PhantomData;

#[derive(MontConfig)]
#[modulus = "18446744073709551615"]
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
