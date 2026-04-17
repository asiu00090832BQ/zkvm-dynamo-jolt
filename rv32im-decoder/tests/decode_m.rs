use rv32im_decoder::{decode, Instruction};

#[test]
fn decodes_mul() {
    let word = 0x023100b3; // mul x1, x2, x3
    assert!(decode(word).is_ok());
}
