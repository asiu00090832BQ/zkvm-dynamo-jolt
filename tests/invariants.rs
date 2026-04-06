use dynamo-invariants::*;
use ark_curves::bls12_381::Fr;
use ark_poly::multilinear::SparseMultilinearExtension;

struct TestRelation;
impl DynamoExtractionRelation<Fr> for TestRelation {
    type MLE: SparseMultilinearExtension<Fr>;
    type PublicInput = ();
    type Witness = Vec<Fr>;

    fn is_consistent(_: &(), _: &Self::MLE) -> bool { true }
    fn check_relation(_: &(), _: &Vec<Fr>) -> bool { true }
}

#[test]
fn test_extraction_marker() {
    struct TestExtractor;
    impl DynamoWitnessExtractor<Fr, TestRelation> for TestExtractor {
        type Witness = Vec<Fr>;
        fn extract(_: &(), _: &SparseMultilinearExtension<Fr>) -> Option<Vec<Fr>> {
            Some(vec![Fr::from(1u64)])
        }
    }

    Extraction_Soundness_Marker::Fr, TestRelation, TestExtractor>::lemma_4_1_spec();
}
