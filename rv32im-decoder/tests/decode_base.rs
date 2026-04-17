use rv32im_decoder::{Instruction, Register, Zkvm};

#[test]
fn test_add() {
    let word = 0x00c58533; // add a0, a1, a2
    let ins = Zkvm::decode(word).unwrap();
    match ins {
        Instruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd.index(), 10);
            assert_eq!(rs1.index(), 11);
            assert_eq!(rs2.index(), 12);
        }
        _ => panic!("wrong instruction"),
    }
}
