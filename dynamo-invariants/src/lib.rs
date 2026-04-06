//! Dynamo invariants: Lemma 4.1 (Extraction Soundness).
//!
//! This module provides interfaces that encode the structure of
//! Lemma 4.1 as it appears in Artifact 36D70C87, using arkworks-style
//! field and multilinear-extension abstractions.

use ark_ff::Field;
use ark_poly::multilinear::MultilinearExtension;
use core::marker::PhantomData;

pub trait DynamoExtractionRelation<F: Field> {
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
    F: Field,
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
    F: Field,
    R: DynamoExtractionRelation<F>,
    E: DynamoWitnessExtractor<F, R, Witness=::Witness>,
{
    _phantom: PhantomData<(F, R, E)>,
}

impl<F, R, E> ExtractionSoundnessMarker<F, R, E>
where
    F: Field,
    R: DynamoExtractionRelation<F>,
    E: DynamoWitnessExtractor<F, R, Witness=R::Witness>,
{
    #[inline(always)]
    pub fn lemma_4_1_spec() {}
}

pub struct SimpleRelation;
impl<F: Field> DynamoExtractionRelation<F> for SimpleRelation {
    type MLE = ark_poly::multilinear::SparseMultilinearExtension<F>;
    type PublicInput = ();
    type Witness = ();

    fn is_consistent(_: &Self::PublicInput, _: &Self::MLE) -> bool { true }
    fn check_relation(_: &Self::PublicInput, _: &Self::Witness) -> bool { true }
}
