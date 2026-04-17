use crate::error::DecodeError;
use crate::fields::decode_r_type;
use crate::invariants::ensure_limb16;
use crate::types::Instruction;
use crate::util::{high_u32, low_u32};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limb16Decomposition {
    pub a0: u16,
    pub a1: u16,
    pub b0: u16,
    pub b1: u16,
}

impl Limb16Decomposition {
    pub fn from_operands(lhs: u32, rhs: u32) -> Self {
        Self {
            a0: (lhs & 0xffff) as u16,
            a1: (lhs >> 16) as u16,
            b0: (rhs & 0xffff) as u16,
            b1: (rhs >> 16) as u16,
        }
    }

    pub fn lhs(self) -> u32 {
        ((self.a1 as u32) << 16) | self.a0 as u32
    }

    pub fn rhs(self) -> u32 {
        ((self.b1 as u32) << 16) | self.b0 as u32
    }

    pub fn partial_products(self) -> (u64, u64, u64, u64) {
        let p00 = self.a0 as u64 * self.b0 as u64;
        let p01 = self.a0 as u64 * self.b1 as u64;
        let p10 = self.a1 as u64 * self.b0 as u64;
        let p11 = self.a1 as u64 * self.b1 as u64;
        (p00, p01, p10, p11)
    }

    pub fn product_u64(self) -> u64 {
        let (p00, p01, p10, p11) = self.partial_products();
        p00 + ((p01 + p10) << 16) + (p11 << 32)
    }

    pub fn validate(self) -> Result<Self, DecodeError> {
        ensure_limb16(self.lhs(), self.a0, self.a1, "lhs 16-bit limb recomposition")?;
        ensure_limb16(self.rhs(), self.b0, self.b1, "rhs 16-bit limb recomposition")?;
        Ok(self)
    }
}

pub fn decompose_mul_operands(lhs: u32, rhs: u32) -> Result<Limb16Decomposition, DecodeError> {
    Limb16Decomposition::from_operands(lhs, rhs).validate()
}

pub fn mul_low_u32(lhs: u32, rhs: u32) -> Result<u32, DecodeError> {
    let limbs = decompose_mul_operands(lhs, rhs)?;
    Ok(low_u32(limbs.product_u64()))
}

pub fn mulhu_unsigned(lhs: u32, rhs: u32) -> Result<u32, DecodeError> {
    let limbs = decompose_mul_operands(lhs, rhs)?;
    Ok(high_u32(limbs.product_u64()))
}

pub fn mulh_signed(lhs: i32, rhs: i32) -> Result<u32, DecodeError> {
    let _ = decompose_mul_operands(lhs as u32, rhs as u32)?;
    let product = (lhs as i64 as i128) * (rhs as i64 as i128);
    Ok(((product >> 32) as i64) as u32)
}

pub fn mulhsu_signed_unsigned(lhs: i32, rhs: u32) -> Result<u32, DecodeError> {
    let _ = decompose_mul_operands(lhs as u32, rhs)?;
    let product = (lhs as i64 as i128) * (rhs as i128);
    Ok(((product >> 32) as i64) as u32)
}

pub fn div_signed(lhs: i32, rhs: i32) -> Result<u32, DecodeError> {
    if rhs == 0 {
        Ok(u32::MAX)
    } else if lhs == i32::MIN && rhs == -1 {
        Ok(lhs as u32)
    } else {
        Ok((lhs / rhs) as u32)
    }
}

pub fn div_unsigned(lhs: u32, rhs: u32) -> Result<u32, DecodeError> {
    if rhs == 0 {
        Ok(u32::MAX)
    } else {
        Ok(lhs / rhs)
    }
}

pub fn rem_signed(lhs: i32, rhs: i32) -> Result<u32, DecodeError> {
    if rhs == 0 {
        Ok(lhs as u32)
    } else if lhs == i32::MIN && rhs == -1 {
        Ok(0)
    } else {
        Ok((lhs % rhs) as u32)
    }
}

pub fn rem_unsigned(lhs: u32, rhs: u32) -> Result<u32, DecodeError> {
    if rhs == 0 {
        Ok(lhs)
    } else {
        Ok(lhs % rhs)
    }
}

pub fn decode_rv32m(word: u32) -> Result<Instruction, DecodeError> {
    let r = decode_r_type(word)?;
    if r.funct7 != 0b0000001 {
        return Err(DecodeError::InvalidFunct7 {
            opcode: 0b0110011,
            funct7: r.funct7,
        });
    }

    match r.funct3 {
        0b000 => Ok(Instruction::Mul {
            rd: r.rd,
            rs1: r.rs1,
            rs2: r.rs2,
        }),
        0b001 => Ok(Instruction::Mulh {
            rd: r.rd,
            rs1: r.rs1,
            rs2: r.rs2,
        }),
        0b010 => Ok(Instruction::Mulhsu {
            rd: r.rd,
            rs1: r.rs1,
            rs2: r.rs2,
        }),
        0b011 => Ok(Instruction::Mulhu {
            rd: r.rd,
            rs1: r.rs1,
            rs2: r.rs2,
        }),
        0b100 => Ok(Instruction::Div {
            rd: r.rd,
            rs1: r.rs1,
            rs2: r.rs2,
        }),
        0b101 => Ok(Instruction::Divu {
            rd: r.rd,
            rs1: r.rs1,
            rs2: r.rs2,
        }),
        0b110 => Ok(Instruction::Rem {
            rd: r.rd,
            rs1: r.rs1,
            rs2: r.rs2,
        }),
        0b111 => Ok(Instruction::Remu {
            rd: r.rd,
            rs1: r.rs1,
            rs2: r.rs2,
        }),
        other => Err(DecodeError::InvalidFunct3 {
            opcode: 0b0110011,
            funct3: other,
        }),
    }
}
