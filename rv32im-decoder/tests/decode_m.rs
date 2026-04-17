use rv32im_decoder::{decode, Instruction, OpKind, Register};

#[test]
fn decodes_mul() {
    let word = 0x0220_81b3;

    let instruction = decode(word).expect("mul should decode");

    assert_eq!(
        instruction,
        Instruction::Op {
            kind: OpKind::Mul,
            rd: Register::from_u5(3),
            rs1: Register::from_u5(1),
            rs2: Register::from_u5(2),
        }
    );
}

#[test]
fn decodes_divu() {
    let word = 0x0252_5333;

    let instruction = decode(word).expect("divu should decode");

    assert_eq!(
        instruction,
        Instruction::Op {
            kind: OpKind::Divu,
            rd: Register::from_u5(6),
            rs1: Register::from_u5(4),
            rs2: Register::from_u5(5),
        }
    );
}

#[test]
fn decodes_remu() {
    let word = 0x02a4_f433;

    let instruction = decode(word).expect("remu should decode");

    assert_eq!(
        instruction,
        Instruction::Op {
            kind: OpKind::Remu,
            rd: Register::from_u5(8),
            rs1: Register::from_u5(9),
            rs2: Register::from_u5(10),
        }
    );
}
