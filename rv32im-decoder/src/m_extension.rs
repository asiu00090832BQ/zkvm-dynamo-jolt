use crate::{
    error::{DecodeResult, DecoderError},
    formats::RType,
    instruction::{DecodedInstruction, MInstruction},
    invariants,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limb16 {
    pub lo: u16,
    pub hi: u16,
}

pub fn decode_m_instruction(raw: u32) -> DecodeResult<DecodedInstruction> {
    let r = RType::new(raw);
    invariants::ensure_register(r.rd())?;
    invariants::ensure_register(r.rs1())?;
    invariants::ensure_register(r.rs2())?;

    let op = match r.funct3() {
        0b000 => MInstruction::Mul,
        0b001 => MInstruction::Mulh,
        0b010 => MInstruction::Mulhsu,
        0b011 => MInstruction::Mulhu,
        0b100 => MInstruction::Div,
        0b101 => MInstruction::Divu,
        0b110 => MInstruction::Rem,
        0b111 => MInstruction::Remu,
        funct3 => return Err(DecoderError::UnsupportedFunct3 { raw, funct3 }),
    };

    invariants::ensure_utf8(op.mnemonic())?;
    invariants::ensure_zkvm_symbol_parity()?;
    Ok(DecodedInstruction::MulDiv(op, r))
}

pub fn decompose_u32(value: u32) -> Limb16 {
    Limb16 {
        lo: value as u16,
        hi: (value >> 16) as u16,
    }
}

pub fn plan_mul_limbs(lhs: u32, rhs: u32) -> [(u32, u32); 4] {
    let l = decompose_u32(lhs);
    let r = decompose_u32(rhs);
    [
        (l.lo as u32, r.lo as u32),
        (l.lo as u32, r.hi as u32),
        (l.hi as u32, r.lo as u32),
        (l.hi as u32, r.hi as u32),
    ]
}

pub fn plan_div_semantics(_lhs: i32, _rhs: i32) -> DecodeResult<()> {
    Ok(())
}
