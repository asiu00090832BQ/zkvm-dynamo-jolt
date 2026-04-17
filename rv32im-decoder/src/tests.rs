use crate::{
    decode,
    instruction::{BranchKind, Instruction, OpImmKind, OpKind, SystemKind},
    Limb16,
};

#[test]
fn limb16_roundtrip() {
    let limbs = Limb16::from_u32(0xdead_beef);
    assert_eq!(limbs.lo, 0xbeef);
    assert_eq!(limbs.hi, 0xdead);
    assert_eq!(limbs.to_u32(), 0xdead_beef);
}

#[test]
fn decode_add() {
    let word = 0x0020_81b3;
    let inst = decode(word).unwrap();
    assert_eq!(
        inst,
        Instruction::Op {
            kind: OpKind::Add,
            rd: 3,
            rs1: 1,
            rs2: 2
        }
    );
}

#[test]
fn decode_mul() {
    let word = 0x0273_02b3;
    let inst = decode(word).unwrap();
    assert_eq!(
        inst,
        Instruction::Op {
            kind: OpKind::Mul,
            rd: 5,
            rs1: 6,
            rs2: 7
        }
    );
}

#[test]
fn decode_branch_immediate() {
    let word = 0x0020_8863;
    let inst = decode(word).unwrap();
    assert_eq!(
        inst,
        Instruction::Branch {
            kind: BranchKind::Beq,
            rs1: 1,
            rs2: 2,
            imm: 16
        }
    );
}

#[test]
fn decode_shift_immediate() {
    let word = 0x0031_1093;
    let inst = decode(word).unwrap();
    assert_eq!(
        inst,
        Instruction::OpImm {
            kind: OpImmKind::Slli,
            rd: 1,
            rs1: 2,
            imm: 3
        }
    );
}

#[test]
fn decode_ecall() {
    let inst = decode(0x0000_0073).unwrap();
    assert_eq!(inst, Instruction::System { kind: SystemKind::Ecall });
}

#[test]
fn reject_compressed() {
    assert!(decode(0x0000_0001).is_err());
}
