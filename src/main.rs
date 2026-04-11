use ark_bn254::Fr;
use zkvm_core::{Zkvm, ZkvmConfig};

fn main() {
    let config = ZkvmConfig::default();
    let vm = Zkvm::<Fr>::new(config);

    println!("memory size: {} bytes", vm.memory().len());
}
