use crate::{
    bits,
    decode::Register,
    error::{Result, ZkvmError},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MOperands {
    pub rd: Register,
    pub rs1: Register,
    pub rs2: Register,
}

impl MOperands {
    #[inline]
    pub fn decode(word: u32) -> Result<Self> {
        Ok(Self {
            rd: Register::new(bits::rd(word))?,
            rs1: Register::new(bits::rs1(word))?,
            rs2: Register::new(bits::rs2(word))?,
        })
    }

    #[inline]
    pub const fn unsigned_limb_decomposition(lhs: u32, rhs: u32) -> MDecomposition16 {
        MDecomposition16::from_u32(lhs, rhs)
    }

    #[inline]
    pub const fn signed_limb_decomposition(lhs: i32, rhs: i32) -> SignedMDecomposition16 {
        SignedMDecomposition16::from_i32(lhs, rhs)
    }
}

/// 32-bit value decomposed into two 16-bit limbs:
/// value = low + 2^16 * high.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct U32Limbs16 {
    pub low: u16,
    pub high: u16,
}

impl U32Limbs16 {
    #[inline]
    pub const fn from_u32(value: u32) -> Self {
        Self {
            low: value as u16,
            high: (value >> 16) as u16,
        }
    }

    #[inline]
    pub const fn recompose(self) -> u32 {
        (self.low as u32) | ((self.high as u32) << 16)
    }
}

/// Signed 32-bit value viewed through the same 16-bit decomposition.
/// This preserves the Lemma 6.1.1 limb layout while exposing a signed high limb.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct I32Limbs16 {
    pub low: u16,
    pub high: i16,
}

impl I32Limbs16 {
    #[inline]
    pub const fn from_i32(value: i32) -> Self {
        let raw = value as u32;
        Self {
            low: raw as u16,
            high: (raw >> 16) as i16,
        }
    }

    #[inline]
    pub const fn recompose(self) -> i32 {
        ((self.high as i32) << 16) | (self.low as i32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MDecomposition16 {
    pub lhs: U32Limbs16,
    pub rhs: U32Limbs16,
}

impl MDecomposition16 {
    #[inline]
    pub const fn from_u32(lhs: u32, rhs: u32) -> Self {
        Self {
            lhs: U32Limbs16::from_u32(lhs),
            rhs: U32Limbs16::from_u32(rhs),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SignedMDecomposition16 {
    pub lhs: I32Limbs16,
    pub rhs: I32Limbs16,
}

impl SignedMDecomposition16 {
    #[inline]
    pub const fn from_i32(lhs: i32, rhs: i32) -> Self {
        Self {
            lhs: I32Limbs16::from_i32(lhs),
            rhs: I32Limbs16::from_i32(rhs),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rv32mInstruction {
    Mul(MOperands),
    Mulh(MOperands),
    Mulhsu(MOperands),
    Mulhu(MOperands),
    Div(MOperands),
    Divu(MOperands),
    Rem(MOperands),
    Remu(MOperands),
}

#[inline]
pub fn decode_rv32m(word: u32) -> Result<Rv32mInstruction> {
    let opcode = bits::opcode(word);
    let funct3 = bits::funct3(word);
    let funct7 = bits::funct7(word);

    if opcode != 0b0110011 {
        return Err(ZkvmError::UnknownOpcode { opcode, word });
    }

    if funct7 != 0b0000001 {
        return Err(ZkvmError::UnknownFunct7 {
            opcode,
            funct3,
            funct7,
            word,
        });
    }

    let operands = MOperands::decode(word)?;

    match funct3 {
        0b000 => Ok(Rv32mInstruction::Mul(operands)),
        0b001 => Ok(Rv32mInstruction::Mulh(operands)),
        0b010 => Ok(Rv32mInstruction::Mulhsu(operands)),
        0b011 => Ok(Rv32mInstruction::Mulhu(operands)),
        0b100 => Ok(Rv32mInstruction::Div(operands)),
        0b101 => Ok(Rv32mInstruction::Divu(operands)),
        0b110 => Ok(Rv32mInstruction::Rem(operands)),
        0b111 => Ok(Rv32mInstruction::Remu(operands)),
        funct3 => Err(ZkvmError::UnknownFunct3 { opcode, funct3, word }),
    }
}
