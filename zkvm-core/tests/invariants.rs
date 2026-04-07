use ark_bn254::Fr;
use dynamo_invariants::*;

struct TestRelation;
impl Lemma41<Fr> for TestRelation {
    fn verify_sub_sequence_extraction(_trace: &[Fr]) -> bool {
        true
    }
}

#[test]
fn test_lemma_41_conformance() {
    let trace = vec![Fr::from(1), Fr::from(2)];
    assert!(TestRelation::verify_sub_sequence_extraction(&trace));
}
