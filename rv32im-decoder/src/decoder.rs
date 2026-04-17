use crate::types:{Instruction, DecodeError};

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    if (word & 0x3) != 0x3 {
        return Err(DecodeError::InvalidWord(word));
    }

    let opcode = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as usize;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = ((word >> 15) & 0x1f) as usize;
    let rs2 = ((word >> 20) & 0x1f) as usize;
    let funct7 = (word >> 25) & 0x7f;

    match opcode {
        0x33 => {
            match (funct3, funct7) {
                (0x0, 0x00) => Ok(Instruction::Add { rd, rs1, rs2 }),
                (0x0, 0x20) => Ok(Instruction::Sub { rd, rs1, rs2 }),
                (0x0, 0x01) => Ok(Instruction::Mul { rd, rs1, rs2 }),
                (0x1, 0x01) => Ok(Instruction::Mulh { rd, rs1, rs2 }),
                (0x2, 0x01) => Ok(Instruction::Mulhsu { rd, rs1, rs2 }),
                (0x3, 0x01) => Ok(Instruction::Mulhu { rd, rs1, rs2 }),
                (0x4, 0x01) => Ok(Instruction::Div { rd, rs1, rs2 }),
                (0x5, 0x01) => Ok(Instruction::Divu { rd, rs1, rs2 }),
                (0x6, 0x01) => Ok(Instruction::Rem { rd, rs1, rs2 }),
                (0x7, 0x01) => Ok(Instruction::Remu { rd, rs1, rs2 }),
                _ => Ok(Instruction::Invalid(word)),
            }
        }
        0x13 => {
            let imm = (word as i32) >> 20;
            match funct3 {
                0x0 => Ok(Instruction::Addi { rd, rs1, imm }),
                _ => Ok(Instruction::Invalid(word)),
            }
        }
        0x37 => {
            let imm = (word & 0xfffff000) as i32;
            Ok(Instruction::Lui { rd, imm })
        }
        0x6f => {
            let imm20 = (word >> 31) & 1;
            let imm10_1 = (word >> 21) & 0x3ff;
            let imm11 = (word >> 20) & 1;
            let imm19_12 = (word >> 12) & 0xff;
            let mut imm = ((imm20 as i32) << 20) | ((imm19_12 as i32) << 12) | ((imm11 as i32) << 11) | ((imm10_1 as i32) << 1);
            if imm20 != 0 { imm |= !0xfffff; }
            Ok(Instruction::Jal { rd, imm })
        }
        0x73 => {
            match word {
                0x0000_0073 => Ok(Instruction::Ecall),
                0x0010_0073 => Ok(Instruction::Ebreak),
                _ => Ok(Instruction::Invalid(word)),
            }
        }
        _ => Ok(Instruction::Invalid(word)),
    }
}
