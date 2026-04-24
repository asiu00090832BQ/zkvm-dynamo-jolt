// Invariant Verification: Lemma 6.1.1
// P(x, y) = a_hi*b_hi*B2 + (a_hi*b_lo + a_lo*b_hi)*B + a_lo*b_lo where B = 2^16.

pub struct ProofPipeline;

impl ProofPipeline {
    pub fn new() -> Self {
        Self
    }

    /// Derives the multilinear reduction coefficients for u32 operands.
    pub fn decompose_limbs(&self, a: u32, b: u32) -> [u64; 3] {
        let a_0 = (a & 0xFFFF) as u64;
        let a_1 = (a >> 16) as u64;
        let b_0 = (b & 0xFFFF) as u64;
        let b_1 = (b >> 16) as u64;

        [
            a_0 * b_0,               // c_0
            a_1 * b_0 + a_0 * b_1,   // c_1
            a_1 * b_1,               // c_2
        ]
    }

    /// Invariant verified: actual == (c_2 << 32) + (c_1 << 16) + c_0
    pub fn check_parity(&self, a: u32, b: u32, c: [u64; 3]) -> bool {
        let expected = (a as u64) * (b as u64);
        let assembled = (c[2] << 32) + (c[1] << 16) + c[0];
        assembled == expected
    }
}
