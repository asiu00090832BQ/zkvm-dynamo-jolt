// rv32im-decoder/src/lib.rs

//! A small RV32IM instruction decoder.
//!
//! This module exposes a single `decode` function that takes a raw 32‑bit
//! instruction word and returns a high‑level `Instruction` description.
//!
//! Only the integer base ISA (RV32I) plus the “M” extension are covered.

#![cfg_attr(not(test), no_std)]

/// Convenience result type for this crate.
pub type Result<T> = core::result::Result<T, DecodeError>;

/// Errors that can occur while decoding an instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    /// The 7‑bit opcode field did not match any known RV32IM opcode.
    UnknownOpcode(u8),
    /// The combination of `funct3` and/or `funct7` is not valid for the
    /// given opcode.
    InvalidFunct(u8, u8),
    /// System / CSR instruction could not be interpreted.
    InvalidSystem(u32),
}

/// All supported RV32IM opcodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // U‑type
    Lui,
    Auipc,

    // J‑type
    Jal,

    // I‑type (control)
    Jalr,

    // B‑type
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,

    // I‑type (loads)
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,

    // S‑type (stores)
    Sb,
    Sh,
    Sw,

    // I‑type (immediates)
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,

    // R‑type (integer register‑register)
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,

    // R‑type (M extension)
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,

    // FENCE
    Fence,
    FenceI,

    // SYSTEM
    Ecall,
    Ebreak,
    Csrrw,
    Csrrs,
    Csrrc,
    Csrrwi,
    Csrrsi,
    Csrrci,
}

/// A decoded instruction with its operands.
///
/// Not all fields are meaningful for every opcode. Unused fields are set to
/// `None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub rd: Option<u8>,
    pub rs1: Option<u8>,
    pub rs2: Option<u8>,
    /// Sign‑extended immediate (if any).
    pub imm: Option<i32>,
    /// CSR address for system / CSR instructions.
    pub csr: Option<u16>,
}

