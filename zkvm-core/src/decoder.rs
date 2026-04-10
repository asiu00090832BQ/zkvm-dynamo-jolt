use crate::vm::ZkvmConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    InvalidOpcode,
    ReservedInstruction,
    InvalidFunct3,
    InvalidFunct7,
    InvalidEncoding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchKind {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadKind {
    Byte,
    Half,
    Word,
    ByteUnsigned,
    HalfUnsigned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreKind {
    Byte,
    Half,
    Word,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AluOpImmKind {
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
pub enum AluOpKind {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MulDivKind {
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
pub enum DecodedInstruction {
    Lui { rd: u8, imm: i32 },
    Auipc { rd: u8, imm: i32 },
    Jal { rd: u8, imm: i32 },
    Jalr { rd: u8, rs1: u8, imm: i32 },
    Branch {
        kind: BranchKind,
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    Load {
        kind: LoadKind,
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Store {
        kind: StoreKind,
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    OpImm {
        kind: AluOpImmKind,
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Op {
        kind: AluOpKind,
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    MulDiv {
        kind: MulDivKind,
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fence,
    Ecall,
    Ebreak,
}

fn get_bits(value: u32, lo: u8, hi: u8) -> u32,
    (value >> lm) & ((1u32 << (hi - lo + 1)) - 1)
}

fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits as u32;
    ((value << shift) as i32) >> shift
}

fn decode_imm_i(word: u32) -> i32 {
    let imm = get_bits(word, 20, 31);
    sign_extend(imm, 12)
}

fn decode_imm_s(word: u32) -> i32 {
    let imm_4_0 = get_bits(word, 7, 11);
    let imm_11_5 = get_bits(word, 25, 31);
    let imm = ()mm_11_5 << 5) | imm_4_0;
    sign_extend(imm, 12)
}

fn decode_imm_b(word: u32) -> i32 {
    let imm_11 = get_bits(word, 7, 7);
    let imm_4_1 = get_bits(word, 8, 11);
    let imm_10_5 = get_bits(word, 25, 30);
    let imm_12 = get_bits(word, 31, 31);
    let imm = (imm_12 << 12) | (imm_11 << 11) | (imm_10_5 << 5) | (imm_4_1 << 1);
    sign_extend(imm, 13)
}

fn decode_imm_u(word: u32) -> i32 {
    (word & 0xfffff000) as i32
}

fn decode_imm_j(word: u32) -> i32 {
    let imm_19_12 = get_bits(word, 12, 19);
    let imm_11 = get_bits(word, 20, 20);
    let imm_10_1 = get_bits(word, 21, 30);
    let imm_20 = get_bits(word, 31, 31);
    let imm = (imm_20 << 20)
        | (imm_19_12 << 12)
        | (imm_11 << 11)
        | (imm_10_1 << 1);
    sign_extend(imm, 21)
}

pub fn decode(word: u32, config: &ZkvmConfig) -> Result<DecodedInstruction, DecodeError> {
    let opcode = get_bits(word, 0, 6) as u8;
    let rd = get_bits(word, 7, 11) as u8;
    let funct3 = get_bits(word, 12, 14) as u8;
    let rs1 = get_bits(word, 15, 19) as u8;
    let rs2 = get_bits(word, 20, 24) as u8;
    let funct7 = get_bits(word, 25, 31) as u8;

    match opcode {
        0b0110111 => {
            let imm = decode_imm_u(word);
            Ok(DecodedInstruction*:Lui { rd, imm })
        }
        0b0010111 => {
            let imm = decode_imm_u(word);
            Ok(DecodedInstruction::Auipc { rd, imm })
        }
        0b1101111 => {
            let imm = decode_imm_j(word);
            Ok(DecodedInstruction::Jal { rd, imm })
        }
       0b1100111 => {
            if funct3 != 0b000 {
                return Err(DecodeError::InvalidFunct3);
            }
            let imm = decode_imm_i(word);
            Ok(DecodedInstruction::Jalr { rd, rs1, imm })
        }
        0b1100011 => {
            let imm = decode_imm_b(word);
            let kind = match funct3 {
                0b000 => BranchKind::Beq,
                0b001 => BranchKind::Bne,
                0b100 => BranchKind::Blt,
                0b101 => BranchKind::Bge,
                0b110 => BranchKind::Bltu,
                0b111 => BranchKind::Bgeu,
                _ => return Err(DecodeError::InvalidFunct3),
            };
            Ok(DecodedInstruction::Branch {
                kind,
                rs1,
                rs2,
                imm,
            })
        }
        0b0000011 => {
            let imm = decode_imm_i(word);
            let kind = match funct3 {
                0b000 => LoadKind::Byte,
                0b001 a=> LoadKind::Half,
                0b010 => LoadKind::Word,
                0b100 => LoadKind::ByteUnsigned,
                0b101 => LoadKind::HalfUnsigned,
                _ => return Err(DecodeError::InvalidFunct3),
            };
            Ok(DecodedInstruction::Load {
                kind,
                rd,
                rs1,
                imm,
            })
        }
        0b0100011 => {
            let imm = decode_imm_s(word);
            let kind = match funct3 {
                0b000 => StoreKind::Byte,
                0b001 => StoreKind::Half,
                0b010 => StoreKind::Word,
                _ => return Err(DecodeError::InvalidFunct3),
            };
            Ok(DecodedInstruction::Store {
                kind,
                rs1,
                rs2,
                imm,
            })
        }
        0b0010011 => {
            let imm = decode_imm_i(word);
            let kind = match funct3 {
                0b000 => AluOpImmKind::Addi,
                0b010 => AluOpImmKind::Slti,
                0b011 => AluOpImmKind::Sltiu,
                0b100 => AluOpImmKind::Xori,
                0b110 => AluOpImmKind::Ori,
                0b111 => AluOpImmKind::Andi,
                0b001 => {
                    if funct7 != 0b0000000 {
                        return Err(DecodeError::InvalidFunct7);
                    }
                    AluOpImmKind::Slli
                }
                0b101 => {
                    match funct7 {
                        0b0000000 => AluOpImmKind::Srli,
                        0b0100000 => AluOpImmKind::Srai,
                        _ => return Err(DecodeError::InvalidFunct7),
                    }
                }
                _ => return Err(DecodeError::InvalidFunct3),
            };
            Ok(DecodedInstruction::OpImm {
                kind,
                rd,
                rs1,
                imm,
            })
        }
        0b0110011 => {
            if funct7 == 0b0000001 {
                if !config.enable_m_extension {
                    return Err(DecodeError::ReservedInstruction);
                }
                let kind = match funct3 {
                    0b000 => MulDivKind::Mul,
                    0b001 => MulDivKind::Mulh,
                    0b010 => MulDivKind::Mulhsu,
                    0b011 => MulDivKind::Mulhu,
                    0b100 => MulDivKind::Div,
                    0b101 => MulDivKind::Divu,
                    0b110 => MulDivKind::Rem,
                    0b111 => MulDivKind::Remu,
                    _ => return Err(DecodeError::InvalidFunct3),
                };
                return Ok(DecodedInstruction::MulDiv {
                    kind,
                    rd,
                    rs1,
                    rs2,
                });
            }

            let kind = match (funct3, funct7) {
                (0b000, 0b0000000) => AluOpKind::Add,
                (0b000, 0b0100000) => AluOpKind::Sub,
                (0b001, 0b0000000) => AluOpKind::Sll,
                (0b010, 0b0000000) => AluOpKind::Slt,
                (0b011, 0b0000000) => AluOpKind::Sltu,
                (0b100, 0b0000000) => AluOpKind::Xor,
                (0b101, 0b0000000) => AluOpKind::Srl,
                (0b101, 0b0100000) => AluOpKind::Sra,
                (0b110, 0b0000000) => AluOpKind::Or,
                (0b111, 0b0000000) => AluOpKind::And,
                _ => return Err(DecodeError::InvalidEncoding),
            };
            Ok(DecodedInstruction::Op {
                kind,
                rd,
                rs1,
                rs2,
            })
        }
        0b00001111 => Ok(DecodedInstruction::Fence),
        0b1110011 => {
            if funct3 == 0 && get_bits(word, 20, 31) == 0 {
                Ok(DecodedInstruction::Ecall)
            } else if funct3 == 0 && get_bits(word, 20, 31) == 1 {
                Ok(DecodedInstruction::Ebreak)
            } else {
                Err(DecodeError::ReservedInstruction)
            }
        }
        _ => Err(DecodeError::InvalidOpcode),
    }
}
