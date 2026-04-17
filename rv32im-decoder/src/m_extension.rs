use crate::decoder::DecodeError;
use crate::instruction::Instruction;
use crate::limbs::Limb16;

pub fn decode_m_instruction(funct3: u8, rd: u8, rs1: u8, rs2: u8) -> Result<Instruction, DecodeError> {
    match funct3 {
        0b000 => Ok(Instruction::Mul { rd, rs1, rs2 }),
        0b001 => Ok(Instruction::Mulh { rd, rs1, rs2 }),
        0b010 => Ok(Instruction::Mulhsu { rd, rs1, rs2 }),
        0b011 => Ok(Instruction::Mulhu { rd, rs1, rs2 }),
        0b100 => Ok(Instruction::Div { rd, rs1, rs2 }),
        0b101 => Ok(Instruction::Divu { rd, rs1, rs2 }),
        0b110 => Ok(Instruction::Rem { rd, rs1, rs2 }),
        0b111 => Ok(Instruction::Remu { rd, rs1, rs2 }),
        _ => Err(DecodeError::InvalidFunct3 { opcode: 0b0110011, funct3 }),
    }
}

pub fn mul_low_u32(lhs: u32, rhs: u32) -> u32 {
    let product = Limb16::decompose(lhs).multiply(Limb16::decompose(rhs));
    product.low32()
}

pub fn mulh_i32(lhs: i32, rhs: i32) -> u32 {
    (((lhs as i64) * (rhs as i64)) >> 32) as u32
}

pub fn mulhsu_i32_u32(lhs: i32, rhs: u32) -> u32 {
    (((lhs as i64) * (rhs as u64 as i64)) >> 32) as u32
}

pub fn mulhu_u32(lhs: u32, rhs: u32) -> u32 {
    let product = Limb16::decompose(lhs).multiply(Limb16::decompose(rhs));
    product.high32()
}

pub fn div_i32(lhs: i32, rhs: i32) -> u32 {
    if rhs == 0 { u32::MAX } else if lhs == i32::MIN && rhs == -1 { lhs as u32 } else { (lhs / rhs) as u32 }
}

pub fn divu_u32(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 { u32::MAX } else { lhs / rhs }
}

pub fn rem_i32(lhs: i32, rhs: i32) -> u32 {
    if rhs == 0 { lhs as u32 } else if lhs == i32::MIN && rhs == -1 { 0 } else { (lhs % rhs) as u32 }
}

pub fn remu_u32(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 { lhs } else { lhs % rhs }
}
