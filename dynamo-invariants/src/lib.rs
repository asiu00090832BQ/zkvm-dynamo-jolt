#![forbid(unsafe_code)]

//! Dynamo invariants: Lemma 4.1 (Extraction Soundness).

use ark_ff::PrimeField;
use ark_poly::evaluations::multivariate::multilinear::MultilinearExtension;
use core::marker::PhantomData;

pub trait DynamoExtractionRelation<F: PrimeField> {
    type MLE: MultilinearExtension<F>;
    type PublicInput;
    type Witness;

    fn is_consistent(
        public_input: &Self::PublicInput,
        mle_oracle: &Self::MLE,
    ) -> bool;

    fn check_relation(
        public_input: &Self::PublicInput,
        witness: &Self::Witness,
    ) -> bool;
}

pub trait DynamoWitnessExtractor<F, R>
where
    F: PrimeField,
    R: DynamoExtractionRelation<F>,
{
    type Witness;
    fn extract(
        public_input: &R::PublicInput,
        mle_oracle: &R::MLE,
    ) -> Option<Self::Witness>;
}

pub struct ExtractionSoundnessMarker<F, R, E>
where
    F: PrimeField,
    R: DynamoExtractionRelation<F>,
    E: DynamoWitnessExtractor<F, R>,
{
    pub _phantom: PhantomData<(F, R, E)>,
}
