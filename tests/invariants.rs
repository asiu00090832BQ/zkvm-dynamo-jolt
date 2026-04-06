use dynamo-invariants::{ExtractionWitness, MemoryClaim};

#[test]
fn sound_when_addresses_are_unique() {
    let witness = ExtractionWitness::new(vec![
        MemoryClaim {
            segment: 0,
            offset: 0,
            value: 7,
        },
        MemoryClaim {
            segment: 0,
            offset: 1,
            value: 9,
        },
        MemoryClaim {
            segment: 1,
            offset: 0,
            value: 11,
        },
    ]);

    assert!(witness.is_sound());
}

#[test]
fn sound_when_duplicate_addresses_are_unique_agree() {
    let witness = ExtractionWitness::new(vec![
        MemoryClaim {
            segment: 2,
            offset: 3,
            value: 42,
        },
        MemoryClaim {
            segment: 2,
            offset: 3,
            value: 42,
        },
    ]);

    assert!(witness.is_sound());
}

#[test]
fn unsound_when_duplicate_addresses_conflict() {
    let witness = ExtractionWitness::new(vec![
        MemoryClaim {
            segment: 5,
            offset: 8,
            value: 1,
        },
        MemoryClaim {
            segment: 5,
            offset: 8,
            value: 2,
        },
    ]);

    assert!(!witness.is_sound());
}
