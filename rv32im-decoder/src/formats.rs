#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RType {
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IType {
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub imm: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SType {
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BType {
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UType {
    pub rd: u8,
    pub imm: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JType {
    pub rd: u8,
    pub imm: i32,
}

#[inline(always)]
pub const fn opcode(word: u32) -> u8 {
    (word & 0x7f) as u8
}

#[inline(always)]
pub const fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1f) as u8
}

#[inline(always)]
pub const fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x07) as u8
}

#[inline(always)]
pub const fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}

#[inline(always)]
pub const fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

#[inline(always)]
pub const fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7f) as u8
}

#[inline(always)]
pub const fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits as u32;
    ((value << shift) as i32) >> shift
}

impl RType {
    #[inline(always)]
    pub const fn decode(word: u32) -> Self {
        Self {
            rd: rd(word),
            funct3: funct3(word),
            rs1: rs1(word),
            rs2: rs2(word),
            funct7: funct7(word),
        }
    }
}

impl IType {
    #[inline(always)]
    pub const fn decode(word: u32) -> Self {
        Self {
            rd: rd(word),
            funct3: funct3(word),
            rs1: rs1(word),
            imm: sign_extend(word >> 20, 12),
        }
    }
}

impl SType {
    #[inline(always)]
    pub const fn decode(word: u32) -> Self {
        let imm = ((word >> 7) & 0x1f) | (((word >> 25) & 0x7f) << 5);
        Self {
            funct3: funct3(word),
            rs1: rs1(word),
            rs2: rs2(word),
            imm: sign_extend(imm, 12),
        }
    }
}

impl BType {
    #[inline(always)]
    pub const fn decode(word: u32) -> Self {
        let imm = (((word >> 8) & 0x0f) << 1)
            | (((word >> 25) & 0x3f) << 5)
            | (((word >> 7) & 0x01) << 11)
            | (((word >> 31) & 0x01) << 12);

        Self {
            funct3: funct3(word),
            rs1: rs1(word),
            rs2: rs2(word),
            imm: sign_extend(imm, 13),
        }
    }
}

impl UType {
    #[inline(always)]
    pub const fn decode(word: u32) -> Self {
        Self {
            rd: rd(word),
            imm: word & 0xffff_f000,
        }
    }
}

impl JType {
    #[inline(always)]
    pub const fn decode(word: u32) -> Self {
        let imm = (((word >> 21) & 0x03ff) << 1)
            | (((word >> 20) & 0x0001) << 11)
            | (((word >> 12) & 0x00ff) << 12)
            | (((word >> 31) & 0x0001) << 20);

        Self {
            rd: rd(word),
            imm: sign_extend(imm, 21),
        }
    }
}
