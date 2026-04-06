//! Jolt Sumcheck: Optimized algebraic verification.
//!
//! This module provides the core Sumcheck protocol logic for
//! the zkvm-dynamo-jolt workspace, aligned with Lemma 2.1.

use ark_ff::Field;
use ark_poly::multilinear::MultilinearExtension;

pub trait SumcheckProtocol<F: Field> {
    type Poly: MultilinearExtension<F>;
    
    fn prove(poly: &Self::Poly) -> Vec<F>;
    fn verify(claim: F, proof: &[F]) -> bool;
}
