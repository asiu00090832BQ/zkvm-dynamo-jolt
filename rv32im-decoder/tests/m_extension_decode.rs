use rv32im_decoder::{decode, Instruction, OpKind};

#[test]
fn decodes_m_extension_examples() {
    let instr = decode(0x0273_02b3).unwrap();
    if let Instruction::Op { kind, .. } = instr {
        assert_eq!(kind.mnemonic(), "mul");
    } else {
        panic!("expected Op instruction");
    }

    let instr = decode(0x02a4_d433).unwrap();
    if let Instruction::Op { kind, .. } = instr {
        assert_eq!(kind.mnemonic(), "divu");
    } else {
        panic!("expected Op instruction");
    }
}
