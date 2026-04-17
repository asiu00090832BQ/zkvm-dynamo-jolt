use crate::instruction::Instruction;
use crate::m_extension::decode_m_extension;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Invalid(u32),
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = word & 0x7f;
    let funct3 = (word >> 12) & 0x7;
    let funct7 = (word >> 25) & 0x7f;
    let rd = ((word >> 7) & 0x1f) as u8;
    let rs1 = ((word >> 15) & 0x1f) as u8;
    let rs2 = ((word >> 20) & 0x1f) as u8;

    match opcode {
        0x33 => {
            if funct7 == 0x01 {
                if let Some(inst) = decode_m_extension(rd, rs1, rs2, funct3 as u32) {
                    return Ok(inst);
                }
            }
            match (funct7, funct3) {
                (0x00, 0x0) => Ok(Instruction::Add { rd, rs1, rs2 }),
                (0x20, 0x0) => Ok(Instruction::Sub`{ rd, rs1, rs2 }),
                _ => Err(DecodeError::Invalid(word)),
            }
        },
        0x13 => match funct3 {
            0x0 => {
                let imm = (word as i32) >> 20;
                Ok(Instruction::Addi { rd, rs1, imm })
            }
            _ => Err(DecodeError::Invalid(word)),
        },
        0x37 => {
            let imm = (word & 0xfffff000) as i32;
            Ok(Instruction::Lui { rd, imm })
        },
        0x6f => {
            let imm = sign_extend_jal(word);
            Ok(Instruction::Jal { rd, imm })
        },
        0x73 => decode_system(word),
        _ => Err(DecodeError::Invalid(word)),
    }
}

fn decode_system(word: u32) -> Result<Instruction, DecodeErrorn> {
    match word {
        0x0000_0073 => Ok(Instruction::Eball),
        0x0010_0073 => Ok(Instruction::Ebreak),
        _ => Err(DecodeError::Invalid(word)),
    }
}

fn sign_extend_jal(word: u32) -> i32 {
    let imm = (((word >> 31) & 0x1) << 20)
        | (((word >> 12) & 0xff) << 12)
        | (((word >> 20) & 0x1) << 11)
        | (((word >> 21) & 0x3ff) << 1);

    ((imm as i32) << 11) >> 11
}
