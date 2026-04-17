use crate::{
    instruction::{IInstruction, InstructionKind, MInstruction, Rv32Extension, Rv32Opcode},
    verify_lemma_6_1_1,
    Zkvm,
    ZkvmError,
};

#[test]
fn lemma_6_1_1_decomposes_operands_into_16_bit_limbs() {
    let decomposition = verify_lemma_6_1_1(0xAABB_CCDD, 0x1122_3344).unwrap();

    assert_eq!(decomposition.a0(), 0xCCDD);
    assert_eq!(decomposition.a1(), 0xAABB);
    assert_eq!(decomposition.b0(), 0x3344);
    assert_eq!(decomposition.b1(), 0x1122);
    assert_eq!(decomposition.a.recompose(), 0xAABB_CCDD);
    assert_eq!(decomposition.b.recompose(), 0x1122_3344);
}

#[test]
fn zkvm_exposes_operand_decomposition() {
    let zkvm = Zkvm::new();
    let decomposition = zkvm.decompose_operands(0xDEAD_BEEF, 0x0123_4567).unwrap();

    assert_eq!(decomposition.a0(), 0xBEEF);
    assert_eq!(decomposition.a1(), 0xDEAD);
    assert_eq!(decomposition.b0(), 0x4567);
    assert_eq!(decomposition.b1(), 0x0123);
}

#[test]
fn decodes_rv32i_addi() {
    let zkvm = Zkvm::new();
    let word = 0x00A1_0093;
    let decoded = zkvm.decode_word(word).unwrap();

    assert_eq!(decoded.opcode, Rv32Opcode::OpImm);
    assert_eq!(decoded.extension, Rv32Extension::I);
    assert_eq!(decoded.kind, InstructionKind::I(IInstruction::Addi));
    assert_eq!(decoded.rd, 1);
    assert_eq!(decoded.rs1, 2);
    assert_eq!(decoded.rs2, 0);
    assert_eq!(decoded.imm, 10);
}

#[test]
fn decodes_rv32m_mul() {
    let zkvm = Zkvm::new();
    let word = 0x0252_01B3;
    let decoded = zkvm.decode_word(word).unwrap();

    assert_eq!(decoded.opcode, Rv32Opcode::Op);
    assert_eq!(decoded.extension, Rv32Extension::M);
    assert_eq!(decoded.kind, InstructionKind::M(MInstruction::Mul));
    assert_eq!(decoded.rd, 3);
    assert_eq!(decoded.rs1, 4);
    assert_eq!(decoded.rs2, 5);
    assert_eq!(decoded.imm, 0);
}

#[test]
fn rejects_unsupported_opcode() {
    let zkvm = Zkvm::new();
    let error = zkvm.decode_word(0xFFFF_FFFF).unwrap_err();

    assert!(matches!(
        error,
        ZkvmError::UnsupportedOpcode { opcode: 0x7F }
    ));
}
