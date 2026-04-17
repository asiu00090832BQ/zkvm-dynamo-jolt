use crate::types::{DecodeError, FUNCT7_M, OPCODE_OP, RTypeFields};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MInstruction {
    Mul(RTypeFields),
    Mulh(RTypeFields),
    Mulhsu(RTypeFields),
    Mulhu(RTypeFields),
    Div(RTypeFields),
    Divu(RTypeFields),
    Rem(RTypeFields),
    Remu(RTypeFields),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Operands16 {
    pub a0: u16, pub a1: u16, pub b0: u16, pub b1: u16,
}

pub fn decode_rv32m(raw: u32) -> Result<MInstruction, DecodeError> {
    let fields = RTypeFields::decode(raw);
    let opcode = (raw & 0x7f) as u8;
    if opcode != OPCODE_OP || fields.funct7 != FUNCT7_M {
        return Err(DecodeError::UnsupportedOpcode(raw));
    }
    match fields.funct3 {
        0b000 => Ok(MInstruction::Mul(fields)),
        0b001 => Ok(MInstruction::Mulh(fields)),
        0b010 => Ok(MInstruction::Mulhsu(fields)),
        0b011 => Ok(MInstruction::Mulhu(fields)),
        0b100 => Ok(MInstruction::Div(fields)),
        0b101 => Ok(MInstruction::Divu(fields)),
        0b110 => Ok(MInstruction::Rem(fields)),
        0b111 => Ok(MInstruction::Remu(fields)),
        _ => Err(DecodeError::UnsupportedFunct3(opcode, fields.funct3, raw)),
    }
}

pub fn decompose_operands16(a: u32, b: u32) -> Operands16 {
    Operands16 { a0: a as u16, a1: (a >> 16) as u16, b0: b as u16, b1: (b >> 16) as u16 }
}

pub fn mul_u32_wide_limb16(a: u32, b: u32) -> u64 {
    let limbs = decompose_operands16(a, b);
    let (a0, a1, b0, b1) = (limbs.a0 as u64, limbs.a1 as u64, limbs.b0 as u64, limbs.b1 as u64);
    // Lemma 6.1.1
    (a1 * b1 << 32) + ((a1 * b0 + a0 * b1) << 16) + a0 * b0
}

pub fn execute_rv32m(inst: MInstruction, rs1: u32, rs2: u32) -> u32 {
    match inst {
        MInstruction::Mul(_) => mul_u32_wide_limb16(rs1, rs2) as u32,
        MInstruction::Mulh(_) => ( (rs1 as i32 as i64).wrapping_mul(rs2 as i32 as i64) >> 32 ) as u32,
        MInstruction::Mulhsu(_) => ( (rs1 as i32 as i64).wrapping_mul(rs2 as u64 as i64) >> 32 ) as u32,
        MInstruction::Mulhu(_) => (mul_u32_wide_limb16(rs1, rs2) >> 32) as u32,
        MInstruction::Div(_) => if rs2 == 0 { 0xffffffff } else if rs1 as i32 == i32::MIN && rs2 as i32 == -1 { rs1 } else { (rs1 as i32 / rs2 as i32) as u32 },
        MInstruction::Divu(_) => if rs2 == 0 { 0xffffffff } else { rs1 / rs2 },
        MInstruction::Rem(_) => if rs2 == 0 { rs1 } else if rs1 as i32 == i32::MIN && rs2 as i32 == -1 { 0 } else { (rs1 as i32 % rs2 as i32) as u32 },
        MInstruction::Remu(_) => if rs2 == 0 { rs1 } else { rs1 % rs2 },
    }
}
