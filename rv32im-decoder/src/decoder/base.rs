use crate::error::ZkvmError;
use crate::instruction::Instruction;

use super::m_extension::decode_m_extension;

pub fn decode_base(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = opcode(word);

    match opcode {
        0b0110111 => Ok(Instruction::Lui {
            rd: rd(word),
            imm: imm_u(word),
        }),
        0b0010111 => Ok(Instruction::Auipc {
            rd: rd(word),
            imm: imm_u(word),
        }),
        0b1101111 => Ok(Instruction::Jal {
            rd: rd(word),
            imm: imm_j(word),
        }),
        0b1100111 => {
            let funct3 = funct3(word);
            if funct3 != 0b000 {
                return Err(ZkvmError::InvalidFunct3 { opcode, funct3, word });
            }

            Ok(Instruction::Jalr {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            })
        }
        0b1100011 => decode_branch(word),
        0b0000011 => decode_load(word),
        0b0100011 => decode_store(word),
        0b0010011 => decode_op_imm(word),
        0b0110011 => decode_op(word),
        0b0001111 => decode_fence(word),
        0b1110011 => decode_system(word),
        _ => Err(ZkvmError::InvalidOpcode { word }),
    }
}

fn decode_branch(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = opcode(word);
    let rs1 = rs1(word);
    let rs2 = rs2(word);
    let imm = imm_b(word);

    match funct3(word) {
        0b000 => Ok(Instruction::Beq { rs1, rs2, imm }),
        0b001 => Ok(Instruction::Bne { rs1, rs2, imm }),
        0b100 => Ok(Instruction::Blt { rs1, rs2, imm }),
        0b101 => Ok(Instruction::Bge { rs1, rs2, imm }),
        0b110 => Ok(Instruction::Bltu { rs1, rs2, imm }),
        0b111 => Ok(Instruction::Bgeu { rs1, rs2, imm }),
        funct3 => Err(ZkvmError::InvalidFunct3 { opcode, funct3, word }),
    }
}

fn decode_load(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = opcode(word);
    let rd = rd(word);
    let rs1 = rs1(word);
    let imm = imm_i(word);

    match funct3(word) {
        0b000 => Ok(Instruction::Lb { rd, rs1, imm }),
        0b001 => Ok(Instruction::Lh { rd, rs1, imm }),
        0b010 => Ok(Instruction::Lw { rd, rs1, imm }),
        0b100 => Ok(Instruction::Lbu { rd, rs1, imm }),
        0b101 => Ok(Instruction::Lhu { rd, rs1, imm }),
        funct3 => Err(ZkvmError::InvalidFunct3 { opcode, funct3, word }),
    }
}

fn decode_store(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = opcode(word);
    let rs1 = rs1(word);
    let rs2 = rs2(word);
    let imm = imm_s(word);

    match funct3(word) {
        0b000 => Ok(Instruction::Sb { rs1, rs2, imm }),
        0b001 => Ok(Instruction::Sh { rs1, rs2, imm }),
        0b010 => Ok(Instruction::Sw { rs1, rs2, imm }),
        funct3 => Err(ZkvmError::InvalidFunct3 { opcode, funct3, word }),
    }
}

fn decode_op_imm(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = opcode(word);
    let rd = rd(word);
    let rs1 = rs1(word);
    let funct3 = funct3(word);

    match funct3 {
        0b000 => Ok(Instruction::Addi {
            rd,
            rs1,
            imm: imm_i(word),
        }),
        0b010 => Ok(Instruction::Slti {
            rd,
            rs1,
            imm: imm_i(word),
        }),
        0b011 => Ok(Instruction::Sltiu {
            rd,
            rs1,
            imm: imm_i(word),
        }),
        0b100 => Ok(Instruction::Xori {
            rd,
            rs1,
            imm: imm_i(word),
        }),
        0b110 => Ok(Instruction::Ori {
            rd,
            rs1,
            imm: imm_i(word),
        }),
        0b111 => Ok(Instruction::Andi {
            rd,
            rs1,
            imm: imm_i(word),
        }),
        0b001 => {
            let funct7 = funct7(word);
            if funct7 != 0b0000000 {
                return Err(ZkvmError::InvalidFunct7 {
                    opcode,
                    funct3,
                    funct7,
                    word,
                });
            }

            Ok(Instruction::Slli {
                rd,
                rs1,
                imm: ((word >> 20) & 0x1f) as i32,
            })
        }
        0b101 => {
            let funct7 = funct7(word);
            match funct7 {
                0b0000000 => Ok(Instruction::Srli {
                    rd,
                    rs1,
                    imm: ((word >> 20) & 0x1f) as i32,
                }),
                0b0100000 => Ok(Instruction::Srai {
                    rd,
                    rs1,
                    imm: ((word >> 20) & 0x1f) as i32,
                }),
                _ => Err(ZkvmError::InvalidFunct7 {
                    opcode,
                    funct3,
                    funct7,
                    word,
                }),
            }
        }
        _ => Err(ZkvmError::InvalidFunct3 { opcode, funct3, word }),
    }
}

