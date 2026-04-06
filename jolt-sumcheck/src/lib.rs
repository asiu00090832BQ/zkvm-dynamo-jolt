#![forbid(unsafe_code)]

//! Minimal sumcheck-facing interfaces for the workspace.
//!
//! This crate is the thin algebraic layer that consumes the witness
//! discipline established by `dynamo-invariants`.

use dynamo_invariants::{ExtractionWitness, MemoryClaim};

/// A placeholder claim that a sumcheck protocol might verify.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumcheckClaim {
    pub expected_claim_count: usize,
}

/// Verifies a minimal interface contract between the algebraic layer
/// and the extracted witness.
pub fn verify_sumcheck_interface(claim: &SumcheckClaim, witness: &ExtractionWitness) -> bool {
    witness.is_sound() && witness.claims().len() == claim.expected_claim_count
}

/// Folds memory claim values into a single accumulator.
/// This is a placeholder for a future algebraic reduction.
pub fn fold_claim_values(claims: &[MemoryClaim]) -> u64 {
    claims
        .iter()
        .fold(0_u64, |acc, claim| acc.wrapping_add(claim.value))
}
