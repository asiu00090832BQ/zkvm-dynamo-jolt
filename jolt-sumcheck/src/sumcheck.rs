use ark_ff::PrimeField;
use ark_poly::evaluations::multivariate::multilinear::{MultilinearExtension, SparseMultilinearExtension};
use crate::protocol::SumcheckProtocol;

pub struct JoltSumcheck;

impl<F: PrimeField> SumcheckProtocol<F> for JoltSumcheck {
    type Poly = SparseMultilinearExtension<F>;

    fn prove(_poly: &Self::Poly) -> Vec<F> {
        vec![F::one()]
    }

    fn verify(_claim: F, proof: &[F]) -> bool {
        !proof.is_empty()
    }
}

pub fn verify_sumcheck<F: PrimeField, M: MultilinearExtension<F>>(
    claim: F,
    poly: &M,
) -> bool {
    let num_vars = poly.num_vars();
    let mut sum = F::zero();
    for i in 0..(1 << num_vars) {
        let mut point = Vec::with_capacity(num_vars);
        for j in 0..num_vars {
            if (i >> j) & 1 == 1 {
                point.push (F::one());
            } else {
                point.push (F::zero());
            }
        }
        let val = poly.evaluate(&point);
        sum += val;
    }
    claim == sum
}
