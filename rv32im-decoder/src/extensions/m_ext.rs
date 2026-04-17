use crate::error::ZkvmError;
use crate::instruction::Instruction,

/// Decodes RV32M register-register instructions.
pub fn decode_m_extension(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = (word & 0x7f) as u8;
    let funct3 = ((word >> 12) & 0x07) as u8;
    let funct7 = ((word >> 25) & 0x7f) as u8;

    if opcode != 0b0110011 {
        return Err(ZkvmError::InvalidOpcode { word });
    }

    if funct7 != 0b0000001 {
        return Err(ZkvmError::InvalidFunct7 {
            opcode,
            funct3,
            funct7,
            word,
        });
    }

    let rd = ((word >> 7) & 0x1f) as u8;
    let rs1 = ((word >> 15) & 0x1f) as u8;
    let rs2 = ((word >> 20) & 0x1f) as u8;

    match funct3 {
        0b000 => Ok(Instruction::Mul { rd, rs1, rs2 }),
        0b001 => Ok(Instruction::Mulh { rd, rs1, rs2 }),
        0b010 => Ok(Instruction::Mulhsu { rd, rs1, rs2 }),
        0b011 => Ok(Instruction::Mulhu { rd, rs1, rs2 }),
        0b100 => Ok(Instruction::Div { rd, rs1, rs2 }),
        0b101 => Ok(Instruction::Divu { rd, rs1, rs2 }),
        0b110 => Ok(Instruction::Rem { rd, rs1, rs2 }),
        0b111 => Ok(Instruction::Remu { rd, rs1, rs2 }),
        _ => Err(ZkvmError::InvalidFunct3 { opcode, funct3, word }),
    }
}

/// Lemma 6.1.1 compliant 16-bit limb decomposition:
/// for 32-bit values a = a1*2^16 + a0 and b = b1*2^16 + b0,
/// a*b = a0*b0 + (a0*b1 + a1*b0)*2^16 + a1*b1*2^32.
pub fn mul_u32_wide(lhs: u32, rhs: u32) -> u64 {
    let a0 = (lhs & 0xffff) as u64;
    let a1 = (lhs >> 16) as u64;
    let b0 = (rhs & 0xffff) as u64;
    let b1 = (rhs >> 16) as u64;

    let p00 = a0 * b0;
    let p01 = a0 * b1;
    let p10 = a1 * b0;
    let p11 = a1 * b1;

    p00 + ((p01 + p10) << 16) + (p11 << 32)
}

pub fn mul_low(lhs: u32, rhs: u32) -> u32 {
    mul_u32_wide(lhs, rhs) as u32
}

pub fn mulhu(lhs: u32, rhs: u32) -> u32 {
    (mul_u32_wide(lhs, rhs) >> 32) as u32
}

pub fn mulh_signed(lhs: i32, rhs: i32) -> u32 {
    let negative = (lhs < 0) ^ (rhs < 0);
    let wide = mul_u32_wide(lhs.wrapping_abs() as u32, rhs.wrapping_abs() as u32);
    let signed = if negative {
        (!wide).wrapping_add(1)
    } else {
        wide
    };

    (signed >> 32) as u32
}

pub fn mulh_signed_unsigned(lhs: i32, rhs: u32) -> u32 {
    let wide = mul_u32_wide(lhs.wrapping_abs() as u32, rhs);
    let signed = if lhs < 0 {
        (!wide).wrapping_add(1)
    } else {
        wide
    };

    (signed >> 32) as u32
}

pub fn div_signed(lhs: i32, rhs: i32) -> u32 {
    if rhs == 0 {
        u32::MAX
    } else if lhs == i32::MIN && rhs == -1 {
        lhs as u32
    } else {
        (lhs / rhs) as u32
    }
}

pub fn div_unsigned(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        u32::MAX
    } else {
        lhs / rhs
    }
}

pub fn rem_signed(lhs: i32, rhs: i32) -> u32 {
    if rhs == 0 {
        lhs as u32
    } else if lhs == i32::MIN && rhs == -1 {
        0
    } else {
        (lhs % rhs) as u32
    }
}

pub fn rem_unsigned(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        lhs
    } else {
        lhs % rhs
    }
}
