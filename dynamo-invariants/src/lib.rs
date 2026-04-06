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
