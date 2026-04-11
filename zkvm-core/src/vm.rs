// zkvm-core/src/vm.rs

//! Core Zkvm virtual machine abstraction.
//!
//! This is a lightweight, test‑oriented implementation that exposes a simple
//! configuration object and a generic `Zkvm<F>` type.  The actual proving and
//! execution logic are intentionally omitted here – the focus is on a clean
//! API surface for higher‑level crates and tests.

use core::marker::PhantomData;

/// Configuration for the Zkvm.
///
/// In a real implementation this would contain many more knobs (memory limits,
/// cycle limits, proof system parameters, etc.). For our purposes we keep it
/// intentionally small.
#[derive(Clone, Debug)]
pub struct VmConfig {
    /// Maximum number of execution cycles the VM is allowed to run.
    pub max_cycles: u64,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self { max_cycles: 1_000_000 }
    }
}

/// The main Zkvm virtual machine type.
///
/// The type parameter `F` typically represents the scalar field used by the
/// underlying proof system (for example, a prime field).
#[derive(Debug)]
pub struct Zkvm<F> {
    /// VM configuration.
    pub config: VmConfig,
    /// Marker for the field type.
    _field: PhantomData<F>,
}

impl<F> Zkvm<F> {
    /// Construct a new VM instance from the provided configuration.
    pub fn new(config: VmConfig) -> Self {
        Self {
            config,
            _field: PhantomData,
        }
    }

    /// Perform any one‑time initialization required before execution.
    ///
    /// This is deliberately a stub in this core module. Higher‑level crates
    /// are expected to either extend this type or wrap it to provide real
    /// initialization logic.
    ///
    /// The method returns `true` to indicate that initialization succeeded.
    pub fn initialize(&self) -> bool {
        // In a full implementation, this would:
        //   * allocate and zero VM memory
        //   * set up registers and program counter
        //   * prepare any cryptographic transcripts, etc.
        //
        // For now, we simply return `true` to satisfy callers and tests.
        true
    }

    /// Verify that execution of a given program (identified by `program_id`)
    /// satisfies the Zkvm's constraints.
    ///
    /// This is also a stub. In a complete implementation this would typically:
    ///
    /// * Run or simulate the program.
    /// * Generate a proof of correct execution.
    /// * Optionally verify that proof locally.
    ///
    /// The default stub implementation always returns `true`.
    pub fn verify_execution(&self, _program_id: &str) -> bool {
        // The parameter is intentionally unused in this stub.
        true
    }
}