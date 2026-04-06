use dynamo_invariants::*;
use ark_bn254::Fr;

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
fn test_extraction_marker() {
    let relation = TestRelation;
    let trace = vec![Fr::from(1u64), Fr::from(1u64)];
    assert!(lemma_4_1_holds(&relation, &trace));

    let _marker = relation.extraction_soundness_marker();
}
