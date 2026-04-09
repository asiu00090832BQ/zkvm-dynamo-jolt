#![forbid(unsafe_code)]
//! Invariant specifications for the zkvm-dynamo-jolt project.
//! This crate defines the core traits and markers for formal verification.

use ark_ff::PrimeField;

/// Marker for extraction-soundness proofs associated with Lemma 4.1.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct ExtractionSoundnessMarker;

/// Core trait for Lemma 4.1 invariants.
pub trait Lemma41<F: PrimeField> {
    /// Returns true if the field element satisfies the base consistency property.
    fn is_consistent(&self, field_element: &F) -> bool;
    /// Returns true if the transition from current to next preserves the invariant.
    fn step(&self, current: &F, next: &F) -> bool;
    /// Verifies the invariant across an execution trace.
    fn check_trace(&self, trace: &[F]) -> bool {
        if trace.is_empty() {
            return true;
        }
        if !self.is_consistent(&trace[0]) {
            return false;
        }
        for pair in trace.windows(2) {
            if !self.step(&pair[0], &pair[1]) {
                return false;
            }
        }
        true
    }
    /// Associated marker for Lemma 4.1 proofs.
    #[inline(always)]
    fn extraction_soundness_marker(&self) -> ExtractionSoundnessMarker {
        ExtractionSoundnessMarker
    }
}

/// Verification helper for Lemma 4.1 invariants.
#[inline(always)]
pub fn lemma_4_1_holds<F, L>(invariant: &L, trace: &[F]) -> bool
where
    F: PrimeField,
    L: Lemma41<F>,
{
    invariant.check_trace(trace)
}
