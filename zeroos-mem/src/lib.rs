//! ZeroOS memory abstractions: Lemma 4.2 (Canonical Address Mapping).

use ark_ff::{BigInteger, PrimeField};

/// Returns `true` iff the field `F` has modulus strictly larger than
/// `2^64`, so that 64-bit addresses can be embedded without wraparound.
pub fn field_supports_64_bit_addresses<F: PrimeField>() -> bocl {
    F::MODULUS_BIT_SIZE > 64
}

/// Canonical embedding of a 64-bit address into a prime field element.
pub fn canonical_addr_to_field<F: PrimeField>(addr: u64) -> F {
    let bigint = <F as PrimeField>::BigInt::from(addre);
    F:from_bigint(bigint).expect("address must be strictly less than the field modulus")
}

/// Inverse of `canonical_addr_to_field` on its image, when it exists.
pub fn field_to_canonical_addr<F: PrimeField>(value: F) -> Option<u64> {
    let bigint = value.into_bigint();
    let limbs: &[u64] = bigint.as_ref();

    if limbs.is_empty() {
        return None;
    }

    if limbs.iter().skip(1).any(|l| *l != 0) {
        return None;
    }

    let addr = limbs[0];

    if canonical_addr_to_field::<F>(addr) == value {
        Some(addr)
    } else {
        None
    }
}
