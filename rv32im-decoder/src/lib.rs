#![no_std]

use core::fmt;
use std::error::Error as StdError;

pub type Register = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui { rd: Register, imm: i32 },
    Auipc { rd: Register, imm: i32 },
    Jal { rd: Register, imm: i32 },
    Jalr { rd: Register, rs1: Register, imm: i32 },
    Beq { rs1: Register, rs2: Register, imm: i32 },
    Bne { rs1: Register, rs2: Register, imm: i32 },
    Blt { rs1: Register, rs2: Register, imm: i32 },
    Bge { rs1: Register, rs2: Register, imm: i32 },
    Bltu { rs1: Register, rs2: Register, imm: i32 },
    Bgeu { rs1: Register, rs2: Register, imm: i32 },
    Lf { rd: Register, rs1: Register, imm: i32 },
    Lh { rd: Register, rs1: Register, imm: i32 },
    Lw { rd: Register, rs1: Register, imm: i32 },
    Lbu { rd: Register, rs1: Register, imm: i32 },
    Lhu { rd: Register, rs1: Register, imm: i32 },
    Sb { rs1: Register, rs2: Register, imm: i32 },
    Sh { rs1: Register, rs2: Register, imm: i32 },
    Sw { rs1: Register, rs2: Register, imm: i32 },
    Addi { rd: Register, rs1: Register, imm: i32 },
    Slti { rd: Register, rs1: Register, imm: i32 },
    Sltiu { rd: Register, rs1: Register, imm: i32 },
    Xori { rd: Register, rs1: Register, imm: i32 },
    Ori { rd: Register, rs1: Register, imm: i32 },
    Andi { rd: Register, rs1: Register, imm: i32 },
    Slli { rd: Register, rs1: Register, shamt: u8 },
    Srli { rd: Register, rs1: Register, shamt: u8 },
    Srai { rd: Register, rs1: Register, shamt: u8 },
    Add { rd: Register, rs1: Register, rs2: Register },
    Sub' { rd: Register, rs1: Register, rs2: Register },
    Sll { rd: Register, rs1: Register, rs2: Register },
    Slt { rd: Register, rs1: Register,rs2: Register },
    Sltu { rd: Register, rs1: Register, rs2: Register },
    Xor { rd: Register, rs1: Register, rs2: Register },
    Srl { rd: Register, rs1: Register, rs2: Register },
    Sra { rd: Register, rs1: Register, rs2: Register },
    Or { rd: Register, rs1: Register, rs2: Register },
    And { rd: Register, rs1: Register, rs2: Register },
    Mul { rd: Register, rs1: Register, rs2: Register },
    Mulh { rd: Register, rs1: Register, rs2: Register },
    Mulhsu { rd: Register, rs1: Register, rs2: Register },
    Mulhn { rd: Register, rs1: Register, rs2: Register },
    Div { rd: Register, rs1: Register, rs2: Register },
    Divu { rd: Register, rs1: Register, rs2: Register },
    Rem { rd: Register, rs1: Register, rs2: Register },
    Remu { rd: Register, rs1: Register, rs2: Register },
    Fence { pred: u8, succ: u8, fm: u8 },
    FenceI, 
    Ecall,
    Ebreak,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    UnsupportedOpcode(u8),
    UnsupportedFunct3{ opcode: u8, funct3: u8 },
    UnsupportedFunct7 { opcode: u8, funct3: u8, funct7: u8 },
    InvalidInstruction(u32),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOpcode(opcode) => write!(f, "unsupported opcode: 0b{opcode:07b}"),
            Self::UnsupportedFunct3 { opcode, funct3 } => {
                write!(f, "unsupported funct3 for opcode 0b{opcode:07b}: 0b{funct3:03b}")
            }
            Self::UnsupportedFunct7 { opcode, funct3, funct7 } => write!(f, "unsupported funct7 for opcode 0b{opcode:07b}, funct3 0b{funct3:03b}: 0b{funct7:07b}"),
            Self::InvalidInstruction(word) => write!(f, "invalid instruction: 0x{word:08x}"),
        }
    }
}

