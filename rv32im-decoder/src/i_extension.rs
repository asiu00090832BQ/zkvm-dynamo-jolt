use crate::types::{DecodeError, Instruction};
use crate::util::{funct3, funct7, imm_b, imm_i, imm_j, imm_s, imm_u, rd, rs1, rs2, shamt};

pub fn decode_lui(word: u32) -> Result<Instruction, DecodeError> {
    Ok(Instruction::Lui {
        rd: rd(word),
        imm: imm_u(word),
    })
}

pub fn decode_auipc(word: u32) -> Result<Instruction, DecodeError> {
    Ok(Instruction::Auipc {
        rd: rd(word),
        imm: imm_u(word),
    })
}

pub fn decode_jal(word: u32) -> Result<Instruction, DecodeError> {
    Ok(Instruction::Jal {
        rd: rd(word),
        imm: imm_j(word),
    })
}

pub fn decode_jalr(word: u32) -> Result<Instruction, DecodeError> {
    match funct3(word) {
        0b000 => Ok(Instruction::Jalr {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        }),
        funct3 => Err(DecodeError::UnsupportedFunct3 {
            word,
            opcode: 0b1100111,
            funct3,
        }),
    }
}

pub fn decode_branch(word: u32) -> Result<Instruction, DecodeError> {
    let decoded = match funct3(word) {
        0b000 => Instruction::Beq {
            rs1: rs1(word),
            rs2: rs2(word),
            imm: imm_b(word),
        },
        0b001 => Instruction::Bne {
            rs1: rs1(word),
            rs2: rs2(word),
            imm: imm_b(word),
        },
        0b100 => Instruction::Blt {
            rs1: rs1(word),
            rs2: rs2(word),
            imm: imm_b(word),
        },
        0b101 => Instruction::Bge {
            rs1: rs1(word),
            rs2: rs2(word),
            imm: imm_b(word),
        },
        0b110 => Instruction::Bltu {
            rs1: rs1(word),
            rs2: rs2(word),
            imm: imm_b(word),
        },
        0b111 => Instruction::Bgeu {
            rs1: rs1(word),
            rs2: rs2(word),
            imm: imm_b(word),
        },
        funct3 => {
            return Err(DecodeError::UnsupportedFunct3 {
                word,
                opcode: 0b1100011,
                funct3,
            })
        }
    };
    Ok(decoded)
}

pub fn decode_load(word: u32) -> Result<Instruction, DecodeError> {
    let decoded = match funct3(word) {
        0b000 => Instruction::Lb {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        },
        0b001 => Instruction::Lh {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        },
        0b010 => Instruction::Lw {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        },
        0b100 => Instruction::Lbu {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        },
        0b101 => Instruction::Lhu {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        },
        funct3 => {
            return Err(DecodeError::UnsupportedFunct3 {
                word,
                opcode: 0b0000011,
                funct3,
            })
        }
    };
    Ok(decoded)
}

pub fn decode_store(word: u32) -> Result<Instruction, DecodeError> {
    let decoded = match funct3(word) {
        0b000 => Instruction::Sb {
            rs1: rs1(word),
            rs2: rs2(word),
            imm: imm_s(word),
        },
        0b001 => Instruction::Sh {
            rs1: rs1(word),
            rs2: rs2(word),
            imm: imm_s(word),
        },
        0b010 => Instruction::Sw {
            rs1: rs1(word),
            rs2: rs2(word),
            imm: imm_s(word),
        },
        funct3 => {
            return Err(DecodeError::UnsupportedFunct3 {
                word,
                opcode: 0b0100011,
                funct3,
            })
        }
    };
    Ok(decoded)
}

pub fn decode_op_imm(word: u32) -> Result<Instruction, DecodeError> {
    let decoded = match funct3(word) {
        0b000 => Instruction::Addi {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        },
        0b010 => Instruction::Slti {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        },
        0b011 => Instruction::Sltiu {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        },
        0b100 => Instruction::Xori {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        },
        0b110 => Instruction::Ori {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        },
        0b111 => Instruction::Andi {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        },
        0b001 => match funct7(word) {
            0b0000000 => Instruction::Slli {
                rd: rd(word),
                rs1: rs1(word),
                shamt: shamt(word),
            },
            funct7 => {
                return Err(DecodeError::InvalidShiftEncoding {
                    word,
                    funct3: 0b001,
                    funct7,
                })
            }
        },
        0b101 => match funct7(word) {
            0b0000000 => Instruction::Srli {
                rd: rd(word),
                rs1: rs1(word),
                shamt: shamt(word),
            },
            0b0100000 => Instruction::Srai {
                rd: rd(word),
                rs1: rs1(word),
                shamt: shamt(word),
            },
            funct7 => {
                return Err(DecodeError::InvalidShiftEncoding {
                    word,
                    funct3: 0b101,
                    funct7,
                })
            }
        },
        funct3 => {
            return Err(DecodeError::UnsupportedFunct3 {
                word,
                opcode: 0b0010011,
                funct3,
            })
        }
    };
    Ok(decoded)
}

pub fn decode_op(word: u32) -> Result<Instruction, DecodeError> {
    let decoded = match (funct3(word), funct7(word)) {
        (0b000, 0b0000000) => Instruction::Add {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        (0b000, 0b0100000) => Instruction::Sub {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        (0b001, 0b0000000) => Instruction::Sll {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        (0b010, 0b0000000) => Instruction::Slt {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        (0b011, 0b0000000) => Instruction::Sltu {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        (0b100, 0b0000000) => Instruction::Xor {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        (0b101, 0b0000000) => Instruction::Srl {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        (0b101, 0b0100000) => Instruction::Sra {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        (0b110, 0b0000000) => Instruction::Or {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        (0b111, 0b0000000) => Instruction::And {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        (funct3, funct7) => {
            return Err(DecodeError::UnsupportedFunct7 {
                word,
                funct3,
                funct7,
            })
        }
    };
    Ok(decoded)
}

pub fn decode_fence(word: u32) -> Result<Instruction, DecodeError> {
    let rd_bits = rd(word);
    let rs1_bits = rs1(word);

    match funct3(word) {
        0b000 if rd_bits == 0 && rs1_bits == 0 => Ok(Instruction::Fence),
        0b001 if rd_bits == 0 && rs1_bits == 0 && ((word >> 20) & 0x0fff) == 0 => {
            Ok(Instruction::FenceI)
        }
        _ => Err(DecodeError::InvalidFenceEncoding { word }),
    }
}

pub fn decode_system(word: u32) -> Result<Instruction, DecodeError> {
    if funct3(word) != 0 || rd(word) != 0 || rs1(word) != 0 {
        return Err(DecodeError::InvalidSystemEncoding { word });
    }

    match (word >> 20) & 0x0fff {
        0 => Ok(Instruction::Ecall),
        1 => Ok(Instruction::Ebreak),
        _ => Err(DecodeError::InvalidSystemEncoding { word }),
    }
}
