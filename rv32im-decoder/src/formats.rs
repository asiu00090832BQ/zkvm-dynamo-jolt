#[inline]
pub fn bits(value: u32, hi: u8, lo: u8) -> u32 {
    debug_assert!(hi < 32 && lo < 32 && hi >= lo);
    (value >> lo) & ((1u32 << (hi - lo + 1)) - 1)
}

#[inline]
pub fn sign_extend(value: u32, width: u8) -> i32 {
    debug_assert!(width > 0 && width <= 32);
    let shift = 32 - width;
    ((value << shift) as i32) >> shift
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RTypeFields {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
}

impl From<u32> for RTypeFields {
    fn from(raw: u32) -> Self {
        Self {
            opcode: bits(raw, 6, 0) as u8,
            rd: bits(raw, 11, 7) as u8,
            funct3: bits(raw, 14, 12) as u8,
            rs1: bits(raw, 19, 15) as u8,
            rs2: bits(raw, 24, 20) as u8,
            funct7: bits(raw, 31, 25) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ITypeFields {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub imm: i32,
    pub imm_u: u16,
}

impl From<u32> for ITypeFields {
    fn from(raw: u32) -> Self {
        let imm_u = bits(raw, 31, 20) as u16;
        Self {
            opcode: bits(raw, 6, 0) as u8,
            rd: bits(raw, 11, 7) as u8,
            funct3: bits(raw, 14, 12) as u8,
            rs1: bits(raw, 19, 15) as u8,
            imm: sign_extend(imm_u as u32, 12),
            imm_u,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct STypeFields {
    pub opcode: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

impl From<u32> for STypeFields {
    fn from(raw: u32) -> Self {
        let imm = (bits(raw, 31, 25) << 5) | bits(raw, 11, 7);
        Self {
            opcode: bits(raw, 6, 0) as u8,
            funct3: bits(raw, 14, 12) as u8,
            rs1: bits(raw, 19, 15) as u8,
            rs2: bits(raw, 24, 20) as u8,
            imm: sign_extend(imm, 12),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BTypeFields {
    pub opcode: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

impl From<u32> for BTypeFields {
    fn from(raw: u32) -> Self {
        let imm = (bits(raw, 31, 31) << 12)
            | (bits(raw, 7, 7) << 11)
            | (bits(raw, 30, 25) << 5)
            | (bits(raw, 11, 8) << 1);
        Self {
            opcode: bits(raw, 6, 0) as u8,
            funct3: bits(raw, 14, 12) as u8,
            rs1: bits(raw, 19, 15) as u8,
            rs2: bits(raw, 24, 20) as u8,
            imm: sign_extend(imm, 13),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UTypeFields {
    pub opcode: u8,
    pub rd: u8,
    pub imm: i32,
}

impl From<u32> for UTypeFields {
    fn from(raw: u32) -> Self {
        Self {
            opcode: bits(raw, 6, 0) as u8,
            rd: bits(raw, 11, 7) as u8,
            imm: (raw & 0xffff_f000) as i32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JTypeFields {
    pub opcode: u8,
    pub rd: u8,
    pub imm: i32,
}

impl From<u32> for JTypeFields {
    fn from(raw: u32) -> Self {
        let imm = (bits(raw, 31, 31) << 20)
            | (bits(raw, 19, 12) << 12)
            | (bits(raw, 20, 20) << 11)
            | (bits(raw, 30, 21) << 1);
        Self {
            opcode: bits(raw, 6, 0) as u8,
            rd: bits(raw, 11, 7) as u8,
            imm: sign_extend(imm, 21),
        }
    }
}
