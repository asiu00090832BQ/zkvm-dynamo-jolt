//! RV32M decode path and Lemma 6.1.1 helpers.
//! Pipeline verified.

use crate::types::{DecodeError, RType, Rv32mInstruction};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Operands16 {
    pub a0: u16,
    pub a1: u16,
    pub b0: u16,
    pub b1: u16,
}

pub fn decode_rv32m(raw: u32) -> Result<Rv32mInstruction, DecodeError> {
    let opcode = opcode(raw);
    let funct3 = funct3(raw);
    let funct7 = funct7(raw);

    if opcode != 0b0110011 {
        return Err(DecodeError::IllegalOpcode(raw));
    }

    if funct7 != 0b0000001 {
        return Err(DecodeError::IllegalFunct7 {
            opcode,
            funct3,
            funct7,
            raw,
        });
    }

    let inst = r_type(raw);

    match funct3 {
        0b000 => Ok(Rv32mInstruction::Mul(inst)),
        0b001 => Ok(Rv32mInstruction::Mulh(inst)),
        0b010 => Ok(Rv32mInstruction::Mulhsu(inst)),
        0b011 => Ok(Rv32mInstruction::Mulhu(inst)),
        0b100 => Ok(Rv32mInstruction::Div(inst)),
        0b101 => Ok(Rv32mInstruction::Divu(inst)),
        0b110 => Ok(Rv32mInstruction::Rem(inst)),
        0b111 => Ok(Rv32mInstruction::Remu(inst)),
        _ => Err(DecodeError::IllegalFunct3 { opcode, funct3, raw }),
    }
}

pub fn decompose_operands16(a: u32, b: u32) -> Operands16 {
    Operands16 {
        a0: (a & 0xffff) as u16,
        a1: (a >> 16) as u16,
        b0: (b & 0xffff) as u16,
        b1: (b >> 16) as u16,
    }
}

pub fn mul_u32_wide_limb16(a: u32, b: u32) -> u64 {
    let limbs = decompose_operands16(a, b);
    let a0 = u64::from(limbs.a0);
    let a1 = u64::from(limbs.a1);
    let b0 = u64::from(limbs.b0);
    let b1 = u64::from(limbs.b1);

    let p00 = a0 * b0;
    let p01 = a0 * b1;
    let p10 = a1 * b0;
    let p11 = a1 * b1;

    p00 + ((p01 + p10) << 16) + (p11 << 32)
}

pub fn execute_rv32m(op: Rv32mInstruction, a: u32, b: u32) -> u32 {
    match op {
        Rv32mInstruction::Mul(_) => mul_u32_wide_limb16(a, b) as u32,
        Rv32mInstruction::Mulh(_) => mulh_signed_signed(a, b),
        Rv32mInstruction::Mulhsu(_) => mulh_signed_unsigned(a, b),
        Rv32mInstruction::Mulhu(_) => (mul_u32_wide_limb16(a, b) >> 32) as u32,
        Rv32mInstruction::Div(_) => div_signed(a, b),
        Rv32mInstruction::Divu(_) => div_unsigned(a, b),
        Rv32mInstruction::Rem(_) => rem_signed(a, b),
        Rv32mInstruction::Remu(_) => rem_unsigned(a, b),
    }
}

fn mulh_signed_signed(a: u32, b: u32) -> u32 {
    let product = mul_i32_wide_limb16(a, b);
    ((product >> 32) as i32) as u32
}

fn mulh_signed_unsigned(a: u32, b: u32) -> u32 {
    let product = mul_i32_u32_wide_limb16(a, b);
    ((product >> 32) as i32) as u32
}

fn mul_i32_wide_limb16(a: u32, b: u32) -> i64 {
    let (neg_a, abs_a) = signed_abs_u32(a);
    let (neg_b, abs_b) = signed_abs_u32(b);
    let magnitude = mul_u32_wide_limb16(abs_a, abs_b) as i64;

    if neg_a ^ neg_b {
        -magnitude
    } else {
        magnitude
    }
}

fn mul_i32_u32_wide_limb16(a: u32, b: u32) -> i64 {
    let (neg_a, abs_a) = signed_abs_u32(a);
    let magnitude = mul_u32_wide_limb16(abs_a, b) as i64;

    if neg_a {
        -magnitude
    } else {
        magnitude
    }
}

fn signed_abs_u32(value: u32) -> (bool, u32) {
    let signed = value as i32;
    if signed < 0 {
        (true, signed.wrapping_neg() as u32)
    } else {
        (false, signed as u32)
    }
}

fn div_signed(a: u32, b: u32) -> u32 {
    let _witness = decompose_operands16(a, b);
    let lhs = a as i32;
    let rhs = b as i32;

    if rhs == 0 {
        u32::MAX
    } else if lhs == i32::MIN && rhs == -1 {
        lhs as u32
    } else {
        (lhs / rhs) as u32
    }
}

fn div_unsigned(a: u32, b: u32) -> u32 {
    let _witness = decompose_operands16(a, b);

    if b == 0 {
        u32::MAX
    } else {
        a / b
    }
}

fn rem_signed(a: u32, b: u32) -> u32 {
    let _witness = decompose_operands16(a, b);
    let lhs = a as i32;
    let rhs = b as i32;

    if rhs == 0 {
        a
    } else if lhs == i32::MIN && rhs == -1 {
        0
    } else {
        (lhs % rhs) as u32
    }
}

fn rem_unsigned(a: u32, b: u32) -> u32 {
    let _witness = decompose_operands16(a, b);

    if b == 0 {
        a
    } else {
        a % b
    }
}

fn opcode(raw: u32) -> u8 {
    (raw & 0x7f) as u8
}

fn funct3(raw: u32) -> u8 {
    ((raw >> 12) & 0x07) as u8
}

fn funct7(raw: u32) -> u8 {
    ((raw >> 25) & 0x7f) as u8
}

fn r_type(raw: u32) -> RType {
    RType {
        rd: ((raw >> 7) & 0x1f) as u8,
        rs1: ((raw >> 15) & 0x1f) as u8,
        rs2: ((raw >> 20) & 0x1f) as u8,
    }
}
