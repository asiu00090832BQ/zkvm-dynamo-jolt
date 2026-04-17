use crate::{
    base_i,
    error::{DecodeResult, DecoderError},
    formats::{RawFields, RType},
    instruction::DecodedInstruction,
    m_extension,
};

pub fn decode_word(raw: u32) -> DecodeResult<DecodedInstruction> {
    let fields = RawFields::new(raw);
    match fields.opcode() {
        0b0110011 => decode_op(raw),
        opcode => base_i::decode_base(raw, opcode),
    }
}

fn decode_op(raw: u32) -> DecodeResult<DecodedInstruction> {
    let r = RType::new(raw);
    match r.funct7() {
        0b0000001 => m_extension::decode_m_instruction(raw),
        0b0000000 | 0b0100000 => base_i::decode_base_r(raw),
        funct7 => Err(DecoderError::UnsupportedFunct7 { raw, funct7 }),
    }
}