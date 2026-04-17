#[inline]
pub const fn opcode(raw: u32) -> u8 {
    (raw & 0x7f) as u8
}

#[inline]
pub const fn rd(raw: u32) -> u8 {
    ((raw >> 7) & 0x1f) as u8
}

#[inline]
pub const fn funct3(raw: u32) -> u8 {
    ((raw >> 12) & 0x07) as u8
}

#[inline]
pub const fn rs1(raw: u32) -> u8 {
    ((raw >> 15) & 0x1f) as u8
}

#[inline]
pub const fn rs2(raw: u32) -> u8 {
    ((raw >> 20) & 0x1f) as u8
}

#[inline]
pub const fn funct7(raw: u32) -> u8 {
    ((raw >> 25) & 0x7f) as u8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RType {
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
}

impl RType {
    #[inline]
    pub fn from_raw(raw: u32) -> Self {
        Self {
            rd: rd(raw),
            funct3: funct3(raw),
            rs1: rs1(raw),
            rs2: rs2(raw),
            funct7: funct7(raw),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IType {
    pub raw: u32,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub imm: i32,
}

impl IType {
    #[inline]
    pub fn from_raw(raw: u32) -> Self {
        Self {
            raw,
            rd: rd(raw),
            funct3: funct3(raw),
            rs1: rs1(raw),
            imm: (raw as i32) >> 20,
        }
    }

    #[inline]
    pub fn shamt(&self) -> u8 {
        ((self.raw >> 20) & 0x1f) as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SType {
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

impl SType {
    #[inline]
    pub fn from_raw(raw: u32) -> Self {
        let imm = (((raw >> 25) & 0x7f) << 5) | ((raw >> 7) & 0x1f);
        let imm = (imm << 20) >> 20;
        Self {
            funct3: funct3(raw),
            rs1: rs1(raw),
            rs2: rs2(raw),
            imm,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BType {
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

impl BType {
    #[inline]
    pub fn from_raw(raw: u32) -> Self {
        let imm = (((raw >> 31) & 0x1) << 12)
            | (((raw >> 7) & 0x1) << 11)
            | (((raw >> 25) & 0x3f) << 5)
            | (((raw >> 8) & 0x0f) << 1);
        let imm = (imm << 19) >> 19;

        Self {
            funct3: funct3(raw),
            rs1: rs1(raw),
            rs2: rs2(raw),
            imm,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UType {
    pub rd: u8,
    pub imm: u32,
}

impl UType {
    #[inline]
    pub fn from_raw(raw: u32) -> Self {
        Self {
            rd: rd(raw),
            imm: raw & 0xffff_f000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JType {
    pub rd: u8,
    pub imm: i32,
}

impl JType {
    #[inline]
    pub fn from_raw(raw: u32) -> Self {
        let imm = (((raw >> 31) & 0x1) << 20)
            | (((raw >> 12) & 0xff) << 12)
            | (((raw >> 20) & 0x1) << 11)
            | (((raw >> 21) & 0x03ff) << 1);
        let imm = (imm << 11) >> 11;

        Self {
            rd: rd(raw),
            imm,
        }
    }
}
