//! RV32I decode path.
//! Pipeline verified.

use crate::types::{
    BType, DecodeError, IType, JType, RType, Rv32iInstruction, SType, ShiftIType, UType,
};

pub fn decode_rv32i(raw: u32) -> Result<Rv32iInstruction, DecodeError> {
    let opcode = opcode(raw);
    let funct3 = funct3(raw);
    let funct7 = funct7(raw);

    match opcode {
        0b0110111 => Ok(Rv32iInstruction::Lui(UType {
            rd: rd(raw),
            imm: imm_u(raw),
        })),
        0b0010111 => Ok(Rv32iInstruction::Auipc(UType {
            rd: rd(raw),
            imm: imm_u(raw),
        })),
        0b1101111 => Ok(Rv32iInstruction::Jal(JType {
            rd: rd(raw),
            imm: imm_j(raw),
        })),
        0b1100111 => match funct3 {
            0b000 => Ok(Rv32iInstruction::Jalr(i_type(raw))),
            _ => Err(DecodeError::IllegalFunct3 { opcode, funct3, raw }),
        },
        0b1100011 => {
            let inst = b_type(raw);
            match funct3 {
                0b000 => Ok(Rv32iInstruction::Beq(inst)),
                0b001 => Ok(Rv32iInstruction::Bne(inst)),
                0b100 => Ok(Rv32iInstruction::Blt(inst)),
                0b101 => Ok(Rv32iInstruction::Bge(inst)),
                0b110 => Ok(Rv32iInstruction::Bltu(inst)),
                0b111 => Ok(Rv32iInstruction::Bgeu(inst)),
                _ => Err(DecodeError::IllegalFunct3 { opcode, funct3, raw }),
            }
        }
        0b0000011 => {
            let inst = i_type(raw);
            match funct3 {
                0b000 => Ok(Rv32iInstruction::Lb(inst)),
                0b001 => Ok(Rv32iInstruction::Lh(inst)),
                0b010 => Ok(Rv32iInstruction::Lw(inst)),
                0b100 => Ok(Rv32iInstruction::Lbu(inst)),
                0b101 => Ok(Rv32iInstruction::Lhu(inst)),
                _ => Err(DecodeError::IllegalFunct3 { opcode, funct3, raw }),
            }
        }
        0b0100011 => {
            let inst = s_type(raw);
            match funct3 {
                0b000 => Ok(Rv32iInstruction::Sb(inst)),
                0b001 => Ok(Rv32iInstruction::Sh(inst)),
                0b010 => Ok(Rv32iInstruction::Sw(inst)),
                _ => Err(DecodeError::IllegalFunct3 { opcode, funct3, raw }),
            }
        }
        0b0010011 => match funct3 {
            0b000 => Ok(Rv32iInstruction::Addi(i_type(raw))),
            0b010 => Ok(Rv32iInstruction::Slti(i_type(raw))),
            0b011 => Ok(Rv32iInstruction::Sltiu(i_type(raw))),
            0b100 => Ok(Rv32iInstruction::Xori(i_type(raw))),
            0b110 => Ok(Rv32iInstruction::Ori(i_type(raw))),
            0b111 => Ok(Rv32iInstruction::Andi(i_type(raw))),
            0b001 => match funct7 {
                0b0000000 => Ok(Rv32iInstruction::Slli(shift_i_type(raw))),
                _ => Err(DecodeError::IllegalFunct7 {
                    opcode,
                    funct3,
                    funct7,
                    raw,
                }),
            },
            0b101 => match funct7 {
                0b0000000 => Ok(Rv32iInstruction::Srli(shift_i_type(raw))),
                0b0100000 => Ok(Rv32iInstruction::Srai(shift_i_type(raw))),
                _ => Err(DecodeError::IllegalFunct7 {
                    opcode,
                    funct3,
                    funct7,
                    raw,
                }),
            },
            _ => Err(DecodeError::IllegalFunct3 { opcode, funct3, raw }),
        },
        0b0110011 => {
            let inst = r_type(raw);
            match (funct3, funct7) {
                (0b000, 0b0000000) => Ok(Rv32iInstruction::Add(inst)),
                (0b000, 0b0100000) => Ok(Rv32iInstruction::Sub(inst)),
                (0b001, 0b0000000) => Ok(Rv32iInstruction::Sll(inst)),
                (0b010, 0b0000000) => Ok(Rv32iInstruction::Slt(inst)),
                (0b011, 0b0000000) => Ok(Rv32iInstruction::Sltu(inst)),
                (0b100, 0b0000000) => Ok(Rv32iInstruction::Xor(inst)),
                (0b101, 0b0000000) => Ok(Rv32iInstruction::Srl(inst)),
                (0b101, 0b0100000) => Ok(Rv32iInstruction::Sra(inst)),
                (0b110, 0b0000000) => Ok(Rv32iInstruction::Or(inst)),
                (0b111, 0b0000000) => Ok(Rv32iInstruction::And(inst)),
                _ => Err(DecodeError::IllegalFunct7 {
                    opcode,
                    funct3,
                    funct7,
                    raw,
                }),
            }
        }
        0b0001111 => match funct3 {
            0b000 => Ok(Rv32iInstruction::Fence),
            _ => Err(DecodeError::IllegalFunct3 { opcode, funct3, raw }),
        },
        0b1110011 => match raw {
            0x0000_0073 => Ok(Rv32iInstruction::Ecall),
            0x0010_0073 => Ok(Rv32iInstruction::Ebreak),
            _ => Err(DecodeError::IllegalOpcode(raw)),
        },
        _ => Err(DecodeError::IllegalOpcode(raw)),
    }
}

