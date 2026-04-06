use dynamo-invariants::{ExtractionWitness, MemoryClaim};
use jolt_sumcheck::{fold_claim_values, verify_sumcheck_interface, SumcheckClaim};

#[test]
fn interface_accepts_sound_witness_with_matching_length() {
    let witness = ExtractionWitness::new(vec![
        MemoryClaim {
            segment: 0,
            offset: 0,
            value: 1,
        },
        MemoryClaim {
            segment: 0,
            offset: 1,
            value: 2,
        },
    ]);

    let claim = SumcheckClaim {
        expected_claim_count: 2,
    };

    assert!(verify_sumcheck_interface(&claim, &witness));
    assert_eq!(fold_claim_values(witness.claims()), 3);
}

#[test]
fn interface_rejects_length_mismatch() {
    let witness = ExtractionWitness::new(vec![MemoryClaim {
        segment: 0,
        offset: 0,
        value: 9,
    }]);

    let claim = SumcheckClaim {
        expected_claim_count: 2,
    };

    assert!(!verify_sumcheck_interface(&claim, &witness));
}
