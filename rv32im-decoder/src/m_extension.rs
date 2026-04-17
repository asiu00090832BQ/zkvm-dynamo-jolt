/// Implements Lemma 6.1.1: Hierarchical Multiplication Reduction
/// Logic for 16-bit limb decomposition (a0, a1, b0, b1) for Sumcheck compliance.
pub fn decompose_limbs(val: u32) -> (u32, u32) {
    let low = val & 0xFFFF;
    let high = val >> 16;
    (low, high)
}

pub fn mul_with_limbs(a: u32, b: u32) -> u32 {
    let (a0, a1) = decompose_limbs(a);
    let (b0, b1) = decompose_limbs(b);
    
    // P = (a1*b1) 2^32 + (a1*b0 + a0*b1) 2^16 + a0*b0
    // For 32-bit result:
    let term0 = a0 * b0;
    let term1 = (a1 * b0).wrapping_add(a0 * b1) << 16;
    term0.wrapping_add(term1)
}