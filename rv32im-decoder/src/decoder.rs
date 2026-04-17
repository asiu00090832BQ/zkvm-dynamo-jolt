use crate::{
    error::ZkvmError,
    instruction::Instruction,
    util::{funct3, funct7, imm_b, imm_i, imm_j, imm_s, imm_u, opcode, rd, rs1, rs2, shamt},
};

pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    match opcode(word) {
        0x37 => Ok(Instruction::Lui {
            rd: rd(word),
            imm: imm_u(word),
        }),
        0x17 => Ok(Instruction::Auipc {
            rd: rd(word),
            imm: imm_u(word),
        }),
        0x6f => Ok(Instruction::Jal {
            rd: rd(word),
            imm: imm_j(word),
        }),
        0x67 => match funct3(word) {
            0x0 => Ok(Instruction::Jalr {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            }),
            _ => Err(ZkvmError::InvalidInstruction(word)),
        },
        0x63 => match funct3(word) {
            0x0 => Ok(Instruction::Beq {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_b(word),
            }),
            0x1 => Ok(Instruction::Bne {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_b(word),
            }),
            0x4 => Ok(Instruction::Blt {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_b(word),
            }),
            0x5 => Ok(Instruction::Bge {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_b(word),
            }),
            0x6 => Ok(Instruction::Bltu {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_b(word),
            }),
            0x7 => Ok(Instruction::Bgeu {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_b(word),
            }),
            _ => Err(ZkvmError::InvalidInstruction(word)),
        },
        0x03 => match funct3(word) {
            0x0 => Ok(Instruction::Lb {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            }),
            0x1 => Ok(Instruction::Lh {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            }),
            0x2 => Ok(Instruction::Lw {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            }),
            0x4 => Ok(Instruction::Lbu {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            }),
            0x5 => Ok(Instruction::Lhu {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            }),
            _ => Err(ZkvmError::InvalidInstruction(word)),
        },
        0x23 => match funct3(word) {
            0x0 => Ok(Instruction::Sb {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_s(word),
            }),
            0x1 => Ok(Instruction::Sh {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_s(word),
            }),
            0x2 => Ok(Instruction::Sw {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_s(word),
            }),
            _ => Err(ZkvmError::InvalidInstruction(word)),
        },
        0x13 => match funct3(word) {
            0x0 => Ok(Instruction::Addi {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            }),
            0x2 => Ok(Instruction::Slti {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            }),
            0x3 => Ok(Instruction::Sltiu {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            }),
            0x4 => Ok(Instruction::Xori {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            }),
            0x6 => Ok(Instruction::Ori {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            }),
            0x7 => Ok(Instruction::Andi {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            }),
            0x1 if funct7(word) == 0x00 => Ok(Instruction::Slli {
                rd: rd(word),
                rs1: rs1(word),
                shamt: shamt(word),
            }),
            0x5 if funct7(word) == 0x00 => Ok(Instruction::Srli {
                rd: rd(word),
                rs1: rs1(word),
                shamt: shamt(word),
            }),
            0x5 if funct7(word) == 0x20 => Ok(Instruction::Srai {
                rd: rd(word),
                rs1: rs1(word),
                shamt: shamt(word),
            }),
            _ => Err(ZkvmError::InvalidInstruction(word)),
        },
        0x33 => match (funct7(word), funct3(word)) {
            (0x00, 0x0) => Ok(Instruction::Add {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x20, 0x0) => Ok(Instruction::Sub {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x00, 0x1) => Ok(Instruction::Sll {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x00, 0x2) => Ok(Instruction::Slt {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x00, 0x3) => Ok(Instruction::Sltu {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x00, 0x4) => Ok(Instruction::Xor {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x00, 0x5) => Ok(Instruction::Srl {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x20, 0x5) => Ok(Instruction::Sra {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x00, 0x6) => Ok(Instruction::Or {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x00, 0x7) => Ok(Instruction::And {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),

            (0x01, 0x0) => Ok(Instruction::Mul {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x01, 0x1) => Ok(Instruction::Mulh {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x01, 0x2) => Ok(Instruction::Mulhsu {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x01, 0x3) => Ok(Instruction::Mulhu {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x01, 0x4) => Ok(Instruction::Div {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x01, 0x5) => Ok(Instruction::Divu {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x01, 0x6) => Ok(Instruction::Rem {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            (0x01, 0x7) => Ok(Instruction::Remu {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            }),
            _ => Err(ZkvmError::InvalidInstruction(word)),
        },
        0x73 => match word >> 20 {
            0x000 => Ok(Instruction::Ecall),
            0x001 => Ok(Instruction::Ebreak),
            _ => Err(ZkvmError::UnsupportedInstruction(word)),
        },
        _ => Err(ZkvmError::InvalidInstruction(word)),
    }
}