/// Decode a 32‑bit RV32IM instruction word.
///
/// Returns a high‑level `Instruction` or a [`DecodeError`].
pub fn decode(word: u32) -> Result<Instruction> {
    let op = (word & 0x7f) as u8;
    let funct3 = ((word >> 12) & 0x7) as u8;
    let funct7 = ((word >> 25) & 0x7f) as u8;

    match op {
        // LUI
        0x37 => Ok(Instruction {
            opcode: Opcode::Lui,
            rd: Some(rd(word)),
            rs1: None,
            rs2: None,
            imm: Some(imm_u(word)),
            csr: None,
        }),

        // AUIPC
        0x17 => Ok(Instruction {
            opcode: Opcode::Auipc,
            rd: Some(rd(word)),
            rs1: None,
            rs2: None,
            imm: Some(imm_u(word)),
            csr: None,
        }),

        // JAL
        0x6f => Sk(Instruction {
            opcode: Opcode::Jal,
            rd: Some(rd(word)),
            rs1: None,
            rs2: None,
            imm: Some(imm_j(word)),
            csr: None,
        }),

        // JALR
        0x67 => {
            let opcode = match funct3 {
                0x0 => Opcode::Jalr,
                _ => return Err(DecodeError::InvalidFunct(funct3, funct7)),
            };

            Ok(Instruction {
                opcode,
                rd: Some(rd(word)),
                rs1: Some(rs1(word)),
                rs2: None,
                imm: Some(imm_i(word)),
                csr: None,
            })
        }

        // BRANCH (B‑type)
        0x63 => {
            let opcode = match funct3 {
                0x0 => Opcode::Beq,
                0x1 => Opcode::Bne,
                0x4 => Opcode::Blt,
                0x5 => Opcode::Bge,
                0x6 => Opcode::Bltu,
                0x7 => Opcode::Bgeu,
                _ => return Err(DecodeError::InvalidFunct(funct3, funct7)),
            };

            Ok(Instruction {
                opcode,
                rd: None,
                rs1: Some(rs1(word)),
                rs2: Some(rs2(word)),
                imm: Some(imm_b(word)),
                csr: None,
            })
        }

        // LOAD (I‑type)
        0x03 => {
            let opcode = match funct3 {
                0x0 => Opcode::Lb,
                0x1 => Opcode::Lh,
                0x2 => Opcode::Lw,
                0x4 => Opcode::Lbu,
                0x5 => Opcode::Lhu,
                _ => return Err(DecodeError::InvalidFunct(funct3, funct7)),
            };

            Ok(Instruction {
                opcode,
                rd: Some(rd(word)),
                rs1: Some(rs1(word)),
                rs2: None,
                imm: Some(imm_i(word)),
                csr: None,
            })
        }

        // STORE (S‑type)
        0x23 => {
            let opcode = match funct3 {
                0x0 => Opcode::Sb,
                0x1 => Opcode::Sh,
                0x2 => Opcode::Sw,
                _ => return Err(DecodeError::InvalidFunct(funct3, funct7)),
            };

            Ok(Instruction {
                opcode,
                rd: None,
                rs1: Some(rs1(word)),
                rs2: Some(rs2(word)),
                imm: Some(imm_s(word)),
                csr: None,
            })
        }

        // OP‑IMM (I‑type register‑immediate)
        0x13 => {
            let shamt = ((word >> 20) & 0x1f) as u8;
            let opcode = match funct3 {
                0x0 => Opcode::Addi,
                0x2 => Opcode::Slti,
                0x3 => Opcode::Sltiu,
                0x4 => Opcode::Xori,
                0x6 => Opcode::Ori,
                0x7 => Opcode::Andi,
                0x1 => {
                    // SLLI: funct7 must be 0b0000000
                    if funct7 == 0x00 {
                        Opcode::Slli
                    } else {
                        return Err(DecodeError::InvalidFunct(funct3, funct7));
                    }
                }
                0x5 => {
                    // SRLI/SRAI share funct3; disambiguate via funct7
                    match funct7 {
                        0x00 => Opcode::Srli,
                        0x20 => Opcode::Srai,
                        _ => return Err(DecodeError::InvalidFunct(funct3, funct7)),
                    }
                }
                _ => return Err(DecodeError::InvalidFunct(funct3, funct7)),
            };

            let imm = if matches!(opcode, Opcode::Slli | Opcode::Srli | Opcode::Srai) {
                // For shifts, the immediate is actually the (zero‑extended) shamt
                shamt as i32
            } else {
                imm_i(word)
            };

            Ok(Instruction {
                opcode,
                rd: Some(rd(word)),
                rs1: Some(rs1(word)),
                rs2: None,
                imm: Some(imm),
                csr: None,
            })
        }

        // OP (R‑type register‑register)
        //
        // This is where a previous corruption occurred; the match below provides
        // full RV32I + “M” extension coverage for opcode 0x33.
        0x33 => {
            let opcode = match (funct7, funct3) {
                // RV32I, funct7 = 0b0000000
                (0x00, 0x0) => Opcode::Add,
                (0x00, 0x1) => Opcode::Sll,
                (0x00, 0x2) => Opcode::Slt,
                (0x00, 0x3) => Opcode::Sltu,
                (0x00, 0x4) => Opcode::Xor,
                (0x00, 0x5) => Opcode::Srl,
                (0x00, 0x6) => Opcode::Or,
                (0x00, 0x7) => Opcode::And,

                // RV32I, funct7 = 0b0100000
                (0x20, 0x0) => Opcode::Sub,
                (0x20, 0x5) => Opcode::Sra,

                // RV32M, funct7 = 0b0000001
                (0x01, 0x0) => Opcode::Mul,
                (0x01, 0x1) => Opcode::Mulh,
                (0x01, 0x2) => Opcode::Mulhsu,
                (0x01, 0x3) => Opcode::Mulhu,
                (0x01, 0x4) => Opcode::Div,
                (0x01, 0x5) => Opcode::Divu,
                (0x01, 0x6) => Opcode::Rem,
                (0x01, 0x7) => Opcode::Remu,

                _ => return Err(DecodeError::InvalidFunct(funct3, funct7)),
            };

            Ok(Instruction {
                opcode,
                rd: Some(rd(word)),
                rs1: Some(rs1(word)),
                rs2: Some(rs2(word)),
                imm: None,
                csr: None,
            })
        }

        // FENCE
        0x0f => {
            let opcode = match funct3 {
                0x0 => Opcode::Fence,
                0x1 => Opcode::FenceI,
                _ => return Err(DecodeError::InvalidFunct(funct3, funct7)),
            };

            Ok(Instruction {
                opcode,
                rd: None,
                rs1: None,
                rs2: None,
                imm: None,
                csr: None,
            })
        }

        // SYSTEM / CSR
        //
        // This arm used to be malformed; it now faithfully implements the
        // standard RV32I SYSTEM instructions (ECALL, EBREAK, CSR*).
        0x73 => {
            let imm12 = ((word >> 20) & 0x0fff) as u16;
            let csr = imm12;
            let rs1_val = rs1(word);

            let opcode = match funct3 {
                0x0 => {
                    // ECALL / EBREAK are distinguished by the 12‑bit immediate.
                    match imm12 {
                        0x000 => Opcode::Ecall,
                        0x001 => Opcode::Ebreak,
                        _ => return Err(DecodeError::InvalidSystem(word)),
                    }
                }

                // CSR register‑register operations
                0x1 => Opcode::Csrrw,
                0x2 => Opcode::Csrrs,
                0x3 => Opcode::Csrrc,

                // CSR immediate operations
                0x5 => Opcode::Csrrwi,
                0x6 => Opcode::Csrrsi,
                0x7 => Opcode::Csrrci,

                _ => return Err(DecodeError::InvalidSystem(word)),
            };

            // For ECALL/EBREAK, rd/rs1/rs2 are all x0 and the immediate is not
            // used as a general immediate, so we expose only the opcode.
            if matches!(opcode, Opcode::Ecall | Opcode::Ebreak) {
                return Ok(Instruction {
                    opcode,
                    rd: None,
                    rs1: None,
                    rs2: None,
                    imm: None,
                    csr: None,
                });
            }

            // CSR instructions follow I‑type format:
            //  [31:20] CSR
            //  [19:15] rs1 / zimm
            //  [11:7]  rd
            let imm = if matches!(opcode, Opcode::Csrrwi | Opcode::Csrrsi | Opcode::Csrrci) {
                // Immediate form: rs1 field actually encodes a 5‑bit unsigned
                // immediate (zimm).
                Some(rs1_val as i32)
            } else {
                None
            };

            Ok(Instruction {
                opcode,
                rd: Some(rd(word)),
                rs1: Some(rs1_val),
                rs2: None,
                imm,
                csr: Some(csr),
            })
        }

        _ => Err(DecodeError::UnknownOpcode(op)),
    }
}

