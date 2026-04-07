use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DecodeError {
    pub word: u32,
    pub reason: &'static str,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cannot decode instruction 0x{word:08x}: {reason}", word = self.word, reason = self.reason)
    }
}

impl StdError for DecodeError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchKind { Beq, Bne, Blt, Bge, Bltu, Bgeu }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadWidth { Byte, Half, Word, ByteUnsigned, HalfUnsigned }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreWidth { Byte, Half, Word }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AluOp { Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MulOp { Mul, Mulh, Mulhsu, Mulhu, Div, Divu, Rem, Remu }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui { rd: u8, imm: u32 },
    Auipc { rd: u8, imm: u32 },
    Jal { rd: u8, offset: i32 },
    Jalr { rd: u8, rs1: u8, offset: i32 },
    Branch { kind: BranchKind, rs1: u8, rs2: u8, offset: i32 },
    Load { width: LoadWidth, rd: u8, rs1: u8, offset: i32 },
    Store { width: StoreWidth, rs1: u8, rs2: u8, offset: i32 },
    OpImm { op: AluOp, rd: u8, rs1: u8, imm: i32 },
    Op { op: AluOp, rd: u8, rs1: u8, rs2: u8 },
    Mul { op: MulOp, rd: u8, rs1: u8, rs2: u8 },
    Fence { pred: u8, succ: u8 },
    FenceI, Ecall, Ebreak,
}

#[inline] fn rd(word: u32) -> u8 { ((word >> 7) & 0x1f) as u8 }
#[inline] fn rs1(word: u32) -> u8 { ((word >> 15) & 0x1f) as u8 }
#[inline] fn rs2(word: u32) -> u8 { ((word >> 20) & 0x1f) as u8 }
#[inline] fn funct3(word: u32) -> u32 { (word >> 12) & 0x7 }
#[inline] fn funct7(word: u32) -> u32 { (word >> 25) & 0x7f }

#[inline] fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - u32::from(bits);
    ((value << shift) as i32) >> shift
}

