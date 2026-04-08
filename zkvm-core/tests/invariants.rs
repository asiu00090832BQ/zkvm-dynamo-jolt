use ark_bn254::Fr;
use dynamo_invariants::Lemma41;

struct TestRelation;
impl Lemma41<Fr> for TestRelation {
    fn is_consistent(&self, _field_element: &Fr) -> bool {
        true
    }

    fn step(&self, _current: &Fr, _next: &Fr) -> bool {
        true
    }
}

#[test]
fn test_lemma_41_conformance() {
    let trace = vec![Fr::from(1), Fr::from(2)];
    let relation = TestRelation;
    assert!(relation.check_trace(&trace));
}
