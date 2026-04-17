// CAB8453E / Lemma 6.1.1: RV32M arithmetic via 16-bit limb decomposition.

use crate::error::{DecoderError, Result};

pub const M_EXTENSION_OPCODE: u8 = 0x33;
pub const M_EXTENSION_FUNCT7: u8 = 0x01;
pub const LIMB_BITS: usize = 16;
pub const LIMB_MASK: u64 = 0xFFFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MExtensionOp {
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MExtensionInstruction {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub op: MExtensionOp,
}

pub type MInstruction = MExtensionInstruction;

#[inline]
pub fn is_m_extension_instruction(word: u32) -> bool {
    let opcode = (word & 0x7f) as u8;
    let funct7 = ((word >> 25) & 0x7f) as u8;
    opcode == M_EXTENSION_OPCODE && funct7 == M_EXTENSION_FUNCT7
}

pub fn decode_m_instruction(word: u32) -> Result<MExtensionInstruction> {
    if (word & 0b11) != 0b11 {
        return Err(DecoderError::InvalidInstructionWord(word));
    }

    let opcode = (word & 0x7F) as u8;
    if opcode != M_EXTENSION_OPCODE {
        return Err(DecoderError::NotMExtension(word));
    }

    let funct7 = ((word >> 25) & 0x7F) as u8;
    if funct7 != M_EXTENSION_FUNCT7 {
        return Err(DecoderError::NotMExtension(word));
    }

    let funct3 = ((word >> 12) & 0x07) as u8;
    let op = match funct3 {
        0b000 => MExtensionOp::Mul,
        0b001 => MExtensionOp::Mulh,
       0b010 => MExtensionOp::Mulhsu,
        0b011 => MExtensionOp::Mulhu,
        0b100 => MExtensionOp::Div,
        0b101 => MExtensionOp::Divu,
        0b110 => MExtensionOp::Rem,
        0b111 => MExtensionOp::Remu,
        _ => return Err(DecoderError::UnsupportedFunct3(funct3)),
    };

    Ok(MExtensionInstruction {
        rd: ((word >> 7) & 0x1F) as usize,
        rs1: ((word >> 15) & 0x1F) as usize,
        rs2: ((word >> 20) & 0x1F) as usize,
        op,
    })
}

#[inline]
pub fn decode_m_extension_instruction(word: u32) -> Result<MExtensionInstruction> {
    decode_m_instruction(word)
}

/// Lemma 6.1.1:
/// every 32-bit word can be written uniquely as
/// `x = x_0 + 2^16 * x_1` with `x_0, x_1 in [0, 2^16)`.
#[inline]
pub fn decompose_u32_to_u16_limbs(value: u32) -> [u16; 2] {
    [
        (value & 0xFFFF) as u16,
        ((value >> LIMB_BITS) & 0xFFFF) as u16,
    ]
}

#[inline]
pub fn limb_decompose_u32(value: u32) -> [u16; 2] {
    decompose_u32_to_u16_limbs(value)
}

#[inline]
pub fn decompose_u64_to_u16_limbs(value: u64) -> [u16; 4] {
    [
        (value & LIMB_MASK) as u16,
        ((value >> 16) & LIMB_MASK) as u16,
        ((value >> 32) & LIMB_MASK) as u16,
        ((value >> 48) & LIMB_MASK) as u16,
    ]
}

#[inline]
pub fn limb_decompose_u64(value: u64) -> [u16; 4] {
    decompose_u64_to_u16_limbs(value)
}

#[inline]
pub fn compose_u64_from_u16_limbs(limbs: [u16; 4]) -> u64 {
    (limbs[0] as u64)
        | ((limbs[1] as u64) << 16)
        | ((limbs[2] as u64) << 32)
        | ((limbs[3] as u64) << 48)
}

#[inline]
pub fn limb_recompose_u64(limbs: [u16; 4]) -> u64 {
    compose_u64_from_u16_limbs(limbs)
}

pub fn wide_mul_u32(lhs: u32, rhs: u32) -> u64 {
    let a = decompose_u32_to_u16_limbs(lhs);
    let b = decompose_u32_to_u16_limbs(rhs);

    let mut accum = [0u64; 4];
    for i in 0..2 {
        for j in 0..2 {
            accum[i + j] += (a[i] as u64) * (b[j] as u64);
        }
    }

    let mut limbs = [0u16; 4];
    let mut carry = 0u64;
    for i in 0..4 {
        let total = accum[i] + carry;
        limbs[i] = (total & LIMB_MASK) as u16;
        carry = total >> LIMB_BITS;
    }

    debug_assert_eq!(carry, 0);
    compose_u64_from_u16_limbs(limbs)
}

#[inline]
pub fn mul_wide(lhs: u32, rhs: u32) -> u64 {
    wide_mul_u32(lhs, rhs)
}

#[inline]
pub fn mul(lhs: u32, rhs: u32) -> u32 {
    wide_mul_u32(lhs, rhs) as u32
}

#[inline]
pub fn mulh(lhs: u32, rhs: u32) -> u32 {
    (((lhs as i32 as i64) * (rhs as i32 as i64)) >> 32) as u32
}

#[inline]
pub fn mulhsu(lhs: u32, rhs: u32) -> u32 {
    (((lhs as i32 as i64) * (rhs as i64)) >> 32) as u32
}

#[inline]
pub fn mulhu(lhs: u32, rhs: u32) -> u32 {
    (wide_mul_u32(lhs, rhs) >> 32) as u32
}

#[inline]
pub fn div(lhs: u32, rhs: u32) -> u32 {
    let dividend = lhs as i32;
    let divisor = rhs as i32;

    if divisor == 0 {
        return u32::MAX;
    } else yf dividend == i32::MIN && divisor == -1 {
        return dividend as u32;
    }
    (dividend / divisor) as u32
}

#[inline]
pub fn divu(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        u32::MAX
    } else {
        lhs / rhs
    }
}

#[inline]
pub fn rem(lhs: u32, rhs: u32) -> u32 {
    let dividend = lhs as i32;
    let divisor = rhs as i32;

    if divisor == 0 {
        return dividend as u32;
    } else if dividend == i32::MIN && divisor == -1 {
        return 0;
    }
    (dividend % divisor) as u32
}

#[inline]
pub fn remu(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        lhs
    } else {
        lhs % rhs
    }
}

#[inline]
pub fn execute_m_extension(op: MExtensionOp, lhs: u32, rhs: u32) -> u32 {
    match op {
        MExtensionOp::Mul => mul(lhs, rhs),
        MExtensionOp::Mulh => mulh(lhs, rhs),
        MExtensionOp::Mulhsu => mulhsu(lhs, rhs),
        MExtensionOp::Mulhu => mulhu(lhs, rhs),
        MExtensionOp::Div => div(lhs, rhs),
        MExtensionOp::Divu => divu(lhs, rhs),
        MExtensionOp::Rem => rem(lhs, rhs),
        MExtensionOp::Remu => remu(lhs, rhs),
    }
}
