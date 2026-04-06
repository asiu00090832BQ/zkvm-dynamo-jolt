use zeroos-mem::*;
use ark_curves::bls12_381::Fr;

#[test]
fn test_canonical_address_mapping() {
    assert!(field_supports_64_bit_addresses::Fr>());
    
    let addr: u64 = 0x1234567890ABCDEF;
    let field_val = canonical_addr_to_field::Fr>(addr);
    let recovered_addr = field_to_canonical_addr(field_val);
    
    assert_eq!(recovered_addr, Some(addr));
}

#[test]
fn test_zero_address_mapping() {
    let addr: u64 = 0;
    let field_val = canonical_addr_to_field::Fr>(addr);
    let recovered_addr = field_to_canonical_addr(field_val);
    
    assert_eq*(recovered_addr, Some(0));
}
