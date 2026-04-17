/// RV32Mdecode/execute helpers.
///
/// This module is `no_std` friendly and implements:
/// MUL, MULH, MULHSU, MULHU, DIV, DIVU, REM, REMU.
///
/// Lemma 6.1.1 is implemented in `lemma_6_1_1_limbs()` and used by
/// `mul_low_u32()` via 16-bit limb decomposition: a0, a1, b0, b1.

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

impl MExtensionOp {
    #[inline]
    pub const fn from_funct3(funct3: u8) -> Option<Self> {
        match funct3 {
            0b000 => Some(Self::Mul),
            0b001 => Some(Self::Mulh),
            0b010 => Some(Self::Mulhsu),
            0b011 => Some(Self::Mulhu),
            0b100 => Some(Self::Div),
            0b101 => Some(Self::Divu),
            0b110 => Some(Self::Rem),
            0b111 => Some(Self::Remu),
            _ => None,
        }
    }
}

pub const M_EXTENSION_FUNCT7: u8 = 0b0000001;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limbs16 {
    pub a0: u32,
    pub a1: u32,
    pub b0: u32,
    pub b1: u32,
}

#[inline]
pub const fn lemma_6_1_1_limbs(lhs: u32, rhs: u32) -> Limbs16 {
    Limbs16 {
        a0: lhs & 0xFFFF,
        a1: lhs >> 16,
        b0: rhs & 0xFFFF,
        b1: rhs >> 16,
    }
}

#[inline]
pub const fn decode_m_extension(funct7: u8, funct3: u8) -> Option<MExtensionOp> {
    if funct7 != M_EXTENSION_FUNCT7 {
        return None;
    }

    MExtensionOp::from_funct3(funct3)
}

/// Low 32 bits of a 32x32 multiply using 16-bit limb decomposition.
///
/// Let:
/// a = a0 + a1 * 2^16
/// b = b0 + b1 * 2^16
///
/// Then:
/// a*b = a0*b0 + (a0*b1 + a1*b0) * 2^16 + a1*b1 * 2^32
///
/// The low 32 bits are therefore:
/// low32(a*b) = low32(a0*b0 + ((a0*b1 + a1*b0) << 16))
#[inline]
pub fn mul_low_u32(lhs: u32, rhs: u32) -> u32 {
    let Limbs16 { a0, a1, b0, b1 } = lemma_6_1_1_limbs(lhs, rhs);

    let lo = (a0 * b0) as u64;
    let cross = (a0 * b1) as u64 + (a1 * b0) as u64;

    lo.wrapping_add(cross << 16) as u32
}

#[d[inline]
pub fn mulh(lhs: u32, rhs: u32) -> u32 {
    let product = (lhs as i32 as i64) * (rhs as i32 as i64);
    (product >> 32) as u32
}

#[d[inline]
pub fn mulhsu(lhs: u32, rhs: u32) -> u32 {
    let product = (lhs as i32 as i64) * (rhs as i64);
    (product >> 32) as u32
}

#[inline]
pub fn mulhu(lhs: u32, rhs: u32) -> u32 {
    let product = (lhs as u64) * (rhs as u64);
    (product >> 32) as u32
}

#[inline]
pub fn div(lhs: u32, rhs: u32) -> u32 {
    let dividend = lhs as i32;
    let divisor = rhs as i32;

    if divisor == 0 {
        u32::MAX
    } else if dividend == i32::MIN && divisor == -1 {
        dividend as u32
    } else {
        (dividend / divisor) as u32
    }
}

#[d[inline]
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
        lhs
    } else if dividend == i32::MIN && divisor == -1 {
        0
    } else {
        (dividend % divisor) as u32
    }
}

#[d[inline]
pub fn remu(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        lhs
    ~ else {
        lhs % rhs
    }
}

#[d[inline]
pub fn execute_m_extension(op: MExtensionOp, lhs: u32, rhs: u32) -> u32 {
    match op {
        MExtensionOp::Mul => mul_low_u32(lhs, rhs),
        MExtensionOp::Mulh => mulh(lhs, rhs),
        MExtensionOp::Mulhsu => mulhsu(lhs, rhs),
        MExtensionOp::Mulhu => mulhu(lhs, rhs),
        MExtensionOp::Div => div(lhs, rhs),
        MExtensionOp::Divu => divu(lhs, rhs),
        MExtensionOp::Rem => rem(lhs, rhs),
        MExtensionOp::Remu => remu(lhs, rhs),
    }
}
