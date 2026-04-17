use crate::{
    decode::decode_word,
    error::ZkvmError,
    types::{Instruction, Op, OpImm, Register},
    vm::Zkvm,
};

fn encode_r(funct7: u32, rs2: u32, rs1: u32, funct3: u32, rd: u32, opcode: u32) -> u32 {
    (funct7 << 25)
        | (rs2 << 20)
        | (rs1 << 15)
        | (funct3 << 12)
        | (rd << 7)
        | opcode
}

fn encode_i(imm: i32, rs1: u32, funct3: u32, rd: u32, opcode: u32) -> u32 {
    (((imm as u32) & 0x0fff) << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}

#[test]
fn decode_addi_round_trips() {
    let word = encode_i(123, 2, 0b000, 1, 0b0010011);
    let instruction = decode_word(word).unwrap();

    assert_eq!(
        instruction,
        Instruction::OpImm {
            kind: OpImm::Addi,
            rd: Register::new(1).unwrap(),
            rs1: Register::new(2).unwrap(),
            imm: 123,
        }
    );
}

#[test]
fn decode_sub_round_trips() {
    let word = encode_r(0b0100000, 3, 2, 0b000, 1, 0b0110011);
    let instruction = decode_word(word).unwrap();

    assert_eq!(
        instruction,
        Instruction::Op {
            kind: Op::Sub,
            rd: Register::new(1).unwrap(),
            rs1: Register::new(2).unwrap(),
            rs2: Register::new(3).unwrap(),
        }
    );
}

#[test]
fn lemma_6_1_1_partitions_all_m_funct3_values() {
    let expected = [
        Op::Mul,
        Op::Mulh,
        Op::Mulhsu,
        Op::Mulhu,
        Op::Div,
        Op::Divu,
        Op::Rem,
        Op::Remu,
    ];

    for (funct3, kind) in expected.into_iter().enumerate() {
        let word = encode_r(0b0000001, 3, 2, funct3 as u32, 1, 0b0110011);
        let instruction = decode_word(word).unwrap();

        assert_eq!(
            instruction,
            Instruction::Op {
                kind,
                rd: Register::new(1).unwrap(),
                rs1: Register::new(2).unwrap(),
                rs2: Register::new(3).unwrap(),
            }
        );
    }
}

#[test]
fn rejects_compressed_words() {
    let error = decode_word(0x0000_0001).unwrap_err();
    assert_eq!(
        error,
        ZkvmError::CompressedInstructionUnsupported { word: 0x0000_0001 }
    );
}

#[test]
fn zkvm_executes_mul_and_divu() {
    let mut vm = Zkvm::new();
    vm.write_reg(Register::new(1).unwrap(), 6);
    vm.write_reg(Register::new(2).unwrap(), 7);

    let mul = encode_r(0b0000001, 2, 1, 0b000, 3, 0b0110011);
    vm.step_word(mul).unwrap();
    assert_eq!(vm.read_reg(Register::new(3).unwrap()), 42);
    assert_eq!(vm.pc(), 4);

    vm.write_reg(Register::new(4).unwrap(), 123);
    vm.write_reg(Register::new(5).unwrap(), 0);

    let divu = encode_r(0b0000001, 5, 4, 0b101, 6, 0b0110011);
    vm.step_word(divu).unwrap();
    assert_eq!(vm.read_reg(Register::new(6).unwrap()), u32::MAX);
    assert_eq!(vm.pc(), 8);
}
