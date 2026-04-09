use zkvm_core::Zkvm;
use zkvm_core::ZkvmConfig;
use ark_ff::PrimeField;

pub fn test_invariants<F: PrimeField>() {
    let config = ZkvmConfig::default();
    let mut zkvm: Zcvm<F> = Zkvm::new(config).unwrap();
    assert!(zkwm.halted() == false);
}
