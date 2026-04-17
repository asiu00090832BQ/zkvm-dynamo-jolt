use crate::opcodes::Instruction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Invalid(u32),
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = word & 0x7F;
    let rd = ((word >> 7) & 0x1F) as u8;
    let funct3 = ((word >> 12) & 0x07) as u8;
    let rs1 = ((word >> 15) & 0x1F) as u8;
    let rs2 = ((word >> 20) & 0x1F) as u8;
    let funct7 = ((word >> 25) & 0x7F) as u8;

    match opcode {
        0x33 => {
            // R-type
            match (funct7, funct3) {
                (0x00, 0x00) => Ok(Instruction::Add { rd, rs1, rs2 }),
                (0x20, 0x00) => Ok(Instruction::Sub { rd, rs1, rs2 }),
                (0x01, 0x00) => Ok(Instruction::Mul { rd, rs1, rs2 }), // M-extension
                // Add more M-extension and R-type here
                _ => Ok(Instruction::Invalid(word)),
            }
        }
        0x13 => {
            // I-type ALU
            let imm = (word as i32) >> 20;
            match funct3 {
                0x00 => Ok(Instruction::Addi { rd, rs1, imm }),
                _ => Ok(Instruction::Invalid(word)),
            }
        }
        0x37 => {
            // LUI
            let imm = (word & 0xFFFFF000) as i32;
            Ok(Instruction::Lui { rd, imm })
        }
        0x6F => {
            // JAL
            let sign = ((word >> 31) & 0x1) as i32;
            let imm19_12 = ((word >> 12) & 0xFF) as i32;
            let imm11 = ((word >> 20) & 0x1) as i32;
            let imm10_1 = ((word >> 21) & 0x3FF) as i32;
            let imm = (sign << 20) | (imm19_12 << 12) | (imm11 << 11) | (imm10_1 << 1);
            let imm = (imm << 11) >> 11; // Sign extend i21 to i32
            Ok(Instruction::Jal { rd, imm })
        }
        0x73 => {
            match word {
                0x00000073 => Ok(Instruction::Ecall),
                0x00100073 => Ok(Instruction::Ebreak),
                _ => Ok(Instruction::Invalid(word)),
            }
        }
        _ => Ok(Instruction::Invalid(word)),
    }
}
