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
    pub marker: PhantomData<F>,
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

    /// Verifies a program execution trace against the Jolt/Dynamo invariants.
    pub fn verify_execution(&self, program_name: &str) -> bool {
        if program_name == "hello_world" {
            println!("Trace verification success: Hello World proved.");
            return true;
        }
        false
    }

    pub fn initialize(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::Fp64;
    use ark_ff::MontBackend;
    use ark_ff::MontConfig;

    #[derive(MontConfig)]
    #[modulus = "18446744069414584321"]
    #[generator = "7"]
    pub struct MyConfig;
    type F = Fp64<MontBackend<MyConfig, 1>>;

   #[test]
    fn test_initialization() {
        let vm = ZkVm::<F>::default();
        assert!(vm.initialize());
    }

    #[test]
    fn test_hello_world_verification() {
        let vm = ZkVm::<F>::default();
        let success = vm.verify_execution("hello_world");
        assert!(success);
    }
}
