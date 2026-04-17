use rv32im_decoder::m_extension::*;

#[test]
fn test_decompose_and_reconstruct_roundtrip() {
    let samples: [u32; 6] = [0, 1, 0xffff, 0x0001_0000, 0x1234_5678, 0xffff_ffff];

    for &x in &samples {
        let (lo, hi) = decompose_u32(x);
        let y = reconstruct_from_limbs(lo, hi);
        assert_eq!(x, y, "roundtrip failed for {:#010x}", x);
    }
}

#[test]
fn test_pair_decomposition_consistency() {
    let pairs: &[(u32, u32)] = &[
        (0, 0),
        (1, 1),
        (0xffff, 0xffff),
        (0x0001_0000, 0x0001_0000),
        (0x1234_5678, 0x9abc_def0),
        (0xffff_ffff, 0xffff_ffff),
    ];

    for &(a, b) in pairs {
        let (a0, a1, b0, b1) = decompose_32bit_limbs(a, b);
        let ra = reconstruct_from_limbs(a0, a1);
        let rb = reconstruct_from_limbs(b0, b1);
        assert_eq!(a, ra, "a reconstruction failed for {:#010x}", a);
        assert_eq!(b, rb, "b reconstruction failed for {:#010x}", b);
    }
}

#[test]
fn test_limb_product_matches_native() {
    let pairs: &[(u32, u32)] = &[
        (0, 0),
        (0, 123456),
        (1, 0xffff_ffff),
        (0xffff, 0xffff),
        (0x1234_5678, 0x9abc_def0),
        (0xffff_ffff, 0xffff_ffff),
    ];

    for &(a, b) in pairs {
        let native = (a as u64) * (b as u64);
        let via_limbs = mul_via_limbs(a, b);
        assert_eq!(native, via_limbs, "mul_via_limbs mismatch for a={:#010x}, b={:#010x}", a, b);
    }
}

#[test]
fn test_verify_limb_decomposition_ok() {
    let pairs: &[(u32, u32)] = &[
        (0, 0),
        (1, 1),
        (123, 456),
        (0x0001_0000, 0x0002_0003),
        (0x1234_5678, 0x9abc_def0),
        (0xffff_ffff, 0xffff_ffff),
    ];

    for &(a, b) in pairs {
        verify_limb_decomposition(a, b).unwrap();
    }
}
