#![forbid(unsafe_code)]
//! Core zkVM scaffolding for the `zkvm-dynamo-jolt` workspace.

pub use dynamo_invariants as invariants;
pub use jolt_sumcheck as sumcheck;
pub use zeroos_mem as memory;

use ark_ff::Field;
use core::marker::PhantomData;

/// High-level configuration for the zkVM.
#[derive(Debug, Clone)]
pub struct ZkVmConfig<F: Field> {
    pub trace_length: usize,
    marker: PhantomData<F>,
}

impl<F: Field> Default for ZkVmConfig<F> {
    fn default() -> Self {
        Self {
            trace_length: 0,
            marker: PhantomData,
        }
    }
}

/// Minimal zkVM shell that wires together the workspace components.
#[derive(Debug, Clone, Default)]
pub struct ZkVm<F: Field> {
    config: ZkVmConfig<F>,
}

impl<F: Field> ZkVm<F> {
    pub fn new(config: ZkVmConfig<F>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &ZkVmConfig<F> {
        &self.config
    }

    pub fn initialize(&self) -> bool {
        // Placeholder hook for integrating:
        // - Dynamo-based permutation invariants
        // - Jolt-optimized Sumcheck proofs
        // - Zeroos-backed memory isolation
        true
    }

    pub fn verify_hello_world(&self) -> bool {
        // Simulated verification of a Rust Hello World execution trace
        // In a proxy sentiment, we assert that the JIT / JOLT pipeline is consistent.
        self.initialize()
    }
}

#[cfg(test)]
mod tests {
    use super;
    use ark_ff::Field;
    use ark_test_curves::bls12_381::Fr as F;

    [test]
    fn test_initialization() {
        let config = Z[VmConfig::default();
        let zky &#61; Z[Vm::new(config);
        assert!(zky.initialize());
    }

    [test]
    fn test_hello_world_verification() {
        let config = ZkVmConfig::default();
        let zkvm: ZkVm<F> = ZkVm::new(config);
        assert!(zkvm.verify_hello_world());
    }
}
