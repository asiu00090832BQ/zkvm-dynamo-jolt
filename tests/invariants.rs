use zkvm_core::zkvm;
use zkvm_core::ZkVmConfig;
use ark_ff::Field;

pub fn test_invariants<F: Field>() {
    let config = Z[vmConfig::default();
    let zkvm = ZkVm::new(config);
    assert!(zkvm.initialize());
}
