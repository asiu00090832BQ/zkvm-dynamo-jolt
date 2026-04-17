use crate::error::ZkvmError;
use crate::isa::m::Rv32M;
use crate::isa::RTypeFields;

/// Lemma 6.1.1 decomposes a 32-bit word as:
/// x = x₀ + 2¹⁶·x₁, where x₀ = x mod 2¹⁶ and x₁ = x >> 16.
pub const fn split16(value: u32) -> (u32, u32) {
    (value & 0xffff, value >> 16)
}

pub(crate) fn decode_rv32m(fields: RTypeFields, opcode: u8, word: u32) -> Result<Rv32M, ZkvmError> {
    let inst = match fields.funct3 {
        0b000 => Rv32M::Mul(fields),
        0b001 => Rv32M::Mulh(fields),
        0b010 => Rv32M::Mulhsu(fields),
        0b011 => Rv32M::Mulhu(fields),
        0b100 => Rv32M::Div(fields),
        0b101 => Rv32M::Divu(fields),
        0b110 => Rv32M::Rem(fields),
        0b111 => Rv32M::Remu(fields),
        value => {
            return Err(ZkvmError::InvalidFunct3 {
                opcode,
                funct3: value,
                word,
            })
        }
    };

    Ok(inst)
}

pub fn execute_rv32m(inst: &Rv32M, lhs: u32, rhs: u32) -> u32 {
    match inst {
        Rv32M::Mul(_) => mul_u64_split16(lhs, rhs) as u32,
        Rv32M::Mulh(_) => mulh(lhs, rhs),
        Rv32M::Mulhsu(_) => mulhsu(lhs, rhs),
        Rv32M::Mulhu(_) => mulhu(lhs, rhs),
        Rv32M::Div(_) => div(lhs, rhs),
        Rv32M::Divu(_) => divu(lhs, rhs),
        Rv32M::Rem(_) => rem(lhs, rhs),
        Rv32M::Remu(_) => remu(lhs, rhs),
    }
}

/// Lemma 6.1.1:
/// (a₀ + 2¹⁶·a₁)(b₀ + 2¹⁶·b₁)
/// = a₀b₀ + 2¹⁶(a₀b₁ + a₁b₀) + 2³²a₁b₁.
pub const fn mul_u64_split16(lhs: u32, rhs: u32) -> u64 {
    let (a0, a1) = split16(lhs);
    let (b0, b1) = split16(rhs);

    let p0 = (a0 as u64) * (b0 as u64);
    let p1 = (a0 as u64) * (b1 as u64);
    let p2 = (a1 as u64) * (b0 as u64);
    let p3 = (a1 as u64) * (b1 as u64);

    p0.wrapping_add((p1.wrapping_add(p2)) << 16).wrapping_add(p3 << 32)
}

pub fn mulhu(lhs: u32, rhs: u32) -> u32 {
    (mul_u64_split16(lhs, rhs) >> 32) as u32
}

pub fn mulh(lhs: u32, rhs: u32) -> u32 {
    let hi = mulhu(lhs, rhs);
    hi.wrapping_sub(if (lhs & 0x8000_0000) != 0 { rhs } else { 0 })
        .wrapping_sub(if (rhs & 0x8000_0000) != 0 { lhs } else { 0 })
}

pub fn mulhsu(lhs: u32, rhs: u32) -> u32 {
    let hi = mulhu(lhs, rhs);
    hi.wrapping_sub(if (lhs & 0x8000_0000) != 0 { rhs } else { 0 })
}

pub fn div(lhs: u32, rhs: u32) -> u32 {
    let lhs_signed = lhs as i32;
    let rhs_signed = rhs as i32;

    if rhs_signed == 0 {
        u32::MAX
    } else if lhs_signed == i32::MIN && rhs_signed == -1 {
        lhs_signed as u32
    } else {
        (lhs_signed / rhs_signed) as u32
    }
}

pub fn divu(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        u32::MAX
    } else {
        lhs / rhs
    }
}

pub fn rem(lhs: u32, rhs: u32) -> u32 {
    let lhs_signed = lhs as i32;
    let rhs_signed = rhs as i32;

    if rhs_signed == 0 {
        lhs
    } else if lhs_signed == i32::MIN && rhs_signed == -1 {
        0
    } else {
        (lhs_signed % rhs_signed) as u32
    }
}

pub fn remu(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        lhs
    } else {
        lhs % rhs
    }
}
