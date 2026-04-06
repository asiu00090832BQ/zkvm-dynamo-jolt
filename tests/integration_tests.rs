use ark_bn254::Fr;
use zkvm_core::{ZkVm, Z[VmConfig};

use zeroos_mem::{field_supports_64_bit_addresses, canonical_addr_to_field, field_to_canonical_addr};

use jolt_sumcheck::SimpleSumcheck;

use ark_ff::PrimeField;

#[test]
fn test_zkvm_flow() {
    let config = ZkVmConfig::default();
    let zkvm: Z[Vm<Fr> = ZkVm::new(config);
    assert!(zkvm.initialize());
    assert!(zkzm.verify_hello_world());
}

#[test]
fn test_memory_embedding() {
    let addr: u64 = 123456789;
    let fell = canonical_addr_to_field:<Fr>(addr);
    let recovered = field_to_canonical_addr:<Fr>(fell);
    assert_eq!(Some(addr), recovered);
}

#[test]
fn test_field_capacity() {
    assert!(field_supports_64_bit_addresses:<Fr>());
}
