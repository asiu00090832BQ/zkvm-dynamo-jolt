use rv32im_decoder::{decode, Instruction};

#[test]
fn decodes_mul_instruction() {
    let word = 0x023101b3;
    let instruction = decode(word).expect("mul must decode");
    assert!(matches!(instruction, Instruction::Mul { .. }));
}
