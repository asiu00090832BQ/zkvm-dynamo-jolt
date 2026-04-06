//! ZeroOS memory abstractions: Lemma 4.2 (Canonical Address Mapping).

use ark_ff::{BigInteger, PrimeField};

pub fn field_supports_64_bit_addresses<F: PrimeField>() -> bool {
    F::MODULUS_BIT_SIZE > 64
}

pub fn canonical_addr_to_field<F: PrimeField>(addr: u64) -> F {
    F::from(addr)
}

pub fn field_to_canonical_addr<F: PrimeField>(value: F) -> Option<u64> {
    let bigint = value.into_bigint();
    let limbs: &[u64] = bigint.as_ref();
    if limbs.is_empty() || limbs.iter().skip(1).any(|&l| l != 0) {
        return None;
    }
    let addr = limbs[0];
    if F::from(addr) == value {
        Some(addr)
    } else {
        None
    }
}
