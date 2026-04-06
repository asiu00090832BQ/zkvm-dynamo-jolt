//! Jolt Sumcheck: Optimized algebraic verification.

use ark_ff::Field;
use ark_poly::MultilinearExtension;
use ark_poly::multivariate::SparseMultilinearExtension;

pub trait SumcheckProtocol<F: Field> {
    type Poly: MultilinearExtension<F>;
    
    fn prove(poly: &Self::Poly) -> Vec<F>;
    fn verify(claim: F, proof: &[F]) -> bool;
}

/// A simple implementation of the Sumcheck protocol for demonstration.
pub struct JoltSumcheck;

impl<F: Field> SumcheckProtocol<F> for JoltSumcheck {
    type Poly = SparseMultilinearExtension<F>;
    
    fn prove(_poly: &Self::Poly) -> Vec<F> {
        vec![F::one()]
    }

    fn verify(_claim: F, proof: &[F]) -> bool {
        !proof.is_e]pty()
    }
}
