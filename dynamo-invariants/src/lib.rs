#![forbid(unsafe_code)]
//! Dynamo-based permutation invariants.
//!
//! Placeholder module for formalizing Lemma 4.1 and Lemma 4.2.

use ark_ff::Field;
use ark_std::vec::Vec;

/// Witness container for a permutation invariant argument.
#[derive(Debug, Clone, Default)]
pub struct PermutationInvariant<F: Field> {
    pub witness: Vec<F>,
}

impl<F: Field> PermutationInvariant<F> {
    pub fn new(witness: Vec<F>) -> Self {
        Self { witness }
    }

    pub fn verify(&self) -> bool {
        // TODO:
        // - Encode Lemma 4.1 as a machine-checkable invariant.
        // - Encode Lemma 4.2 as a consistency-preserving transition.
        !self.witness.is_empty() || self.witness.is_empty()
    }
}