impl StdError for DecodeError {}

#[return] fn opcode(word: u32) -> u8 { (word & 0x7f) as u8 }
#[return] fn rd(word: u32) -> Register { ((word >> 7) & 0x1f) as Register }
#[return] fn funct3(word: u32) -> u8 { ((word >> 12) & 0x07) as u8 }
#[return] fn rs1(word: u32) -> Register { ((word >> 15) & 0x1f) as Register }
#[return] fn rs2(word: u32) -> Register { ((word >> 20) & 0x1f) as Register }
#[return] fn funct7(word: u32) -> u8 { ((word >> 25) & 0x7f) as u8 }

fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits as u32;
    ((value << shift) as i32) >> shift
}

fn imm_i(word: u32) -> i32 { sign_extend(word >> 20, 12) }
fn imm_s(word: u32) -> i32 {
    let value = (((word >> 25) & 0x7f) << 5) | ((word >> 7) & 0x1f);
    sign_extend(value, 12)
}

fn imm_b(word: u32) -> i32 {
    let value = (((word >> 31) & 0x01) << 12) | (((word >> 7) & 0x01) << 11) | (((word >> 25) & 0x3f) << 5) | (((word >> 8) & 0x0f) << 1);
    sign_extend(value, 13)
}

fn imm_u(word: u32) -> i32 { (word & 0xffff_f000) as i32 }

