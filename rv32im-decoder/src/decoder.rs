use crate::error::{Result, ZkvmError};
use crate::instruction::*;

fn opcode(word: u32) -> u8 {
    (word & 0x7f) as u8
}

fn rd(word: u32) -> Reg {
    ((word >> 7) & 0x1f) as u8
}

fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x7) as u8
}

fn rs1(word: u32) -> Reg {
    ((word >> 15) & 0x1f) as u8
}

fn rs2(word: u32) -> Reg {
    ((word >> 20) & 0x1f) as u8
}

fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7f) as u8
}

fn imm_i(word: u32) -> i32 {
    (word as i32) >> 20
}

fn imm_s(word: u32) -> i32 {
    let imm4_0 = (word >> 7) & 0x1f;
    let imm11_5 = (word >> 25) & 0x7f;
    let imm = (imm11_5 << 5) | imm4_0;
    ((imm as i32) << 20) >> 20
}

fn imm_b(word: u32) -> i32 {
    let imm11 = (word >> 7) & 0x1;
    let imm4_1 = (word >> 8) & 0xf;
    let imm10_5 = (word >> 25) & 0x3f;
    let imm12 = (word >> 31) & 0x1;
    let imm = (imm12 << 12) | (imm11 << 11) | (imm10_5 << 5) | (imm4_1 << 1);
    ((imm as i32) << 19) >> 19
}

fn imm_u(word: u32) -> i32 {
    ((word & 0xfffff000) as i32)
}

fn imm_j(word: u32) -> i32 {
    let imm20 = (word >> 31) & 0x1;
    let imm10_1 = (word >> 21) & 0x3ff;
    let imm11 = (word >> 20) & 0x1;
    let imm19_12 = (word >> 12) & 0xff;
    let imm = (imm20 << 20) | (imm19_12 << 12) | (imm11 << 11) | (imm10_1 << 1);
    ((imm as i32) << 11) >> 11
}

pub fn decode(word: u32) -> Result<Instruction> {
    let opc = opcode(word);
    match opc {
        0b0110111 => Ok(Instruction::Lui { rd: rd(word), imm: imm_u(word) }),
        0b0010111 => Ok(Instruction::Auipc { rd: rd(word), imm: imm_u(word) }),
        0b1101111 => Ok(Instruction::Jal { rd: rd(word), imm: imm_j(word) }),
        0b1100111 => Ok(Instruction::Jalr { rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
        0b0000011 => Ok(Instruction::Load { funct3: funct3(word), rd: rd(word), rs1: rs1(word), imm: imm_i(word) }),
        0b0100011 => Ok(Instruction::Store { funct3: funct3(word), rs1: rs1(word), rs2: rs2(word), imm: imm_s(word) }),
        0b1100011 => Ok(Instruction::Branch { funct3: funct3(word), rs1: rs1(word), rs2: rs2(word), imm: imm_b(word) }),
        0b0110011 => decode_rtype(word),
        0b0001111 => Ok(Instruction::Fence),
        0b1110011 => decode_system(word),
        _ => Err(ZkvmError::InvalidOpcode(word)),
    }
}

fn decode_rtype(word: u32) -> Result<Instruction> {
    let f3 = funct3(word);
    let f7 = funct7(word);
    let rd_ = rd(word);
    let rs1_ = rs1(word);
    let rs2_ = rs2(word);

    match (f7, f3) {
        (0b0000000, 0b000) => Ok(Instruction::Add { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0100000, 0b000) => Ok(Instruction::Sub { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000000, 0b001) => Ok(Instruction::Sll { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000000, 0b010) => Ok(Instruction::Slt { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000000, 0b011) => Ok(Instruction::Sltu { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000000, 0b100) => Ok(Instruction::Xor { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000000, 0b101) => Ok(Instruction::Srl { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0100000, 0b101) => Ok(Instruction::Sra { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000000, 0b110) => Ok(Instruction::Or { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000000, 0b111) => Ok(Instruction::And { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000001, 0b000) => Ok(Instruction::Mul { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000001, 0b001) => Ok(Instruction::Mulh { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000001, 0b010) => Ok(Instruction::Mulhsu { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000001, 0b011) => Ok(Instruction::Mulhu { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000001, 0b100) => Ok(Instruction::Div { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000001, 0b101) => Ok(Instruction::Divu { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000001, 0b110) => Ok(Instruction::Rem { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        (0b0000001, 0b111) => Ok(Instruction::Remu { rd: rd_, rs1: rs1_, rs2: rs2_ }),
        _ => Err(ZkvmError::InvalidFunct7 { opcode: opcode(word), funct3: f3, funct7: f7 }),
    }
}

fn decode_system(word: u32) -> Result<Instruction> {
    let f3 = funct3(word);
    let imm = (word >> 20) & 0xfff;

    match (f3, imm) {
        (0b000, 0) => Ok(Instruction::Ecall),
        (0b000, 1) => Ok(Instruction::Ebreak),
        _ => Err(ZkvmError::UnsupportedInstruction(word)),
    }
}
