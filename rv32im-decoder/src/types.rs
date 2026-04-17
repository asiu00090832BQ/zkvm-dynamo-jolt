pub type Word = u32;
pub type SignedWord = i32;
pub type Register = u8;

pub const REGISTER_COUNT: usize = 32;
pub const ZERO_REGISTER: Register = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedFields {
    raw: Word,
    pub opcode: u8,
    pub rd: Register,
    pub funct3: u8,
    pub rs1: Register,
    pub rs2: Register,
    pub funct7: u8,
    pub shamt: u8,
    pub imm_i: i32,
    pub imm_s: i32,
    pub imm_b: i32,
    pub imm_u: Word,
    pub imm_j: i32,
}

impl DecodedFields {
    pub const fn new(raw: Word) -> Self {
        let opcode = bits(raw, 0, 7) as u8;
        let rd = bits(raw, 7, 5) as Register;
        let funct3 = bits(raw, 12, 3) as u8;
        let rs1 = bits(raw, 15, 5) as Register;
        let rs2 = bits(raw, 20, 5) as Register;
        let funct7 = bits(raw, 25, 7) as u8;
        let shamt = bits(raw, 20, 5) as u8;

        let imm_i = sign_extend(bits(raw, 20, 12), 12);
        let imm_s = sign_extend((bits(raw, 25, 7) << 5) | bits(raw, 7, 5), 12);
        let imm_b = sign_extend(
            (bits(raw, 31, 1) << 12)
                | (bits(raw, 7, 1) << 11)
                | (bits(raw, 25, 6) << 5)
                | (bits(raw, 8, 4) << 1),
            13,
        );
        let imm_u = raw & 0xffff_f000;
        let imm_j = sign_extend(
            (bits(raw, 31, 1) << 20)
                | (bits(raw, 12, 8) << 12)
                | (bits(raw, 20, 1) << 11)
                | (bits(raw, 21, 10) << 1),
            21,
        );

        Self {
            raw,
            opcode,
            rd,
            funct3,
            rs1,
            rs2,
            funct7,
            shamt,
            imm_i,
            imm_s,
            imm_b,
            imm_u,
            imm_j,
        }
    }

    pub const fn raw(self) -> Word {
        self.raw
    }
}

impl From<Word> for DecodedFields {
    fn from(raw: Word) -> Self {
        Self::new(raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// 16-bit limb decomposition used by RV32M multiplication paths.
pub struct LimbDecomposition16 {
    pub lo: u16,
    pub hi: u16,
}

impl LimbDecomposition16 {
    pub const fn from_u32(value: u32) -> Self {
        Self {
            lo: (value & 0xffff) as u16,
            hi: (value >> 16) as u16,
        }
    }

    pub const fn to_u32(self) -> u32 {
        (self.lo as u32) | ((self.hi as u32) << 16)
    }
}

pub const fn bits(value: u32, start: u8, len: u8) -> u32 {
    let mask = if len >= 32 {
        u32::MAX
    } else {
        (1u32 << len) - 1
    };
    (value >> start) & mask
}

pub const fn sign_extend(value: u32, width: u8) -> i32 {
    if width == 0 {
        0
    } else {
        let shift = 32 - width;
        ((value << shift) as i32) >> shift
    }
}

/// Multiplies two 32-bit values via 16-bit limbs (Lemma 6.1.1 support).
pub const fn mul_u32_via_limbs(lhs: u32, rhs: u32) -> u64 {
    let lhs_limbs = LimbDecomposition16::from_u32(lhs);
    let rhs_limbs = LimbDecomposition16::from_u32(rhs);

    let a0 = lhs_limbs.lo as u64;
    let a1 = lhs_limbs.hi as u64;
    let b0 = rhs_limbs.lo as u64;
    let b1 = rhs_limbs.hi as u64;

    (a0 * b0) + ((a0 * b1 + a1 * b0) << 16) + ((a1 * b1) << 32)
}
