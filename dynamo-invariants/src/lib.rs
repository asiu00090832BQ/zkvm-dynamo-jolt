use std::collections::BTreeMap;

// Lemma 4.1: an extraction witness is sound exactly when no two claims
// assign different values to the same canonical memory address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryClaim {
    pub segment: usize,
    pub offset: usize,
    pub value: u64,
}

impl MemoryClaim {
    pub fn canonical_address(&self) -> (usize, usize) {
        (self.segment, self.offset)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExtractionWitness {
    pub claims: Vec<MemoryClaim>,
}

impl ExtractionWitness {
    pub fn new(claims: Vec<MemoryClaim>) -> Self {
        Self { claims }
    }

    pub fn is_sound(&self) -> bool {
        let mut seen: BTreeMap<(usize, usize), u64> = BTreeMap::new();

        for claim in &self.claims {
            let addr = claim.canonical_address();
            match seen.get(&addr) {
                Some(existing) if *existing != claim.value => return false,
                Some(_) => {}
                None => {
                    seen.insert(addr, claim.value);
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::{ExtractionWitness, MemoryClaim};

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
    fn sound_when_duplicate_addresses_agree() {
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
}
