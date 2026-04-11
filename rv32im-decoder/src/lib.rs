//! RV32IM instruction decoder (Base RV32I + M-extension).
//!
//! Clean, idiomatic Rust with a complete decode of the RV32IM instruction set.
//! This crate exposes a single, UTF-8 clean API surface:
//!   - `Instruction` enum encapsulating all RV32IM instructions
//!   - `decode(u32) -> Result<Instruction, DecodeError>`
//!
//! Notes:
//! - No compressed (C) extension support; every instruction must be 32 bits.
//! - SUB variant is correctly named `Sub` (fixes prior `Sub'` typo).
//! - CSR instructions are supported.

#![forbid(unsafe_code)]

use core::fmt;

/// A 5-bit register index (x0..x31).
pub type Reg = u8;

/// Decode errors for RV32IM.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    InvalidOpcode(u32),
    InvalidFunct3(u32),
    InvalidFunct7(u32),
    MisalignedInstructionAddress(u32),
    IllegalInstruction(u32),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::InvalidOpcode(w) => write!(f, "invalid opcode: 0x{w:08x}"),
            DecodeError::InvalidFunct3(w) => write!(f, "invalid funct3: 0x{w:08x}"),
            DecodeError::InvalidFunct7(w) => write!(f, "invalid funct7: 0x{w:08x}"),
            DecodeError::MisalignedInstructionAddress(pc) => {
                write!(f, "misaligned instruction address: 0x{pc:08x}")
            }
            DecodeError::IllegalInstruction(w) => write!(f, "illegal instruction: 0x{w:08x}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

/// All RV32IM instructions (I + M).
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    // U-type
    Lui { rd: Reg, imm: u32 },
    Auipc { rd: Reg, imm: u32 },

    // J-type
    Jal { rd: Reg, imm: i32 },

    // I-type
    Jalr { rd: Reg, rs1: Reg, imm: i32 },
    LoadB { rd: Reg, rs1: Reg, imm: i32 },
    LoadH { rd: Reg, rs1: Reg, imm: i32 },
    LoadW { rd: Reg, rs1: Reg, imm: i32 },
    LoadBu { rd: Reg, rs1: Reg, imm: i32 },
    LoadHu { rd: Reg, rs1: Reg, imm: i32 },
    Addi { rd: Reg, rs1: Reg, imm: i32 },
    Slti { rd: Reg, rs1: Reg, imm: i32 },
    Sltiu { rd: Reg, rs1: Reg, imm: i32 },
    Xori { rd: Reg, rs1: Reg, imm: i32 },
    Ori { rd: Reg, rs1: Reg, imm: i32 },
    Andi { rd: Reg, rs1: Reg, imm: i32 },
    Slli { rd: Reg, rs1: Reg, shamt: u32 },
    Srli { rd: Reg, rs1: Reg, shamt: u32 },
    Srai { rd: Reg, rs1: Reg, shamt: u32 },

    // S-type
    StoreB { rs1: Reg, rs2: Reg, imm: i32 },
    StoreH { rs1: Reg, rs2: Reg, imm: i32 },
    StoreW { rs1: Reg, rs2: Reg, imm: i32 },

    // B-type
    Beq { rs1: Reg, rs2: Reg, imm: i32 },
    Bne { rs1: Reg, rs2: Reg, imm: i32 },
    Blt { rs1: Reg, rs2: Reg, imm: i32 },
    Bge { rs1: Reg, rs2: Reg, imm: i32 },
    Bltu { rs1: Reg, rs2: Reg, imm: i32 },
    Bgeu { rs1: Reg, rs2: Reg, imm: i32 },

    // R-type (I-extension)
    Add { rd: Reg, rs1: Reg, rs2: Reg },
    Sub { rd: Reg, rs1: Reg, rs2: Reg },
    Sll { rd: Reg, rs1: Reg, rs2: Reg },
    Slt { rd: Reg, rs1: Reg, rs2: Reg },
    Sltu { rd: Reg, rs1: Reg, rs2: Reg },
    Xor { rd: Reg, rs1: Reg, rs2: Reg },
    Srl { rd: Reg, rs1: Reg, rs2: Reg },
    Sra { rd: Reg, rs1: Reg, rs2: Reg },
    Or { rd: Reg, rs1: Reg, rs2: Reg },
    And { rd: Reg, rs1: Reg, rs2: Reg },

    // R-type (M-extension)
    Mul { rd: Reg, rs1: Reg, rs2: Reg },
    Mulh { rd: Reg, rs1: Reg, rs2: Reg },
    Mulhsu { rd: Reg, rs1: Reg, rs2: Reg },
    Mulhu { rd: Reg, rs1: Reg, rs2: Reg },
    Div { rd: Reg, rs1: Reg, rs2: Reg },
    Divu { rd: Reg, rs1: Reg, rs2: Reg },
    Rem { rd: Reg, rs1: Reg, rs2: Reg },
    Remu { rd: Reg, rs1: Reg, rs2: Reg },

    // Misc-mem
    Fence { pred: u8, succ: u8 },

    // System
    Ecall,
    Ebreak,
    Csrrw { rd: Reg, rs1: Reg, csr: u16 },
    Csrrs { rd: Reg, rs1: Reg, csr: u16 },
    Csrrc { rd: Reg, rs1: Reg, csr: u16 },
    Csrrwi { rd: Reg, zimm: u8, csr: u16 },
    Csrrsi { rd: Reg, zimm: u8, csr: u16 },
    Csrrci { rd: Reg, zimm: u8, csr: u16 },
}

