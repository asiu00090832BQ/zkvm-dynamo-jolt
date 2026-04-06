use ark_bn254::Fr;
use ark_ff::PrimeField;
use zkvm_core::{Zkvm, ZkvmConfig};

fn main() {
    let config = ZkvmConfig::default();
    let zkvm = Zkvm::<Fr>::new(config);

    println!("Hello from Zkvm!");
    println!("Field modulus bits: {}", <Fr as PrimeField>::MODULUS_BIT_SIZE);
    println!("Max cycles: {}", zkvm.config.max_cycles);
}
