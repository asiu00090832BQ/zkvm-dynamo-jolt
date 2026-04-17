use crate::error::ZkvmError;

const LIMB_BITS: u32 = 16;
const LIMB_MASK: u64 = (1u64 << LIMB_BITS) - 1;

pub const fn decompose_u32_to_limbs_16(value: u32) -> [u16; 2] {
    [
        (value & LIMB_MASK as u32) as u16,
        ((value >> LIMB_BITS) & LIMB_MASK as u32) as u16,
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limb16Product {
    pub lhs_limbs: [u16; 2],
    pub rhs: [u16; 2],
    pub partial_products: [u64; 4],
    pub carries: [u64; 2],
    pub recomposed: u64,
    pub product: u64,
}

impl Limb16Product {
    pub fn from_operands(lhs: u32, rhs: u32) -> Self {
        let lhs_limbs = decompose_u32_to_limbs_16(lhs);
        let rhs_limbs = decompose_u32_to_limbs_16(rhs);

        let p00 = u64::from(lhs_limbs[0]) * u64::from(rhs_limbs[0]);
        let p01 = u64::from(lhs_limbs[0]) * u64::from(rhs_limbs[1]);
        let p10 = u64::from(lhs_limbs[1]) * u64::from(rhs_limbs[0]);
        let p11 = u64::from(lhs_limbs[1]) * u64::from(rhs_limbs[1]);

        let carry0 = p00 >> LIMB_BITS;
        let middle = p01 + p10 + carry0;
        let carry1 = middle >> LIMB_BITS;

        let low16 = p00 & LIMB_MASK;
        let mid16 = middle & LIMB_MASK;
        let high32 = p11 + carry1;

        let recomposed = low16 | (mid16 << LIMB_BITS) | (high32 << (LIMB_BITS * 2));
        let product = u64::from(lhs) * u64::from(rhs);

        Self {
            lhs_limbs,
            rhs_limbs,
            partial_products: [p00, p01, p10, p11],
            carries: [carry0, carry1],
            recomposed,
            product,
        }
    }

    pub const fn verify(&self) -> bool {
        self.recomposed == self.product
    }
}

pub fn verify_lemma_6_1_1(lhs: u32, rhs: u32) -> Result<(), ZkvmError> {
    let witness = Limb16Product::from_operands(lhs, rhs);

    if witness.verify() {
        Ok(())
    } else {
        Err(ZkvmError::VerificationFailed(
            "Lemma 6.1.1 failed for 16-bit limb recomposition",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{decompose_u32_to_limbs_16, verify_lemma_6_1_1, Limb16Product};

    #[test]
    fn decomposition_is_little_endian_in_16_bit_limbs() {
        assert_eq!(decompose_u32_to_limbs_16(0x1234_abcd), [0xabcd, 0x1234]);
    }

    #[test]
    fn lemma_holds_for_edge_cases() {
        for (lhs, rhs) in [
            (0, 0),
            (1, 1),
            (u32::MAX, 1),
            (u32::MAX, u32::MAX),
            (0x1234_5678, 0x9abc_def0),
        ] {
            let witness = Limb16Product::from_operands(lhs, rhs);
            assert!(witness.verify(), "{lhs:#x} * {rhs:#x}");
            assert!(verify_lemma_6_1_1(lhs, rhs).is_ok());
        }
    }
}