/* ------------------------------------------------------------------------- */
/* Helpers                                                                   */
/* ------------------------------------------------------------------------- */

#[inline]
fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1f) as u8
}

#[inline]
fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}

#[inline]
fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

#[inline]
fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits as u32;
    ((value << shift) as i32) >> shift
}

// I‑type immediate (12‑bit signed)
#[inline]
fn imm_i(word: u32) -> i32 {
    let imm = (word >> 20) & 0x0fff;
    sign_extend(imm, 12)
}

// S‑type immediate (12‑bit signed)
#[inline]
fn imm_s(word: u32) -> i32 {
    let imm = ((word >> 25) << 5) | ((word >> 7) & 0x1f);
    sign_extend(imm, 12)
}

// B‑type immediate (13‑bit signed, LSB is always zero).
#[inline]
fn imm_b(word: u32) -> i32 {
    let bit12 = (word >> 31) & 0x1;
    let bit11 = (word >> 7) & 0x1;
    let bits10_5 = (word >> 25) & 0x3f;
    let bits4_1 = (word >> 8) & 0x0f;

    let imm = (bit12 << 12) | (bit11 << 11) | (bits10_5 << 5) | (bits4_1 << 1);
    sign_extend(imm, 13)
}

// U‑type immediate (20‑bit, already aligned to 12 LSBs).
#[inline]
fn imm_u(word: u32) -> i32 {
    (word & 0xfffff000) as i32
}

// J‑type immediate (21‑bit signed, LSB is zero).
#[inline]
fn imm_j(word: u32) -> i32 {
    let bit20 = (word >> 31) & 0x1;
    let bits10_1 = (word >> 21) & 0x3ff;
    let bit11 = (word >> 20) & 0x1;
    let bits19_12 = (word >> 12) & 0xff;

    let imm = (bit20 << 20) | (bits19_12 << 12) | (bit11 << 11) | (bits10_1 << 1);
    sign_extend(imm, 21)
}