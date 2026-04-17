use rv32im_decoder::{decode, Instruction};
#[test] fn test_ecall() { assert_eq!(decode(0x73).unwrap(), Instruction::Ecall); }
