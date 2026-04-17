pub mod types;
pub mod util;

pub use crate::types::{Instruction, DecodeError, Register};

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
                    0x0 => Ok(Instruction::Mul { rd, rs1, rs2 }),
                    0x1 => Ok(Instruction::Mulh { rd, rs1, rs2 }),
                    0x2 => Ok(Instruction::Mulhsu { rd, rs1, rs2 }),
                    0x3 => Ok(Instruction::Mulhu { rd, rs1, rs2 }),
                    0x4 => Ok(Instruction::Div { rd, rs1, rs2 }),
                    0x5 => Ok(Instruction::Divu { rd, rs1, rs2 }),
                    0x6 => Ok(Instruction::Rem { rd, rs1, rs2 }),
                    0x7 => Ok(Instruction::Remu { rd, rs1, rs2 }),
                    _ => Err(DecodeError::IllegalInstruction(word)),
                }
            } else {
                match (funct3, funct7) {
                    (0x0, 0x00) => Ok(Instruction::Add { rd, rs1, rs2 }),
                    (0x0, 0x20) => Ok(Instruction::Sub { rd, rs1, rs2 }),
                    _ => Err(DecodeError::IllegalInstruction(word)),
                }
            }
        }
        0x37 => Ok(Instruction::Lui { rd, imm: util::imm_u(word) }),
        0x17 => Ok(Instruction::Auipc { rd, imm: util::imm_u(word) }),
        0x73 => {
            if word == 0x00000073 { Ok(Instruction::Ecall) }
            else { Err(DecodeError::IllegalInstruction(word)) }
        }
        _ => Err(DecodeError::IllegalInstruction(word)),
    }
}