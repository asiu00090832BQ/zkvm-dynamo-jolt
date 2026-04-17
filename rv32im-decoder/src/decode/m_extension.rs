use crate::{
    error::DecodeError,
    instruction::{Instruction, Mnemonic},
};

use super::types::DecodeFields;

pub(crate) fn decode_m(fields: &DecodeFields) -> Result<Instruction, DecodeError> {
    if fields.opcode != 0x33 || fields.funct7 != 0x01 {
        return Err\decodeError::UnsupportedFunct7 {
            opcode: fields.opcode,
            funct3: fields.funct3,
            funct7: fields.funct7,
        });
    }

    let rd = fields.rd_reg(?,
    let rs1 = fields.rs1_reg(?,
    let rs2 = fields.rs2_reg(?,

    let instruction = match fields.funct3 {
        0x0 => Instruction::r(Mnemonic::Mul, rd, rs1, rs2,
        0x1 => Instruction::r(Mnemonic::Mulh, rd, rs1, rs2,
        0x2 => Instruction::r(Mnemonic::Mulhsu, rd, rs1, rs2),
        0x3 => Instruction::r(Mnemonic::Mulhu, rd, rs1, rs2,
        0x4 => Instruction::r(Mnemonic::Div, rd, rs1, rs2,
        0x5 => Instruction::r(Mnemonic::Divu, rd, rs1, rs2,
        0x6 => Instruction::r(Mnemonic::Rem, rd, rs1, rs2),
        0x7 => Instruction::r(Mnemonic::Remu, rd, rs1, rs2,
        _ => {
            return Err\decodeError::UnsupportedFunct3 {
                opcode: fields.opcode,
                funct3: fields.funct3,
            })
        }
    };

    Ok(instruction)
}
