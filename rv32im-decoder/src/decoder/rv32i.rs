use crate::error::DecodeError;
use crate::fields::{
    decode_b_type, decode_i_type, decode_j_type, decode_r_type, decode_s_type, decode_u_type,
    funct3, funct7, opcode,
};
use crate::types::{Instruction, Register};

pub fn decode_rv32i(word: u32) -> Result<Instruction, DecodeError> {
    match opcode(word) {
        0b0110111 => {
            let u = decode_u_type(word)?;
            Ok(Instruction::Lui { rd: u.rd, imm: u.imm })
        }
        0b0010111 => {
            let u = decode_u_type(word)?;
            Ok(Instruction::Auipc { rd: u.rd, imm: u.imm })
        }
        0b1101111 => {
            let j = decode_j_type(word)?;
            Ok(Instruction::Jal { rd: j.rd, imm: j.imm })
        }
        0b1100111 => {
            let i = decode_i_type(word)?;
            match i.funct3 {
                0b000 => Ok(Instruction::Jalr {
                    rd: i.rd,
                    rs1: i.rs1,
                    imm: i.imm,
                }),
                other => Err(DecodeError::InvalidFunct3 {
                    opcode: opcode(word),
                    funct3: other,
                }),
            }
        }
        0b1100011 => {
            let b = decode_b_type(word)?;
            match b.funct3 {
                0b000 => Ok(Instruction::Beq {
                    rs1: b.rs1,
                    rs2: b.rs2,
                    imm: b.imm,
                }),
                0b001 => Ok(Instruction::Bne {
                    rs1: b.rs1,
                    rs2: b.rs2,
                    imm: b.imm,
                }),
                0b100 => Ok(Instruction::Blt {
                    rs1: b.rs1,
                    rs2: b.rs2,
                    imm: b.imm,
                }),
                0b101 => Ok(Instruction::Bge {
                    rs1: b.rs1,
                    rs2: b.rs2,
                    imm: b.imm,
                }),
                0b110 => Ok(Instruction::Bltu {
                    rs1: b.rs1,
                    rs2: b.rs2,
                    imm: b.imm,
                }),
                0b111 => Ok(Instruction::Bgeu {
                    rs1: b.rs1,
                    rs2: b.rs2,
                    imm: b.imm,
                }),
                other => Err(DecodeError::InvalidFunct3 {
                    opcode: opcode(word),
                    funct3: other,
                }),
            }
        }
        0b0000011 => {
            let i = decode_i_type(word)?;
            match i.funct3 {
                0b000 => Ok(Instruction::Lb {
                    rd: i.rd,
                    rs1: i.rs1,
                    imm: i.imm,
                }),
                0b001 => Ok(Instruction::Lh {
                    rd: i.rd,
                    rs1: i.rs1,
                    imm: i.imm,
                }),
                0b010 => Ok(Instruction::Lw {
                    rd: i.rd,
                    rs1: i.rs1,
                    imm: i.imm,
                }),
                0b100 => Ok(Instruction::Lbu {
                    rd: i.rd,
                    rs1: i.rs1,
                    imm: i.imm,
                }),
                0b101 => Ok(Instruction::Lhu {
                    rd: i.rd,
                    rs1: i.rs1,
                    imm: i.imm,
                }),
                other => Err(DecodeError::InvalidFunct3 {
                    opcode: opcode(word),
                    funct3: other,
                }),
            }
        }
        0b0100011 => {
            let s = decode_s_type(word)?;
            match s.funct3 {
                0b000 => Ok(Instruction::Sb {
                    rs1: s.rs1,
                    rs2: s.rs2,
                    imm: s.imm,
                }),
                0b001 => Ok(Instruction::Sh {
                    rs1: s.rs1,
                    rs2: s.rs2,
                    imm: s.imm,
                }),
                0b010 => Ok(Instruction::Sw {
                    rs1: s.rs1,
                    rs2: s.rs2,
                    imm: s.imm,
                }),
                other => Err(DecodeError::InvalidFunct3 {
                    opcode: opcode(word),
                    funct3: other,
                }),
            }
        }
        0b0010011 => {
            let i = decode_i_type(word)?;
            match i.funct3 {
                0b000 => Ok(Instruction::Addi {
                    rd: i.rd,
                    rs1: i.rs1,
                    imm: i.imm,
                }),
                0b010 => Ok(Instruction::Slti {
                    rd: i.rd,
                    rs1: i.rs1,
                    imm: i.imm,
                }),
                0b011 => Ok(Instruction::Sltiu {
                    rd: i.rd,
                    rs1: i.rs1,
                    imm: i.imm,
                }),
                0b100 => Ok(Instruction::Xori {
                    rd: i.rd,
                    rs1: i.rs1,
                    imm: i.imm,
                }),
                0b110 => Ok(Instruction::Ori {
                    rd: i.rd,
                    rs1: i.rs1,
                    imm: i.imm,
                }),
                0b111 => Ok(Instruction::Andi {
                    rd: i.rd,
                    rs1: i.rs1,
                    imm: i.imm,
                }),
                0b001 => match funct7(word) {
                    0b0000000 => Ok(Instruction::Slli {
                        rd: i.rd,
                        rs1: i.rs1,
                        shamt: ((word >> 20) & 0x1f) as u8,
                    }),
                    other => Err(DecodeError::InvalidFunct7 {
                        opcode: opcode(word),
                        funct7: other,
                    }),
                },
                0b101 => match funct7(word) {
                    0b0000000 => Ok(Instruction::Srli {
                        rd: i.rd,
                        rs1: i.rs1,
                        shamt: ((word >> 20) & 0x1f) as u8,
                    }),
                    0b0100000 => Ok(Instruction::Srai {
                        rd: i.rd,
                        rs1: i.rs1,
                        shamt: ((word >> 20) & 0x1f) as u8,
                    }),
                    other => Err(DecodeError::InvalidFunct7 {
                        opcode: opcode(word),
                        funct7: other,
                    }),
                },
                other => Err(DecodeError::InvalidFunct3 {
                    opcode: opcode(word),
                    funct3: other,
                }),
            }
        }
        0b0110011 => {
            let r = decode_r_type(word)?;
            match (r.funct3, r.funct7) {
                (0b000, 0b0000000) => Ok(Instruction::Add {
                    rd: r.rd,
                    rs1: r.rs1,
                    rs2: r.rs2,
                }),
                (0b000, 0b0100000) => Ok(Instruction::Sub {
                    rd: r.rd,
                    rs1: r.rs1,
                    rs2: r.rs2,
                }),
                (0b001, 0b0000000) => Ok(Instruction::Sll {
                    rd: r.rd,
                    rs1: r.rs1,
                    rs2: r.rs2,
                }),
                (0b010, 0b0000000) => Ok(Instruction::Slt {
                    rd: r.rd,
                    rs1: r.rs1,
                    rs2: r.rs2,
                }),
                (0b011, 0b0000000) => Ok(Instruction::Sltu {
                    rd: r.rd,
                    rs1: r.rs1,
                    rs2: r.rs2,
                }),
                (0b100, 0b0000000) => Ok(Instruction::Xor {
                    rd: r.rd,
                    rs1: r.rs1,
                    rs2: r.rs2,
                }),
                (0b101, 0b0000000) => Ok(Instruction::Srl {
                    rd: r.rd,
                    rs1: r.rs1,
                    rs2: r.rs2,
                }),
                (0b101, 0b0100000) => Ok(Instruction::Sra {
                    rd: r.rd,
                    rs1: r.rs1,
                    rs2: r.rs2,
                }),
                (0b110, 0b0000000) => Ok(Instruction::Or {
                    rd: r.rd,
                    rs1: r.rs1,
                    rs2: r.rs2,
                }),
                (0b111, 0b0000000) => Ok(Instruction::And {
                    rd: r.rd,
                    rs1: r.rs1,
                    rs2: r.rs2,
                }),
                (_, 0b0000001) => Err(DecodeError::UnsupportedInstruction(word)),
                (_, other) => Err(DecodeError::InvalidFunct7 {
                    opcode: opcode(word),
                    funct7: other,
                }),
            }
        }
        0b0001111 => match funct3(word) {
            0b000 => Ok(Instruction::Fence {
                fm: ((word >> 28) & 0x0f) as u8,
                pred: ((word >> 24) & 0x0f) as u8,
                succ: ((word >> 20) & 0x0f) as u8,
            }),
            other => Err(DecodeError::InvalidFunct3 {
                opcode: opcode(word),
                funct3: other,
            }),
        },
        0b1110011 => {
            let i = decode_i_type(word)?;
            if i.funct3 != 0 {
                return Err(DecodeError::InvalidFunct3 {
                    opcode: opcode(word),
                    funct3: i.funct3,
                });
            }

            if i.rs1 != Register::X0 || i.rd != Register::X0 {
                return Err(DecodeError::UnsupportedInstruction(word));
            }

            match ((word >> 20) & 0x0fff) as u16 {
                0 => Ok(Instruction::Ecall),
                1 => Ok(Instruction::Ebreak),
                _ => Err(DecodeError::UnsupportedInstruction(word)),
            }
        }
        other => Err(DecodeError::InvalidOpcode(other)),
    }
}
