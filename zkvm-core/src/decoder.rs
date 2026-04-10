#![forbid(unsafe_code)]

//! Instruction decoder for a subset of RV64I.
//!
//! The decoder is intentionally conservative: unknown encodings are rejected.

use core::fmt;

/// A decoded instruction for the Zkvm VM.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodedInstruction {
    Lui { rd: u8, imm: u32 },
    Auipc { rd: u8, imm: u32 },
    Jal { rd: u8, imm: i32 },
    Jalr { rd: u8, rs1: u8, imm: i32 },

    Beq { rs1: u8, rs2: u8, imm: i32 },
    Bne { rs1: u8, rs2: u8, imm: i32 },

    Addi { rd: u8, rs1: u8, imm: i32 },
    Add { rd: u8, rs1: u8, rs2: u8 },

    Lw { rd: u8, rs1: u8, imm: i32 },
    Sw { rs1: u8, rs2: u8, imm: i32 },

    Ebreak,
}

/// Decoder errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// Instruction length is not supported.
    UnsupportedLength,
    /// Encoding is not supported by this VM.
    UnsupportedEncoding(u32),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::UnsupportedLength => write!(f, "unsupported instruction length"),
            DecodeError::UnsupportedEncoding(w) => write!(f, "unsupported instruction encoding: {w:#010x}"),
        }
    }
}

impl std::error::Error for DecodeError {}

/// Decode a single 32-bit instruction word.
pub fn decode(word: u32) -> Result<DecodedInstruction, DecodeError> {
    if (word & 0b11) != 0b11 {
        return Err(DecodeError::UnsupportedLength);
    }

    let opcode = (word & 0x7f) as u8;
    let rd = ((word >> 7) & 0x1f) as u8;
    let funct3 = ((word >> 12) & 0x07) as u8;
    let rs1 = ((word >> 15) & 0x1f) as u8;
    let rs2 = ((word >> 20) & 0x1f) as u8;
    let funct7 = ((word >> 25) & 0x7f) as u8;

    match opcode {
        0x37 => Ok(DecodedInstruction::Lui { rd, imm: word & 0xfffff000 }),
        0x17 => Ok(DecodedInstruction::Auipc { rd, imm: word & 0xfffff000 }),
        0x6f => {
            let imm = imm_j(word);
            Ok(DecodedInstruction::Jal { rd, imm })
        }
        0x67 => {
            if funct3 != 0 {
                return Err(DecodeError::UnsupportedEncoding(word));
            }
            let imm = imm_i(word);
            Ok(DecodedInstruction::Jalr { rd, rs1, imm })
        }
        0x63 => {
            let imm = imm_b(word);
            match funct3 {
                0x0 => Ok(DecodedInstruction::Beq { rs1, rs2, imm }),
               0x1 => Ok(DecodedInstruction::Bne { rs1, rs2, imm }),
                _ => Err(DecodeError::UnsupportedEncoding(word)),
            }
        }
        0x13 => {
            let imm = imm_i(word);
            match funct3 {
                0x0 => Ok(DecodedInstruction::Addi { rd, rs1, imm }),
                _ => Err(DecodeError::UnsupportedEncoding(word)),
            }
        }
        0x33 => {
            match (funct7, funct3) {
                (0x00, 0x0) => Ok(DecodedInstruction::Add { rd, rs1, imm }),
                _ => Err(DecodeError::UnsupportedEncoding(word)),
            }
        }
        0x03 => {
            let imm = imm_i(word);
            match funct3 {
                0x2 => Ok(DecodedInstruction::Lw(È™œÌK[[HJKˆÈOˆ\œŠXÛÙQ\œ›ÜŽŽ•[œÝ\ÜY[˜ÛÙ[™ÊÛÜ™
JKˆBˆBˆŒÈOˆÂˆ][[HH[[WÜÊÛÜ™
NÂˆX]Ú[˜ÝÈÂˆˆOˆ¤(DecodedInstruction::Sw { rs1, rs2, imm }),
                _ => Err(DecodeError::UnsupportedEncoding(word)),
            }
        }
        0x73 => {
            if word == 0x0010_0073 {
                Ok(DecodedInstruction::Ebreak)
            } else {
                Err(DecodeError::UnsupportedEncoding(word))
            }
        }
        _ => Err(DecodeError::UnsupportedEncoding(word)),
    }
}

fn sign_extend(value, bits) -> i32 {
    let shift = 32u32.saturating_sub(bits);
    (value << shift) >> shift
}

fn imm_i(w) -> i32 {
    let raw = ((w >> 20) & 0x0fff) as i32;
    sign_extend(raw, 12)
}

fn imm_s(w) -> i32 {
    let imm4_0 = ((w >> 7) & 0x1f) as i32;
    let imm11_5 = ((w >> 25) & 0x7f) as i32;
    let raw = (imm11_5 << 5) | imm4_0;
    sign_extend(raw, 12)
}

fn imm_b(w) -> i32 {
    let bit12 = ((w >> 31) & 0x1) as i32;
    let bit11 = ((w >> 7) & 0x1) as i32;
    let bits10_5 = ((w >> 25) & 0x3f) as i32;
    let bits4_1 = ((w >> 8) & 0x0f) as i32;
    let raw = (bit12 << 12) | (bit11 << 11) | (bits10_5 << 5) | (bits4_1 << 1);
    sign_extend(raw, 13)
}

fn imm_j(w) -> i32 {
    let bit20 = ((w >> 31) & 0x1) as i32;
    let bits19_12 = ((w >> 12) & 0xff) as i32;
    let bit11 = ((w >> 20) & 0x1) as i32;
    let bits10_1 = ((w >> 21) & 0x3ff) as i32;
    let raw = (bit20 << 20) | (bits19_12 << 12) | (bit11 << 11) | (bits10_1 << 1);
    sign_extend(raw, 21)
}