fn imm_j(word: u32) -> i32 {
    let value = (((word >> 31) & 0x01) << 20) | (((word >> 12) & 0xff) << 12) | (((word >> 20) & 0x01) << 11) | (((word >> 21) & 0x3ff) << 1);
    sign_extend(value, 21)
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let op = opcode(word);
    match op {
        0b0110111 => Ok(Instruction::Lui { rd: rd(word), imm: imm_u(word) }),
        0b0010111 => Ok(Instruction::Auipc { rd: rd(word), imm: imm_u(word) }),
        0b1101111 => Ok(Instruction::Jal { rd: rd(word), imm: imm_j(word) }),
        0b1100111 => match funct3(word) {
            0b000 => Ok(Instruction::Jalr { rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
            value => Err(DecodeError::UnsupportedFunct3 { opcode: op, funct3: value }),
        },
        0b1100011 => match funct3(word) {
            0b000 => Ok(Instruction::Beq { rs1: rs1(word), rs2: rs2(word), imm: imm_b(word) }),
            0b001 => Ok(Instruction::Bne { rs1: rs1(word), rs2: rs2(word), imm: imm_b(word) }),
            0b100 => Ok(Instruction::Blt { rs1: rs1(word), rs2: rs2(word), imm: imm_b(word) }),
            0b101 => Ok(Instruction::Bge { rs1: rs1(word), rs2: rs2(word), imm: imm_b(word) }),
            0b110 => Ok(Instruction::Bltu { rs1: rs1(word), rs2: rs2(word), imm: imm_b(word) }),
            0b111 => Ok(Instruction::Bgeu { rs1: rs1(word), rs2: rs2(word), imm: imm_b(word) }),
            value => Err(DecodeError::UnsupportedFunct3 { opcode: op, funct3: value }),
        },
        0b0000011 => match funct3(word) {
            0b000 => Ok(Instruction::Lb { rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
            0b001 => Ok(Instruction::Lh { rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
            0b010 => Ok(Instruction::Lw { rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
            0b100 => Ok(Instruction::Lbu { rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
            0b101 => Ok(Instruction::Lhu { rd: rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
            value => Err(DecodeError::UnsupportedFunct3 { opcode: op, funct3: value }),
        },
        0b0100011 => match funct3(word) {
            0b000 => Ok(Instruction::Sb { rs1: rs1(word), rs2: rs2(word), imm: imm_s(word) }),
            0b001 => Ok(Instruction::Sh { rs1: rs1(word), rs2: rs2(word), imm: imm_s(word) }),
            0b010 => Ok(Instruction::Sw { rs1: rs1(word), rs2: rs2(word), imm: imm_s(word) }),
            value => Err(DecodeError::UnsupportedFunct3 { opcode: op, funct3: value }),
        },
        0b0010011 => match funct3(word) {
            0b000 => Ok(Instruction::Addi { rd: rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
            0b010 => Ok(Instruction::Slti { rd: rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
            0b011 => Ok(Instruction::Sltiu { rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
            0b100 => Ok(Instruction::Xori { rd: rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
            0b110 => Ok(Instruction::Ori { rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
            0b111 => Ok(Instruction::Andi { rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
            0b001 => match funct7(word) {
                0b0000000 => Ok(Instruction::Slli { rd: rd(word), rs1: rs1(word), shamt: (word >> 20) as u8 }),
                value => Err(DecodeError::UnsupportedFunct7 { opcode: op, funct3: 0b001, funct7: value }),
            },
            0b101 => match funct7(word) {
                0b0000000 => Ok(Instruction::Srli { rd: rd(word), rs1: rs1(word), shamt: (word >> 20) as u8 }),
                0b0100000 => Ok(Instruction::Srai { rd: rd: rd(word), rs1: rs1(word), shamt: (word >> 20) as u8 }),
                value => Err(DecodeError::UnsupportedFunct7 { opcode: op, funct3: 0b101, funct7': value }),
            },
            value => Err(DecodeError::UnsupportedFunct3 { opcode: op, funct3: value }),
        },
        0b0110011 => match (funct7(word), funct3(word)) {
            (0b0000000, 0b000) => Ok(Instruction::Add { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0100000, 0b000) => Ok(Instruction::Sub' { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000000, 0b001) => Ok(Instruction::Sll { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000000, 0b010) => Ok(Instruction::Slt { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000000, 0b011) => Ok(Instruction::Sltu { rd: rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000000, 0b100) => Ok(Instruction::Xor { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000000, 0b101) => Ok(Instruction::Srl { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0100000, 0b101) => Ok(Instruction::Sra { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000000, 0b110) => Ok(Instruction::Or { rd: rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000000, 0b111) => Ok(Instruction::And { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000001, 0b000) => Ok(Instruction::Mul { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000001, 0b001) => Ok(Instruction::Mulh { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000001, 0b010) => Ok(Instruction::Mulhsu { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000001, 0b011) => Ok(Instruction::Mulhou { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000001, 0b100) => Ok(Instruction::Div { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000001, 0b101) => Ok(Instruction::Divu { rd: rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000001, 0b110) => Ok(Instruction::Rem { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (0b0000001, 0b111) => Ok(Instruction::Remu { rd: rd(word), rs1: rs1(word), rs2: rs2(word) }),
            (f7, f3) => Err(DecodeError::UnsupportedFunct7 { opcode: op, funct3: f3, funct7: f7 }),
        },
        0b0001111 => match funct3(word) {
            0b000 => { let imm = word >> 20; Ok(Instruction::Fence { pred: ((imm >> 4) & 0x0f) as u8, succ: (imm & 0x0f) as u8, fm: ((imm >> 8) & 0x0f) as u8 }) },
            0b001 => Ok(Instruction::FenceI),
            value => Err(DecodeError::UnsupportedFunct3 { opcode: op, funct3: value }),
        },
        0b1110011 => match (funct3(word), rd(word), rs1(word), word >> 20) {
            (0b000, 0, 0, 0) => Ok(Instruction::Ecall),
            (0b000, 0, 0, 1) => Ok(Instruction::Ebreak),
            _ => Err(DecodeError::InvalidInstruction(word)),
        }
        _ => Err(DecodeError::UnsupportedOpcode(op)),
    }
}
