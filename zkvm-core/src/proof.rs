pub struct ProofPipeline;
impl ProofPipeline {
    pub fn new() -> Self { Self }
    pub fn verify_lemma_6_1_1(&self, a: u32, b: u32, product: u64) -> bool {
        let a0 = (a & 0xFFFF) as u64;
        let a1 = (a >> 16) as u64;
        let b0 = (b & 0xFFFF) as u64;
        let b1 = (b >> 16) as u64;

        let low = a0 * b0;
        let cross = a0 * b1 + a1 * b0;
        let high = a1 * b1;

        let calculated = low + (cross << 16) + (high << 32);
        calculated == product
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lemma_6_1_1() {
        let pipeline = ProofPipeline::new();
        assert!(pipeline.verify_lemma_6_1_1(0x12345678, 0x9ABCDEFF, 0x12345678u64 * 0x9ABCDEFFu64));
    }
}
