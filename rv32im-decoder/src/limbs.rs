/// Lemma 6.1.1: Hierarchical Multiplication Reduction Invariants
pub struct Limb16 {
    pub low: u32,
    pub high: u32,
}

impl Limb16 {
    pub fn decompose(val: u32) -> Self {
        Self {
            low: val & 0xFFFF,
            high: val >> 16,
        }
    }
}