#[inline] fn decode_i_imm(word: u32) -> i32 { sign_extend((word >> 20) & 0x0fff, 12) }
#[inline] fn decode_s_imm(word: u32) -> i32 { sign_extend(((word >> 25) & 0x7f) << 5 | ((word >> 7) & 0x1f), 12) }
#[inline] fn decode_b_imm(word: u32) -> i32 {
    let imm = ((word >> 31) & 0x1) << 12 | ((word >> 7) & 0x1) << 11 | ((word >> 25) & 0x3f) << 5 | ((word >> 8) & 0x0f) << 1;
    sign_extend(imm, 13)
}
#[inline] fn decode_j_imm(word: u32) -> i32 {
    let imm = ((word >> 31) & 0x1) << 20 | ((word >> 12) & 0x0ff) << 12 | ((word >> 20) & 0x1) << 11 | ((word >> 21) & 0x03ff) << 1;
    sign_extend(imm, 21)
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = word & 0x7f;
    match opcode {
        0x37 => Ok(Instruction::Lui { rd: rd(word), imm: word & 0xfffff000 }),
        0x17 => Ok(Instruction::Auipc { rd: rd(word), imm: word & 0xfffff000 }),
        0x6f => Ok(Instruction::Jal { rd: rd(word), offset: decode_j_imm(word) }),
        0x67 => Ok(Instruction::Jalr { rd: rd(word), rs1: rs1(word), offset: decode_i_imm(word) }),
        0x63 => {
            let kind = match funct3(word) {
                0x0 => BranchKind::Beq, 0x1 => BranchKind::Bne, 0x4 => BranchKind::Blt, 0x5 => BranchKind::Bge, 0x6 => BranchKind::Bltu, 0x7 => BranchKind::Bgeu,
                _ => return Err(DecodeError { word, reason: "invalid funct3 for BRANCH" })
            };
            Ok(Instruction::Branch { kind, rs1: rs1(word), rs2: rs2(word), offset: decode_b_imm(word) })
        }
        0x03 => {
            let width = match funct3(word) {
                0x0 => LoadWidth::Byte, 0x1 => LoadWidth::Half, 0x2 => LoadWidth::Word, 0x4 => LoadWidth::ByteUnsigned, 0x5 => LoadWidth::HalfUnsigned,
                _ => return Err(DecodeError { word, reason: "invalid funct3 for LOAD" })
            };
            Ok(Instruction::Load { width, rd: rd(word), rs1: rs1(word), offset: decode_i_imm(word) })
        }
        0x23 => {
            let width = match funct3(word) {
                0x0 => StoreWidth::Byte, 0x1 => StoreWidth::Half, 0x2 => StoreWidth::Word,
                _ => return Err(DecodeError { word, reason: "invalid funct3 for STORE" })
            };
            Ok(Instruction::Store { width, rs1: rs1(word), rs2: rs2(word), offset: decode_s_imm(word) })
        }
        0x13 => {
            let imm = decode_i_imm(word);
            let rd = rd(word);
            let rs1 = rs1(word);
            match funct3(word) {
                0x0 => Ok(Instruction::OpImm { op: AluOp::Add, rd, rs1, imm }),
                0x2 => Ok(Instruction::OpImm { op: AluOp::Slt, rd, rs1, imm }),
                0x3 => Ok(Instruction::OpImm { op: AluOp::Sltu, rd, rs1, imm }),
                0x4 => Ok(Instruction::OpImm { op: AluOp::Xor, rd, rs1, imm }),
                0x6 => Ok(Instruction::OpImm { op: AluOp::Or, rd, rs1, imm }),
                0x7 => Ok(Instruction::OpImm { op: AluOp::And, rd, rs1, imm }),
                0x1 | 0x5 => {
                    let shamt = ((word >> 20) & 0x1f) as i32;
                    match (funct3(word), funct7(word)) {
                        (0x1, 0x00) => Ok(Instruction::OpImm { op: AluOp::Sll, rd, rs1, imm: shamt }),
                        (0x5, 0x00) => Ok(Instruction::OpImm { op: AluOp::Srl, rd, rs1, imm: shamt }),
                        (0x5, 0x20) => Ok(Instruction::OpImm { op: AluOp::Sra, rd, rs1, imm: shamt }),
                        _ => Err(DecodeError { word, reason: "invalid shift immediate encoding" })
                    }
                }
                _ => Err(DecodeError { word, reason: "invalid funct3 for OP-IMM" })
            }
        }
        0x33 => {
            let rd = rd(word); let rs1 = rs1(word); let rs2 = rs2(word);
            match (funct7(word), funct3(word)) {
                (0x00, 0x0) => Ok(Instruction::Op { op: AluOp::Add, rd, rs1, rs2 }),
                (0x00, 0x1) => Ok(Instruction::Op { op: AluOp::Sll, rd, rs1, rs2 }),
                (0x00, 0x2) => Ok(Instruction::Op { op: AluOp::Slt, rd, rs1, rs2 }),
                (0x00, 0x3) => Ok(Instruction::Op { op: AluOp::Sltu, rd, rs1, rs2 }),
                (0x00, 0x4) => Ok(Instruction::Op { op: AluOp::Xor, rd, rs1, rs2 }),
                (0x00, 0x5) => Ok(Instruction::Op { op: AluOp::Srl, rd, rs1, rs2 }),
                (0x00, 0x6) => Ok(Instruction::Op { op: AluOp::Or, rd, rs1, rs2 }),
                (0x00, 0x7) => Ok(Instruction::Op { op: AluOp::And, rd, rs1, rs2 }),
                (0x20, 0x0) => Ok(Instruction::Op { op: AluOp::Sub, rd, rs1, rs2 }),
                (0x20, 0x5) => Ok(Instruction::Op { op: AluOp::Sra, rd, rs1, rs2 }),
                (0x01, 0x0) => Ok(Instruction::Mul { op: MulOp::Mul, rd, rs1, rs2 }),
                (0x01, 0x1) => Ok(Instruction::Mul { op: MulOp::Mulh, rd, rs1, rs2 }),
                (0x01, 0x2) => Ok(Instruction::Mul { op: MulOp::Mulhsu, rd, rs1, rs2 }),
                (0x01, 0x3) => Ok(Instruction::Mul { op: MulOp::Mulhu, rd, rs1, rs2 }),
                (0x01, 0x4) => Ok(Instruction::Mul { op: MulOp::Div, rd, rs1, rs2 }),
                (0x01, 0x5) => Ok(Instruction::Mul { op: MulOp::Divu, rd, rs1, rs2 }),
                (0x01, 0x6) => Ok(Instruction::Mul { op: MulOp::Rem, rd, rs1, rs2 }),
                (0x01, 0x7) => Ok(Instruction::Mul { op: MulOp::Remu, rd, rs1, rs2 }),
                _ => Err(DecodeError { word, reason: "unknown funct7/funct3 for OP/MUL encoding" })
            }
        }
        0x0f => {
            match funct3(word) {
                0x0 => Ok(Instruction::Fence { pred: ((word >> 24) & 0x0f) as u8, succ: ((word >> 20) & 0x0f) as u8 }),
                0x1 => Ok(Instruction::FenceI),
                _ => Err(DecodeError { word, reason: "invalid funct3 for MISC-MEM" })
            }
        }
    -°x73 => match word {
            0x0000_0073 => Ok(Instruction::Ecall),
            0x0010_0073 => Ok(Instruction::Ebreak),
            _ => Err(DecodeError { word, reason: "unsupported SYSTEM instruction" })
        }
        _ => Err(DecodeError { word, reason: "unknown opcode" })
    }
}
