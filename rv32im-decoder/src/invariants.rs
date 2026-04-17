use crate::{
    error::{DecodeError, Result},
    formats::{BType, IType, JType, RType, SType, UType},
    m_extension::{verify_mul_witness, MulWitness},
};

pub const OPCODE_LUI: u8 = 0b011_0111;
pub const OPCODE_AUIPC: u8 = 0b001_0111;
pub const OPCODE_JAL: u8 = 0b110_1111;
pub const OPCODE_JALR: u8 = 0b110_0111;
pub const OPCODE_BRANCH: u8 = 0b110_0011;
pub const OPCODE_LOAD: u8 = 0b000_0011;
pub const OPCODE_STORE: u8 = 0b010_0011;
pub const OPCODE_OP_IMM: u8 = 0b001_0011;
pub const OPCODE_OP: u8 = 0b011_0011;
pub const OPCODE_MISC_MEM: u8 = 0b000_1111;
pub const OPCODE_SYSTEM: u8 = 0b111_0011;

pub const FUNCT7_BASE: u8 = 0b000_0000;
pub const FUNCT7_SUB_SRA: u8 = 0b010_0000;
pub const FUNCT7_M: u8 = 0b000_0001;

#[inline]
pub fn ensure_register(index: u8) -> Result<()> {
    if index < 32 {
        Ok(())
    } else {
        Err(DecodeError::InvariantViolation(
            "register index must be in the range 0..32",
        ))
    }
}

#[inline]
pub fn ensure_shift_amount(shamt: u8) -> Result<()> {
    if shamt < 32 {
        Ok(())
    } else {
        Err(DecodeError::InvariantViolation(
            "shift amount must be in the range 0..32",
        ))
    }
}

#[inline]
pub fn validate_rtype(format: &RType) -> Result<()> {
    ensure_register(format.rd)?;
    ensure_register(format.rs1)?;
    ensure_register(format.rs2)
}

#[inline]
pub fn validate_itype(format: &IType) -> Result<()> {
    ensure_register(format.rd)?;
    ensure_register(format.rs1)
}

#[inline]
pub fn validate_stype(format: &SType) -> Result<()> {
    ensure_register(format.rs1)?;
    ensure_register(format.rs2)
}

#[inline]
pub fn validate_btype(format: &BType) -> Result<()> {
    ensure_register(format.rs1)?;
    ensure_register(format.rs2)
}

#[inline]
pub fn validate_utype(format: &UType) -> Result<()> {
    ensure_register(format.rd)
}

#[inline]
pub fn validate_jtype(format: &JType) -> Result<()> {
    ensure_register(format.rd)
}

#[inline]
pub fn is_valid_shamt(shamt: u8) -> bool {
    shamt < 32
}

#[inline]
pub fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32_u32.saturating_sub(bits as u32);
    ((value << shift) as i32) >> shift
}
