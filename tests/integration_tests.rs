use zkvm_core::{VmConfig, Zkvm};

type Fr = u64;

#[test]
fn zkvm_initialization_and_verification() {
    let config = VmConfig::default();
    let mut vm: Zkvm<Fr> = Zkvm::new(config);

    // Ensure the VM can be initialized successfully.
    assert!(vm.initialize());

    // Ensure that execution verification stub behaves as expected.
    assert!(vm.verify_execution("test-program"));
}