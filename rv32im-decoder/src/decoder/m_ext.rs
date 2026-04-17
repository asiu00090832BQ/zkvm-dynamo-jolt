use crate::{
    decoder::{
        fields::{funct3, funct7, opcode, rd, rs1, rs2},
        invariants,
    },
    error::ZkvmError,
    types::{Instruction, Op},
};

const OP_OPCODE: u8 = 0b0110011;
const M_FUNCT7: u8 = 0b0000001;

/// Lemma 6.1.1:
/// Under the RV32IM OP encoding class, the predicate
/// `(opcode == 0b0110011) && (funct7 == 0b0000001)` partitions the M-extension
/// decode space into exactly eight disjoint cases indexed by `funct3`.
///
/// This implementation is the proof object:
/// - totality: the lookup table covers every 3-bit `funct3` value,
/// - disjointness: each slot maps to one and only one `Op`.
pub fn lemma_6_1_1(word: u32) -> Result<Op, ZkvmError> {
    if opcode(word) != OP_OPCODE {
        return Err(ZkvmError::InvalidOpcode {
            opcode: opcode(word),
            word,
        });
    }

    if funct7(word) != M_FUNCT7 {
        return Err(ZkvmError::InvalidFunct7 {
            funct7: funct7(word),
            opcode: OP_OPCODE,
            word,
        });
    }

    const PARTITION: [Op; 8] = [
        Op::Mul,
        Op::Mulh,
        Op::Mulhsu,
        Op::Mulhu,
        Op::Div,
        Op::Divu,
        Op::Rem,
        Op::Remu,
    ];

    PARTITION
        .get(funct3(word) as usize)
        .copied()
        .ok_or(ZkvmError::DecodeInvariantViolation {
            word,
            message: "funct3 escaped 3-bit partition in Lemma 6.1.1",
        })
}

pub fn is_m_extension(word: u32) -> bool {
    opcode(word) == OP_OPCODE && funct7(word) == M_FUNCT7
}

pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    let kind = lemma_6_1_1(word)?;
    let rd = invariants::register(rd(word), "m-extension rd")?;
    let rs1 = invariants::register(rs1(word), "m-extension rs1")?;
    let rs2 = invariants::register(rs2(word), "m-extension rs2")?;

    Ok(Instruction::Op { kind, rd, rs1, rs2 })
}

#[inline]
pub const fn decompose_u32_limb16(value: u32) -> [u16; 2] {
    [value as u16, (value >> 16) as u16]
}

#[inline]
pub const fn compose_u32_limb16(limbs: [u16; 2]) -> u32 {
    (limbs[0] as u32) | ((limbs[1] as u32) << 16)
}

#[inline]
pub fn limb16_mul_u64(lhs: u32, rhs: u32) -> u64 {
    let [a0, a1] = decompose_u32_limb16(lhs);
    let [b0, b1] = decompose_u32_limb16(rhs);

    let p00 = (a0 as u64) * (b0 as u64);
    let p01 = (a0 as u64) * (b1 as u64);
    let p10 = (a1 as u64) * (b0 as u64);
    let p11 = (a1 as u64) * (b1 as u64);

    p00 + ((p01 + p10) << 16) + (p11 << 32)
}

#[inline]
pub fn mulhu_u32_limb16(lhs: u32, rhs: u32) -> u32 {
    (limb16_mul_u64(lhs, rhs) >> 32) as u32
}

#[inline]
pub fn mulh_i32_limb16(lhs: i32, rhs: i32) -> u32 {
    let lhs_u = lhs as u32;
    let rhs_u = rhs as u32;
    let unsigned_hi = mulhu_u32_limb16(lhs_u, rhs_u);

    let lhs_sign = ((lhs_u >> 31) & 1) as u64;
    let rhs_sign = ((rhs_u >> 31) & 1) as u64;
    let correction = lhs_sign * (rhs_u as u64) + rhs_sign * (lhs_u as u64);

    unsigned_hi.wrapping_sub(correction as u32)
}

#[inline]
pub fn mulhsu_i32_u32_limb16(lhs: i32, rhs: u32) -> u32 {
    let lhs_u = lhs as u32;
    let unsigned_hi = mulhu_u32_limb16(lhs_u, rhs);
    let lhs_sign = ((lhs_u >> 31) & 1) as u64;
    let correction = lhs_sign * (rhs as u64);

    unsigned_hi.wrapping_sub(correction as u32)
}
