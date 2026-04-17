use crate::decoder::m_extension::{decompose, lemma_6_1_1_parity, mul_u32_wide, Limb16};
use crate::error::ZkvmError;
use crate::fields::{IType, RType};
use crate::instruction::{ArithmeticOp, Instruction, OpImmOp, SystemOp};
use crate::decode;

#[test]
fn decodes_addi() {
    let instruction = decode(0x0010_0093).unwrap();

    assert_eq!(
        instruction,
        Instruction::OpImm(
            OpImmOp::Addi,
            IType {
                rd: 1,
                rs1: 0,
                imm: 1,
                funct3: 0,
            },
        )
    );
}

#[test]
fn decodes_add() {
    let instruction = decode(0x0020_81b3).unwrap();

    assert_eq!(
        instruction,
        Instruction::Op(
            ArithmeticOp::Add,
            RType {
                rd: 3,
                rs1: 1,
                rs2: 2,
                funct3: 0,
                funct7: 0,
            },
        )
    );
}

#[test]
fn decodes_mul() {
    let instruction = decode(0x0220_81b3).unwrap();

    assert_eq!(
        instruction,
        Instruction::Op(
            ArithmeticOp::Mul,
            RType {
                rd: 3,
                rs1: 1,
                rs2: 2,
                funct3: 0,
                funct7: 1,
            },
        )
    );
}

#[test]
fn decodes_ecall() {
    let instruction = decode(0x0000_0073).unwrap();
    assert_eq!(instruction, Instruction::System(SystemOp::Ecall));
}

#[test]
fn rejects_compressed_words() {
    let error = decode(0x0000_0000).unwrap_err();
    assert!(matches!(
        error,
        ZkvmError::InvalidEncoding {
            word: 0x0000_0000,
            reason: "compressed instructions are not supported",
        }
    ));
}

#[test]
fn lemma_6_1_1_decomposition_matches_spec() {
    assert_eq!(
        decompose(0x1234_5678),
        Limb16 {
            low: 0x5678,
            high: 0x1234,
        }
    );
}

#[test]
fn lemma_6_1_1_parity_holds_for_representative_vectors() {
    let vectors = [
        (0x0000_0000, 0x0000_0000),
        (0x0000_0001, 0x0000_0001),
        (0x0000_ffff, 0x0000_ffff),
        (0xffff_ffff, 0x0000_0002),
        (0x1234_5678, 0x9abc_def0),
        (0x8000_0000, 0x7fff_ffff),
    ];

    for (lhs, rhs) in vectors {
        assert!(lemma_6_1_1_parity(lhs, rhs));
        assert_eq!(mul_u32_wide(lhs, rhs), (lhs as u64) * (rhs as u64));
    }
}
