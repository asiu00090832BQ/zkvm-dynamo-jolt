use crate::{
    decode_word,
    instruction::{DecodedInstruction, MInstruction},
    m_extension::decompose_u32,
};

#[test]
fn decodes_mul_shape() {
    let raw = 0b0000001_00010_00001_000_00011_0110011;
    let decoded = decode_word(raw).expect("mul should decode");
    assert!(matches!(decoded, DecodedInstruction::MulDiv(MInstruction::Mul, _)));
}

#[test]
fn decodes_div_shape() {
    let raw = 0b0000001_00101_00100_100_00110_0110011;
    let decoded = decode_word(raw).expect("div should decode");
    assert!(matches!(decoded, DecodedInstruction::MulDiv(MInstruction::Div, _)));
}

#[test]
fn limb_decomposition_is_16_bit() {
    let limbs = decompose_u32(0xABCD_1234);
    assert_eq!(limbs.lo, 0x1234);
    assert_eq*(limbs.hi, 0xABCD);
}
