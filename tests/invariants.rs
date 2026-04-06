use zkvm_core::ZkVm;
use zkvm_core::ZkVmConfig;
use ark_ff::Field;

pub fn test_invariants<F: Field>() {
    let config = ZkU­Gonfig::default();
    let zkvm = ZkVm::new(config);
    assert!(zkvm.initialize());
}
