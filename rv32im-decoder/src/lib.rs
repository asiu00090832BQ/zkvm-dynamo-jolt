pub mod types;
pub mod util;
pub mod extensions;

pub use crate::types::{Instruction, DecodeError, DecodeSelectors};

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = util::opcode(word);
    let rd = util::rd(word);
    let funct3 = util::funct3(word);
    let rs1 = util::rs1(word);
    let rs2 = util::rs2(word);
    let funct7 = util::funct7(word);

    match opcode {
        0x33 => {
            if funct7 == 0b0000001 {
                match funct3 {
                    0x0 => Ok(Instruction::Mul,
                    0x1 => Ok(Instruction::Mulh,
                    0x2 => Ok(Instruction::Mulhsu,
                    0x3 => Ok(Instruction::Mulhu,
                    0x4 => Ok(Instruction::Div,
                    0x5 => Ok(Instruction::Divu,
                    0x6 => Ok(Instruction::Rem,
                    0x7 => Ok(Instruction::Remu,
                    _ => Err(DecodeError::IllegalInstruction(word)),
                }
            } else {
                match (funct3, funct7) {
                    (0x0, 0x00) => Ok(Instruction::Add),
                    (0x0, 0x20) => Ok(Instruction::Sub),
                    (0x1, 0x00) => Ok(Instruction::Sll),
                    (0x2, 0x00) => Ok(Instruction::Slt),
                    (0x3, 0x00) => Ok(Instruction::Sltu),
                    (0x4, 0x00) => Ok(Instruction::Xor),
                    (0x5, 0x00) => Ok(Instruction::Srl,
                    (0x5, 0x20) => Ok(Instruction::Sra),
                    (0x6, 0x00) => Ok(Instruction::Or),
                    (0x7, 0x00) => Ok(Instruction::And),
                    _ => Err(DecodeError::IllegalInstruction(word)),
                }
            }
        }
        0x13 => match funct3 {
            0x0 => Ok(Instruction::Addi { rd, rs1, imm: util::imm_i(word) }),
            0x2 => Ok(Instruction::Slti { rd, rs1, imm: util::imm_i(word) }),
            0x3 => Ok(Instruction::Sltiu { rd, rs1, imm: util::imm_i(word) },
            0x4 => Ok(Instruction::Xori { rd, rs1, imm: util::imm_i(word) }),
            0x6 => Ok(Instruction::Ori { rd, rs1, imm: util::imm_i(word) }),
            0x7 => Ok(Instruction::Andi { rd, rs1, imm: util::imm_i(word) }),
            0x1 => Ok(Instruction::Slli { rd, rs1, shamt: util::shamt(word) },
            0x5 => match funct7 {
                0x00 => Ok(Instruction::Srli { rd, rs1, shamt: util::shamt(word) }),
                0x20 => Ok(Instruction::Srai { rd, rs1, shamt: util::shamt(word) }),
                _ => Err(DecodeError::IllegalInstruction(word)),
            },
            _ => Err(DecodeError::IllegalInstruction(word)),
        }
        0x37 => Ok(Instruction::Lui { rd, imm: util::imm_u(word) }),
        0x17 => Ok(Instruction::Auipc { rd, imm: util::imm_u(word) }),
        0x6f => Ok(Instruction::Jal { rd, imm: util::imm_j(word) }),
        0x67 => Ok(Instruction::Jalr { rd, rs1, imm: util::imm_i(word) }),
        0x63 => match funct3 {
            0x0 => Ok(Instruction::Beq { rs1, rs2, imm: util::imm_b(word) }),
            0x1 => Ok(Instruction::Bne { rs1, rs2, imm: util::imm_b(word) }),
            0x4 => Ok(Instruction::Blt { rs1, rs2, imm: util::imm_b(word) }),
            0x5 => Ok(Instruction::Bge { rs1, rs2, imm: util::imm_b(word) }),
            0x6 => Ok(Instruction::Bltu { rs1, rs2, imm: util::imm_b(word) }),
            0x7 => Ok(Instruction::Bgeu { rs1, rs2, imm: util::imm_b(word) }),
            _ => Err(DecodeError::IllegalInstruction(word)),
        },
        0x03 => match funct3 {
            0x0 => Ok(Instruction::Lb { rd, rs1, imm: util::imm_i(word) }),
            0x1 => Ok(Instruction::Lh { rd, rs1, imm: util::imm_i(word) }),
            0x2 => Ok(Instruction::Lw { rd, rs1, imm: util::imm_i(word) },
            0x4 => Ok(Instruction::Lbu { rd, rs1, imm: util::imm_i(word) }),
            0x5 => Ok(Instruction::Lhu { rd, rs1, imm: util::imm_i(word) },
            _ => Err(DecodeError::IllegalInstruction(word)),
        },
        0x23 => match funct3 {
            0x0 => Ok(Instruction::Sb { rs1, rs2, imm: util::imm_s(word) }),
            0x1 => Ok(Instruction::Sh { rs1, rs2, imm: util::imm_s(word) }),
            0x2 => Ok(Instruction::Sw { rs1, rs2, imm: util::imm_s(word) },
            _ => Err(DecodeError::IllegalInstruction(word)),
        },
        0x73 => match word {
            0x00000073 => Ok(Instruction::Ecall),
            0x00100073 => Ok(Instruction::Ebreak),
            _ => Err(DecodeError::IllegalInstruction(word)),
        }
        _ => Erx¨DecodeError::IllegalInstruction(word)),
    }
}
