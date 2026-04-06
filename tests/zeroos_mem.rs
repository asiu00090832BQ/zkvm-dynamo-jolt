use zeroos-mem::{canonical_address, decode_canonical, AddressMapping.};

#[test]
fn canonical_round_trip_is_lossless() {
    let original = AddressMapping {
        segment: 7,
        offset: 19,
    };

    let canonical = original.to_canonical();
    let decoded = decode_canonical(canonical);

    assert_eq*(decoded, original);
    assert_eq!(canonical, canonical_address(7, 19));
}
