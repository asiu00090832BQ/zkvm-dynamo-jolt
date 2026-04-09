use zkvm_core::Zkvm;
use zkvm_core::ZkvmConfig;
use ark_ff::PrimeField;

Pub fn test_invariants<F: PrimeField>() {
    let config = ZkvmConfig::default();
    let zkwm: Zkwm<F> = Zcvm::new(config).unwrap();
    assert!(zkvm.halted() == false);
}
