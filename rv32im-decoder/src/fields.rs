use crate::decoder::sign_ext::sign_extend;
use crate::error::DecodeError;
use crate::types::Register;
use crate::util::bits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RType {
    pub rd: Register,
    pub rs1: Register,
    pub rs2: Register,
    pub funct3: u8,
    pub funct7: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IType {
    pub rd: Register,
    pub rs1: Register,
    pub funct3: u8,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SType {
    pub rs1: Register,
    pub rs2: Register,
    pub funct3: u8,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BType {
    pub rs1: Register,
    pub rs2: Register,
    pub funct3: u8,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UType {
    pub rd: Register,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JType {
    pub rd: Register,
    pub imm: i32,
}

pub const fn opcode(word: u32) -> u8 {
    bits(word, 6, 0) as u8
}

pub const fn rd(word: u32) -> u8 {
    bits(word, 11, 7) as u8
}

pub const fn funct3(word: u32) -> u8 {
    bits(word, 14, 12) as u8
}

pub const fn rs1(word: u32) -> u8 {
    bits(word, 19, 15) as u8
}

pub const fn rs2(word: u32) -> u8 {
    bits(word, 24, 20) as u8
}

pub const fn funct7(word: u32) -> u8 {
    bits(word, 31, 25) as u8
}

pub fn decode_r_type(word: u32) -> Result<RType, DecodeError> {
    Ok(RType {
        rd: Register::try_from(rd(word))?,
        rs1: Register::try_from(rs1(word))?,
        rs2: Register::try_from(rs2(word))?,
        funct3: funct3(word),
        funct7: funct7(word),
    })
}

pub fn decode_i_type(word: u32) -> Result<IType, DecodeError> {
    Ok(IType {
        rd: Register::try_from(rd(word))?,
        rs1: Register::try_from(rs1(word))?,
        funct3: funct3(word),
        imm: sign_extend(bits(word, 31, 20), 12),
    })
}

pub fn decode_s_type(word: u32) -> Result<SType, DecodeError> {
    let imm = (bits(word, 31, 25) << 5) | bits(word, 11, 7);
    Ok(SType {
        rs1: Register::try_from(rs1(word))?,
        rs2: Register::try_from(rs2(word))?,
        funct3: funct3(word),
        imm: sign_extend(imm, 12),
    })
}

pub fn decode_b_type(word: u32) -> Result<BType, DecodeError> {
    let imm = (bits(word, 31, 31) << 12)
        | (bits(word, 7, 7) << 11)
        | (bits(word, 30, 25) << 5)
        | (bits(word, 11, 8) << 1);

    Ok(BType {
        rs1: Register::try_from(rs1(word))?,
        rs2: Register::try_from(rs2(word))?,
        funct3: funct3(word),
        imm: sign_extend(imm, 13),
    })
}

pub fn decode_u_type(word: u32) -> Result<UType, DecodeError> {
    Ok(UType {
        rd: Register::try_from(rd(word))?,
        imm: (word & 0xfffff000) as i32,
    })
}

pub fn decode_j_type(word: u32) -> Result<JType, DecodeError> {
    let imm = (bits(word, 31, 31) << 20)
        | (bits(word, 19, 12) << 12)
        | (bits(word, 20, 20) << 11)
        | (bits(word, 30, 21) << 1);

    Ok(JType {
        rd: Register::try_from(rd(word))?,
        imm: sign_extend(imm, 21),
    })
}
