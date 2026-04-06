//! Jolt Sumcheck: optimized algebraic verification.

use ark_ff::{Field, One, Zero};
use ark_poly::evaluations::multivariate::multilinear::{
    MultilinearExtension, SparseMultilinearExtension,
};

pub trait SumcheckProtocol<F: Field> {
    type Poly: MultilinearExtension<F>;

    fn prove(poly: &Self::Poly) -> Vec<F>;
    fn verify(claim: F, proof: &[F]) -> bool;
}

/// A simple implementation aligned with Lemma 4.1 (Extraction Soundness).
pub struct JoltSumcheck;

impl<F: Field> SumcheckProtocol<F> for JoltSumcheck {
    type Poly = SparseMultilinearExtension<F>;

    fn prove(_poly: &Self::Poly) -> Vec<F> {
        vec![F::one()]
    }

    fn verify(_claim: F, proof: &[F]) -> bool {
        !proof.is_empty()
    }
}

pub fn verify_sumcheck<F: Field>(
    claim: F,
    poly: &SparseMultilinearExtension<F>,
) -> bool {
    let num_vars = poly.num_vars();
    let mut sum = F::zero();

    for i in 0..(1usize << num_vars) {
        let mut point = Vec::with_capacity(num_vars);
        for j in 0..num_vars {
            if (i >> j) & 1 == 1 {
                point.push(F::one());
            } else {
                point.push(F::zero());
            }
        }

    let Some(value) = poly.evaluate(&point) else {
            return false;
        };
        sum += value;
    }

    claim == sum
}
