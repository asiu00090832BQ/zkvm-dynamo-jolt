use rv32im_decoder::{Instruction, Zkvm, MulDecomposition};

#[test]
fn test_mul() {
    let word = 0x02c58533; // mul a0, a1, a2
    let ins = Zkvm::decode(word).unwrap();
    assert!(matches!(ins, Instruction::Mul { .. }));
}

#[test]
fn test_lemma_6_1_1_parity() {
    let a = 0x12345678;
    let b = 0x87654321;
    let d = MulDecomposition::from_operands(a, b);
    let expected = (a as u64) * (b as u64);
    assert_eq!(d.product_u64(), expected);
}