fn decode_op(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = opcode(word);
    let rd = rd(word);
    let rs1 = rs1(word);
    let rs2 = rs2(word);
    let funct3 = funct3(word);
    let funct7 = funct7(word);

    if funct7 == 0b0000001 {
        return decode_m_extension(word);
    }

    match (funct3, funct7) {
        (0b000, 0b0000000) => Ok(Instruction::Add { rd, rs1, rs2 }),
        (0b000, 0b0100000) => Ok(Instruction::Sub { rd, rs1, rs2 }),
        (0b001, 0b0000000) => Ok(Instruction::Sll { rd, rs1, rs2 }),
        (0b010, 0b0000000) => Ok(Instruction::Slt { rd, rs1, rs2 }),
        (0b011, 0b0000000) => Ok(Instruction::Sltu { rd, rs1, rs2 }),
        (0b100, 0b0000000) => Ok(Instruction::Xor { rd, rs1, rs2 }),
        (0b101, 0b0000000) => Ok(Instruction::Srl { rd, rs1, rs2 }),
        (0b101, 0b0100000) => Ok(Instruction::Sra { rd, rs1, rs2 }),
        (0b110, 0b0000000) => Ok(Instruction::Or { rd, rs1, rs2 }),
        (0b111, 0b0000000) => Ok(Instruction::And { rd, rs1, rs2 }),
        _ => Err(ZkvmError::InvalidFunct7 {
            opcode,
            funct3,
            funct7,
            word,
        }),
    }
}

fn decode_fence(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = opcode(word);
    let funct3 = funct3(word);

    match funct3 {
        0b000 => Ok(Instruction::Fence),
        _ => Err(ZkvmError::InvalidFunct3 { opcode, funct3, word }),
    }
}

fn decode_system(word: u32) -> Result<Instruction, ZkvmError> {
    match word {
        0x0000_0073 => Ok(Instruction::Ecall),
        0x0010_0073 => Ok(Instruction::Ebreak),
        _ => Err(ZkvmError::DecodeError {
            message: "unsupported system instruction",
            word,
        }),
    }
}

fn opcode(word: u32) -> u8 {
    (word & 0x7f) as u8
}

fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1f) as u8
}

fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x07) as u8
}

fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}

fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7f) as u8
}

fn imm_i(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

fn imm_s(word: u32) -> i32 {
    let value = ((word >> 25) << 5) | ((word >> 7) & 0x1f);
    sign_extend(value, 12)
}

fn imm_b(word: u32) -> i32 {
    let value = (((word >> 31) & 0x1) << 12)
        | (((word >> 7) & 0x1) << 11)
        | (((word >> 25) & 0x3f) << 5)
        | (((word >> 8) & 0x0f) << 1);
    sign_extend(value, 13)
}

fn imm_u(word: u32) -> i32 {
    (word & 0xffff_f000) as i32
}

fn imm_j(word: u32) -> i32 {
    let value = (((word >> 31) & 0x1) << 20)
        | (((word >> 12) & 0xff) << 12)
        | (((word >> 20) & 0x1) << 11)
        | (((word >> 21) & 0x3ff) << 1);
    sign_extend(value, 21)
}

fn sign_extend(value: u32, bits: u32) -> i32 {
    let shift = 32 - bits;
    ((value << shift) as i32) >> shift
}