#[inline]
const fn bit(val: u32, idx: u32) -> u32 {
    (val >> idx) & 1
}

#[inline]
const fn bits(val: u32, hi: u32, lo: u32) -> u32 {
    (val >> lo) & ((1u32 << (hi - lo + 1)) - 1)
}

#[inline]
fn sext(value: u32, width: u32) -> i32 {
    let shift = 32 - width;
    ((value << shift) as i32) >> shift
}

fn decode_imm_i(word: u32) -> i32 {
    sext(bits(word, 31, 20), 12)
}
fn decode_imm_s(word: u32) -> i32 {
    let imm = (bits(word, 31, 25) << 5) | bits(word, 11, 7);
    sext(imm, 12)
}
fn decode_imm_b(word: u32) -> i32 {
    // imm[12|10:5|4:1|11] << 1
    let imm12 = bit(word, 31);
    let imm11 = bit(word, 7);
    let imm10_5 = bits(word, 30, 25);
    let imm4_1 = bits(word, 11, 8);
    let imm = (imm12 << 12) | (imm11 << 11) | (imm10_5 << 5) | (imm4_1 << 1);
    sext(imm, 13)
}
fn decode_imm_u(word: u32) -> u32 {
    word & 0xfffff000
}
fn decode_imm_j(word: u32) -> i32 {
    // imm[20|10:1|11|19:12] << 1
    let imm20 = bit(word, 31);
    let imm10_1 = bits(word, 30, 21);
    let imm11 = bit(word, 20);
    let imm19_12 = bits(word, 19, 12);
    let imm = (imm20 << 20) | (imm19_12 << 12) | (imm11 << 11) | (imm10_1 << 1);
    sext(imm, 21)
}

