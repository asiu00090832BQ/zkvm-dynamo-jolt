use ark_ff::PrimeField;
use ark_poly::evaluations::multivariate::multilinear::MultilinearExtension;

pub trait SumcheckProtocol<F: PrimeField> {
    type Poly: MultilinearExtension<F>;

    fn prove(poly: &Self::Poly) -> Vec<F>;
    fn verify(claim: F, proof: &[F]) -> bool;
}
