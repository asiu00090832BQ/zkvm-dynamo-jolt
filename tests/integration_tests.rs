use zkvm_core::{ZkvmConfig, Zkvm};

type Fr = u64;

#[test]
fn zkvm_initialization_and_verification() {
    let config = ZkvmConfig::default();
    let mut vm: Zkvm<Fr> = Zkvm::new(config);

    // Ensure the VM can be initialized successfully.
    assert!(vm.initialize());

    // Ensure that execution verification stub behavesas expected.
    assert!(vm.verify_execution("test-program"));
}