/// Decode a 32-bit instruction word into a high-level `Instruction`.
pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = bits(word, 6, 0) as u8;
    let rd = bits(word, 11, 7) as Reg;
    let funct3 = bits(word, 14, 12) as u8;
    let rs1 = bits(word, 19, 15) as Reg;
    let rs2 = bits(word, 24, 20) as Reg;
    let funct7 = bits(word, 31, 25) as u8;

    match opcode {
        0x37 => Ok(Instruction::Lui { rd, imm: decode_imm_u(word) }),
        0x17 => Ok(Instruction::Auipc { rd, imm: decode_imm_u(word) }),
        0x6f => Ok(Instruction::Jal { rd, imm: decode_imm_j(word) }),
        0x67 => {
            // JALR: funct3 must be 000
            if funct3 != 0 {
                return Err(DecodeError::InvalidFunct3(word));
            }
            Ok(Instruction::Jalr {
                rd,
                rs1,
                imm: decode_imm_i(word),
            })
        }
        0x63 => {
            // Branch
            let imm = decode_imm_b(word);
            match funct3 {
                0x0 => Ok(Instruction::Beq { rs1, rs2, imm }),
                0x1 => Ok(Instruction::Bne { rs1, rs2, imm }),
                0x4 => Ok(Instruction::Blt { rs1, rs2, imm }),
                0x5 => Ok(Instruction::Bge { rs1, rs2, imm }),
                0x6 => Ok(Instruction::Bltu { rs1, rs2, imm }),
                0x7 => Ok(Instruction::Bgeu { rs1, rs2, imm }),
                _ => Err(DecodeError::InvalidFunct3(word)),
            }
        }
        0x03 => {
            // Load
            let imm = decode_imm_i(word);
            match funct3 {
                0x0 => Ok(Instruction::LoadB { rd, rs1, imm }),
                0x1 => Ok(Instruction::LoadH { rd, rs1, imm }),
                0x2 => Ok(Instruction::LoadW { rd, rs1, imm }),
                0x4 => Ok(Instruction::LoadBu { rd, rs1, imm },
                0x5 => Ok(Instruction::LoadHu { rd, rs1, imm }),
                _ => Err(DecodeError:::InvalidFunct3(word)),
            }
        }
        0x23 => {
            // Store
            let imm = decode_imm_s(word);
            match funct3 {
                0x0 => Ok(Instruction::StoreB { rs1, rs2, imm }),
                0x1 => Ok(Instruction::StoreH { rs1, rs2, imm }),
                0x2 => Ok(Instruction::StoreW { rs1, rs2, imm }),
                _ => Err(DecodeError::InvalidFunct3(word)),
            }
        }
        0x13 => {
            // OP-IMM
            let imm = decode_imm_i(word);
            match funct3 {
                0x0 => Ok(Instruction::Addi { rd, rs1, imm }),
                0x2 => Ok(Instruction::Slti { rd, rs1, imm }),
                0x3 => Ok(Instruction::Sltiu { rd, rs1, imm }),
                0x4 => Ok(Instruction::Xori { rd, rs1, imm }),
                0x6 => Ok(Instruction::Ori { rd, rs1, imm }),
                0x7 => Ok(Instruction::Andi { rd, rs1, imm }),
                0x1 => {
                    // SLLI: funct7 must be 0000000, shamt[4:0] from rs2 field
                    if funct7 != 0x00 {
                        return Err(DecodeError::InvalidFunct7(word));
                    }
                    Ok(Instruction::Slli {
                        rd,
                        rs1,
                        shamt: bits(word, 24, 20),
                    })
                }
                0x5 => {
                    // SRLI/SRAI distinguished by funct7
                    match funct7 {
                        0x00 => Ok(Instruction::Srli {
                            rd,
                            rs1,
                            shamt: bits(word, 24, 20),
                        }),
                        0x20 => Ok(Instruction::Srai {
                            rd,
                            rs1,
                            shamt: bits(word, 24, 20),
                        }),
                        _ => Err(DecodeError::InvalidFunct7(word)),
                    }
                }
                _ => Err(DecodeError::InvalidFunct3(word)),
            }
        }
        0x33 => {
            // OP (R-type): I and M extension
            match (funct7, funct3) {
                (0x00, 0x0) => Ok(Instruction::Add { rd, rs1, rs2 }),
                (0x20, 0x0) => Ok(Instruction::Sub { rd, rs1, rs2 }),
                (0x00, 0x1) => Ok(Instruction::Sll { rd, rs1, rs2 }),
                (0x00, 0x2) => Ok(Instruction::Slt { rd, rs1, rs2 }),
                (0x00, 0x3) => Ok(Instruction::Sltu { rd, rs1, rs2 }),
                (0x00, 0x4) => Ok(Instruction::Xor { rd, rs1, rs2 }),
                (0x00, 0x5) => Ok(Instruction::Srl { rd, rs1, rs2 }),
                (0x20, 0x5) => Ok(Instruction::Sra { rd, rs1, rs2 }),
                (0x00, 0x6) => Ok(Instruction::Or { rd, rs1, rs2 }),
                (0x00, 0x7) => Ok(Instruction::And { rd, rs1, rs2 }),
                // M-extension
                (0x01, 0x0) => Ok(Instruction::Mul { rs1, rs2, rd }),
                (0x01, 0x1) => Ok(Instruction::Mulh { rd, rs1, rs2 }),
                (0x01, 0x2) => Ok(Instruction::Mulhsu { rd, rs1, rs2 }),
                (0x01, 0x3) => Ok(Instruction::Mulhu { rd, rs1, rs2 }),
                (0x01, 0x4) => Ok(Instruction::Div { rd, rs1, rs2 }),
                (0x01, 0x5) => Ok(Instruction::Divu { rd, rs1, rs2 }),
                (0x01, 0x6) => Ok(Instruction::Rem { rd, rs1, rs2 }),
                (0x01, 0x7) => Ok(Instruction::Remu { rd, rs1, rs2 }),
                _ => Err(DecodeError::IllegalInstruction(word)),
            }
        }
        0x0f => {
            // FENCE family
            if funct3 == 0x0 {
                let pred = bits(word, 27, 24) as u8;
                let succ = bits(word, 23, 20) as u8;
                Ok(Instruction::Fence { pred, succ })
            } else {
                Err(DecodeError::InvalidFunct3(word))
            }
        }
        0x73 => {
            // SYSTEM
            match funct3 {
                0x0 => {
                    let imm12 = bits(word, 31, 20);
                    match imm12 {
                        0x000 => Ok(Instruction::Ecall),
                        0x001 => Ok(Instruction::Ebreak),
                        _ => Err(DecodeError::IllegalInstruction(word)),
                    }
                }
                0x1 => Ok(Instruction::Csrrw {
                    rd,
                    rs1,
                    csr: bits(word, 31, 20) as u16,
                }),
                0x2 => Ok(Instruction::Csrrs {
                    rd,
                    rs1,
                    csr: bits(word, 31, 20) as u16,
                }),
                0x3 => Ok(Instruction::Csrrc {
                    rd,
                    rs1,
                    csr: bits(word, 31, 20) as u16,
                }),
                0x5 => Ok(Instruction::Csrrwi {
                    rd,
                    zimm: rs1,
                    csr: bits(word, 31, 20) as u16,
                }),
                0x6 => Ok(Instruction::Csrrsi {
                    rd,
                    zimm: rs1,
                    csr: bits(word, 31, 20) as u16,
                }),
                0x7 => Ok(Instruction::Csrrci {
                    rd,
                    zimm: rs1,
                    csr: bits(word, 31, 20) as u16,
                }),
                _ => Err(DecodeError::InvalidFunct3(word)),
            }
        }
        _ => Err(DecodeError::InvalidOpcode(word)),
    }
}
