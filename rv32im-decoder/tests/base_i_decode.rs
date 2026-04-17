use rv32im_decoder::{decode, decode_bytes, BranchKind, Instruction, LoadKind, StoreKind};

#[test]
fn decodes_core_base_i_examples() {
    assert!(matches!(
        decode(0x1234_5537).unwrap(),
        Instruction::Lui(..) 
    ));

    assert!(matches!(
        decode(0x02a0_0093).unwrap(),
        Instruction::OpImm { .. }
    ));

    assert!(matches!(
        decode(0x4073_5293).unwrap(),
        Instruction::OpImm { .. }
    ));

    assert!(matches!(
        decode(0x0020_8863).unwrap(),
        Instruction::Branch { .. }
    ));

    assert!(matches!(
        decode(0x0032_2623).unwrap(),
        Instruction::Store { .. }
    ));

    assert!(matches!(
        decode(0x0200_00ef).unwrap(),
        Instruction::Jal(..)
    ));
}

#[test]
fn decodes_little_endian_bytes() {
    let instr = decode_bytes([0x93, 0x00, 0xa0, 0x02]).unwrap();
    assert!(matches!(instr, Instruction::OpImm { .. }));
}
