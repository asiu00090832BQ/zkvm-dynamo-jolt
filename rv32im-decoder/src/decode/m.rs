use crate::{
    error::{DecodeError, DecodeResult},
    fields::Fields,
    instruction::{Instruction, OpKind},
};

pub fn decode(fields: Fields) -> DecodeResult<Instruction> {
    let kind = match fields.funct3 {
        0b000 => OpKind::Mul,
        0b001 => OpKind::Mulh,
        0b010 => OpKind::Mulhsu,
        0b011 => OpKind::Mulhu,
        0b100 => OpKind::Div,
        0b101 => OpKind::Divu,
        0b110 => OpKind::Rem,
        0b111 => OpKind::Remu,
        _ => {
            return Err(DecodeError::UnsupportedFunct3 {
                opcode: fields.opcode,
                funct3: fields.funct3,
            })
        }
    };

    Ok(Instruction::Op {
        kind,
        rd: fields.rd_register(),
        rs1: fields.rs1_register(),
        rs2: fields.rs2_register(),
    })
}
