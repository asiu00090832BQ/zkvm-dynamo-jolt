#![no_std]

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    IllegalInstruction(u32),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::IllegalInstruction(word) => {
                write!(f, "illegal instruction: 0x{word:08x}")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchOp {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadOp {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreOp {
    Sb,
    Sh,
    Sw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AluImmOp {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AluOp {
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
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemOp {
    Ecall,
    Ebreak,
    Csrrw,
    Csrrs,
    Csrrc,
    Csrrwi,
    Csrrsi,
    Csrrci,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui { rd: u8, imm: u32 },
    Auipc { rd: u8, imm: u32 },
    Jal { rd: u8, offset: i32 },
    Jalr { rd: u8, rs1: u8, offset: i32 },
    Branch {
        op: BranchOp,
        rs1: u8,
        rs2: u8,
        offset: i32,
    },
    Load {
        op: LoadOp,
        rd: u8,
        rs1: u8,
        offset: i32,
    },
    Store {
        op: StoreOp,
        rs1: u8,
        rs2: u8,
        offset: i32,
    },
    OpImm {
        op: AluImmOp,
        rd: u8,
        rs1: u8,
        imm: i32,
        shamt: u8,
    },
    Op {
        op: AluOp,
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fence,
    System {
        op: SystemOp,
        rd: u8,
        rs1: u8,
        csr: u16,
        zimm: u8,
    },
}

#[inline(always)]
fn bits(word: u32, hi: u8, lo: u8) -> u32 {
    (word >> lo) & ((1u32 << (hi - lo + 1)) - 1)
}

#[inline(always)]
fn bit(word: u32, idx: u8) -> u32 {
    (word >> idx) & 1
}

#[inline(always)]
fn sign_extend(value: u32, width: u8) -> i32 {
    let shift = 32 - width;
    ((value << shift) as i32) >> shift
}

#[inline(always)]
fn decode_i_imm(word: u32) -> i32 {
    sign_extend(bits(word, 31, 20), 12)
}

#[inline(always)]
fn decode_s_imm(word: u32) -> i32 {
    let imm = (bits(word, 31, 25) << 5) | bits(word, 11, 7);
    sign_extend(imm, 12)
}

#[inline(always)]
fn decode_b_imm(word: u32) -> i32 {
    let imm = (bit(word, 31) << 12)
        | (bit(word, 7) << 11)
        | (bits(word, 30, 25) << 5)
        | (bits(word, 11, 8) << 1);
    sign_extend(imm, 13)
}

#[inline(always)]
fn decode_u_imm(word: u32) -> u32 {
    word & 0xfffff000
}

#[inline(always)]
fn decode_j_imm(word: u32) -> i32 {
    let imm = (bit(word, 31) << 20)
        | (bits(word, 30, 21) << 1)
        | (bit(word, 20) << 11)
        | (bits(word, 19, 12) << 12);
    sign_extend(imm, 21)
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = bits(word, 6, 0);
    let rd = bits(word, 11, 7) as u8;
    let funct3 = bits(word, 14, 12);
    let rs1 = bits(word, 19, 15) as u8;
    let rs2 = bits(word, 24, 20) as u8;
    let funct7 = bits(word, 31, 25);

    let inst = match opcode {
        0b0110111 => Instruction::Lui {
            rd,
            imm: decode_u_imm(word),
        },
        0b0010111 => Instruction::Auipc {
            rd,
            imm: decode_u_imm(word),
        },
        0b1101111 => Instruction::Jal {
            rd,
            offset: decode_j_imm(word),
        },
        0b1100111 => {
            if funct3 != 0 {
                return Err(DecodeError::IllegalInstruction(word));
            }
            Instruction::Jalr {
                rd,
                rs1,
                offset: decode_i_imm(word),
            }
        }
        0b1100011 => {
            let op = match funct3 {
                0b000 => BranchOp::Beq,
                0b001 => BranchOp::Bne,
                0b100 => BranchOp::Blt,
                0b101 => BranchOp::Bge,
                0b110 => BranchOp::Bltu,
                0b111 => BranchOp::Bgeu,
                _ => return Err(DecodeError::IllegalInstruction(word)),
            };
            Instruction::Branch {
                op,
                rs1,
                rs2,
                offset: decode_b_imm(word),
            }
        }
        0b0000011 => {
            let op = match funct3 {
                0b000 => LoadOp::Lb,
                0b001 => LoadOp::Lh,
                0b010 => LoadOp::Lw,
                0b100 => LoadOp::Lbu,
                0b101 => LoadOp::Lhu,
                _ => return Err(DecodeError::IllegalInstruction(word)),
            };
            Instruction::Load {
                op,
                rd,
                rs1,
                offset: decode_i_imm(word),
            }
        }
        0b0100011 => {
            let op = match funct3 {
                0b000 => StoreOp::Sb,
                0b001 => StoreOp::Sh,
                0b010 => StoreOp::Sw,
                _ => return Err(DecodeError::IllegalInstruction(word)),
            };
            Instruction::Store {
                op,
                rs1,
                rs2,
                offset: decode_s_imm(word),
            }
        }
        0b0010011 => {
            let imm = decode_i_imm(word);
            let shamt = bits(word, 24, 20) as u8;
            let op = match funct3 {
                0b000 => AluImmOp::Addi,
                0b010 => AluImmOp::Slti,
                0b011 => AluImmOp::Sltiu,
                0b100 => AluImmOp::Xori,
                0b110 => AluImmOp::Ori,
                0b111 => AluImmOp::Andi,
                0b001 => {
                    if funct7 != 0b0000000 {
                        return Err(DecodeError::IllegalInstruction(word));
                    }
                    AluImmOp::Slli
                }
                0b101 => match funct7 {
                    0b0000000 => AluImmOp::Srli,
                    0b0100000 => AluImmOp::Srai,
                    _ => return Err(DecodeError::IllegalInstruction(word)),
                },
                _ => return Err(DecodeError::IllegalInstruction(word)),
            };
            Instruction::OpImm {
                op,
                rd,
                rs1,
                imm,
                shamt,
            }
        }
        0b0110011 => {
            let op = if funct7 == 0b0000001 {
                match funct3 {
                    0b000 => AluOp::Mul,
                    0b001 => AluOp::Mulh,
                    0b010 => AluOp::Mulhsu,
                    0b011 => AluOp::Mulhu,
                    0b100 => AluOp::Div,
                    0b101 => AluOp::Divu,
                    0b110 => AluOp::Rem,
                    0b111 => AluOp::Remu,
                    _ => return Err(DecodeError::IllegalInstruction(word)),
                }
            } else {
                match (funct7, funct3) {
                    (0b0000000, 0b000) => AluOp::Add,
                    (0b0100000, 0b000) => AluOp::Sub,
                    (0b0000000, 0b001) => AluOp::Sll,
                    (0b0000000, 0b010) => AluOp::Slt,
                    (0b0000000, 0b011) => AluOp::Sltu,
                    (0b0000000, 0b100) => AluOp::Xor,
                    (0b0000000, 0b101) => AluOp::Srl,
                    (0b0100000, 0b101) => AluOp::Sra,
                    (0b0000000, 0b110) => AluOp::Or,
                    (0b0000000, 0b111) => AluOp::And,
                    _ => return Err(DecodeError::IllegalInstruction(word)),
                }
            };
            Instruction::Op { op, rd, rs1, rs2 }
        }
        0b0001111 => {
            if funct3 == 0 {
                Instruction::Fence
            } else {
                return Err(DecodeError::IllegalInstruction(word));
            }
        }
        0b1110011 => {
            let csr = bits(word, 31, 20) as u16;
            let zimm = rs1;

            if funct3 == 0 {
                match bits(word, 31, 20) {
                    0 => Instruction::System {
                        op: SystemOp::Ecall,
                        rd: 0,
                        rs1: 0,
                        csr: 0,
                        zimm: 0,
                    },
                    1 => Instruction::System {
                        op: SystemOp::Ebreak,
                        rd: 0,
                        rs1: 0,
                        csr: 0,
                        zimm: 0,
                    },
                    _ => return Err(DecodeError::IllegalInstruction(word)),
                }
            } else {
                let op = match funct3 {
                    0b001 => SystemOp::Csrrw,
                    0b010 => SystemOp::Csrrs,
                    0b011 => SystemOp::Csrrc,
                    0b101 => SystemOp::Csrrwi,
                    0b110 => SystemOp::Csrrsi,
                    0b111 => SystemOp::Csrrci,
                    _ => return Err(DecodeError::IllegalInstruction(word)),
                };
                Instruction::System {
                    op,
                    rd,
                    rs1,
                    csr,
                    zimm,
                }
            }
        }
        _ => return Err(DecodeError::IllegalInstruction(word)),
        };

    Ok(inst)
}
