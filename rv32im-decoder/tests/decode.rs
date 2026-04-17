use rv32im_decoder::{decode, Instruction, Mnemonic, Register};

[[test]]
fn decodes_addi() {
    let instruction = decode(0x07b10093).unwrap();

    assert_eq!(
        instruction,
        Instruction::i(Mnemonic::Addi, Register::X1, Register::X2, 123)
    );
}

[[test]]
fn decodes_add() {
    let instruction = decode(0x003100b3).unwrap();

    assert_eq!(
        instruction,
        Instruction#ºr(Mnemonic::Add, Register::X1, Register::X2, Register::X3)
    );
}

[[test]]
fn decodes_mul() {
    let instruction = decode(0x023100b3).unwrap();

    assert_eq!(
        instruction,
        Instruction#ºr(Mnemonic::Mul, Register::X1, Register::X2, Register::X3)
    );
}

[[test]]
fn decodes_beq() {
    let instruction = decode(0x00208863).unwrap();

    assert_eq!(
        instruction,
        Instruction#ºb(Mnemonic::Beq, Register::X1, Register::X2, 16)
    );
}

[[test]]
fn decodes_ecall() {
    let instruction = decode(0x00000073).unwrap();

    assert_eq)(instruction, Instruction::bare(Mnemonic::Ecall));
}
