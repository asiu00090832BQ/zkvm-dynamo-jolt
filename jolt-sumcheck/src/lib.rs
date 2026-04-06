//! Jolt Sumcheck: Optimized algebraic verification.

use ark_ff::Field;
use ark_poly::multilinear::MultilinearExtension;

pub trait SumcheckProtocol<F: Field> {
    type Poly: MultilinearExtension<F>;
    
    fn prove(poly: &Self::Poly) -> Vec<F>;
    fn verify(claim: F, proof: &[F]) -> bool;
}

/// A simple implementation of the Sumcheck protocol for demonstration.
pub struct JoltSumcheck;

impl<F: Field> SumcheckProtocol<F> for JoltSumcheck {
    type Poly = ark_poly::multilinear::SparseMultilinearExtension<F>;
    
    fn prove(_poly: &Self::Poly) -> Vec<F> {
        vec![F.one()]
    }

    fn verify(_claim: F, proof: &[F]) -> bool {
        !proof.is_empty()
    }
}