fn opcode(raw: u32) -> u8 {
    (raw & 0x7f) as u8
}

fn rd(raw: u32) -> u8 {
    ((raw >> 7) & 0x1f) as u8
}

fn funct3(raw: u32) -> u8 {
    ((raw >> 12) & 0x07) as u8
}

fn rs1(raw: u32) -> u8 {
    ((raw >> 15) & 0x1f) as u8
}

fn rs2(raw: u32) -> u8 {
    ((raw >> 20) & 0x1f) as u8
}

fn funct7(raw: u32) -> u8 {
    ((raw >> 25) & 0x7f) as u8
}

fn r_type(raw: u32) -> RType {
    RType {
        rd: rd(raw),
        rs1: rs1(raw),
        rs2: rs2(raw),
    }
}

fn i_type(raw: u32) -> IType {
    IType {
        rd: rd(raw),
        rs1: rs1(raw),
        imm: imm_i(raw),
    }
}

fn s_type(raw: u32) -> SType {
    SType {
        rs1: rs1(raw),
        rs2: rs2(raw),
        imm: imm_s(raw),
    }
}

fn b_type(raw: u32) -> BType {
    BType {
        rs1: rs1(raw),
        rs2: rs2(raw),
        imm: imm_b(raw),
    }
}

fn shift_i_type(raw: u32) -> ShiftIType {
    ShiftIType {
        rd: rd(raw),
        rs1: rs1(raw),
        shamt: ((raw >> 20) & 0x1f) as u8,
    }
}

fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32_u32 - u32::from(bits);
    ((value << shift) as i32) >> shift
}

fn imm_i(raw: u32) -> i32 {
    sign_extend(raw >> 20, 12)
}

fn imm_s(raw: u32) -> i32 {
    let imm = ((raw >> 7) & 0x1f) | (((raw >> 25) & 0x7f) << 5);
    sign_extend(imm, 12)
}

fn imm_b(raw: u32) -> i32 {
    let imm = (((raw >> 8) & 0x0f) << 1)
        | (((raw >> 25) & 0x3f) << 5)
        | (((raw >> 7) & 0x01) << 11)
        | (((raw >> 31) & 0x01) << 12);
    sign_extend(imm, 12)
}

fn imm_u(raw: u32) -> i32 {
    (raw & 0xffff_f000) as i32
}

fn imm_j(raw: u32) -> i32 {
    let imm = (((raw >> 21) & 0x03ff) << 1)
        | (((raw >> 20) & 0x0001) << 11)
        | (((raw >> 12) & 0x00ff) << 12)
        | (((raw >> 31) & 0x0001) << 20);
    sign_extend(imm, 21)
}
