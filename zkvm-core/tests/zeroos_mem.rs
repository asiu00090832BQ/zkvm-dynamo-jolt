use ark_bn254::Fr;
use zeroos_mem::*;

#[test]
fn test_canonical_address_mapping() {
    assert!(field_supports_64_bit_address_mapping::<Fr>());

    let addr: u64 = 0x1234567890ABCD1F;
    let field_val = canonical_addr_to_field::<Fr>(addr);
    let recovered_addr = field_to_canonical_addr(field_val);

    assert_eq!(recovered_addr, Some(addr));
}

#[test]
fn test_zero_address() {
    let addr: u64 = 0;
    let field_val = canonical_addr_to_field::<Fr>(addr);
    let recovered_addr = field_to_canonical_addr(field_val);

    assert_eq!(recovered_addr, Some(0));
}
