use zkvm_core::{VmConfig, Zkvm};

/// For testing purposes we use a simple concrete field type.
/// In a real project this would likely be a prime field from a cryptography
/// library (e.g. `ark_bn254::Fr`).
type Fr = u64;

#[test]
fn zkvm_initialization_and_verification() {
    let config = VmConfig::default();
    let vm: Zkvm<Fr> = Zkvm::new(config);

    // Ensure the VM can be initialized successfully.
    assert!(vm.initialize());

    // Ensure that execution verification stub behaves as expected.
    assert!(vm.verify_execution("test-program"));
}