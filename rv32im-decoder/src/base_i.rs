use crate::{
    error::{DecodeResult, DecoderError},
    formats::{BType, IType, JType, RType, SType, UType},
    instruction::DecodedInstruction,
};

pub fn decode_base(raw: u32, opcode: u8) -> DecodeResult<DecodedInstruction> {
    match opcode {
        0b0110111 => Ok(DecodedInstruction::Lui(UType::new(raw))),
        0b0010111 => Ok(DecodedInstruction::Auipc(UType::new(raw))),
        0b1101111 => Ok(DecodedInstruction::Jal(JType::new(raw))),
        0b1100111 => Ok(DecodedInstruction::Jalr(IType::new(raw))),
        0b1100011 => Ok(DecodedInstruction::Branch(BType::new(raw))),
        0b0000011 => Ok(DecodedInstruction::Load(IType::new(raw))),
        0b0100011 => Ok(DecodedInstruction::Store(SType::new(raw))),
        0b0010011 => Ok(DecodedInstruction::OpImm(IType::new(raw))),
        0b0110011 => decode_base_r(raw),
        0b1110011 => Ok(DecodedInstruction::System(raw)),
        _ => Err(DecoderError::UnknownOpcode { raw, opcode }),
    }
}

pub fn decode_base_r(raw: u32) -> DecodeResult<DecodedInstruction> {
    let r = RType::new(raw);
    match (r.funct7(), r.funct3()) {
        (0b0000000, _) | (0b0100000, _) => Ok(DecodedInstruction::Op(r)),
        (funct7, _) => Err(DecoderError::UnsupportedFunct7 { raw, funct7 }),
    }
}