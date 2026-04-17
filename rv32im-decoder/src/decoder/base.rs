use crate::{
    encoding,
    error::ZkvmError,
    instruction::Instruction,
};

use super::{rd, rs1, rs2};

pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    match encoding::opcode(word) {
        encoding::OPCODE_LUI => Ok(Instruction::Lui {
            rd: rd(word)?,
            imm: encoding::imm_u(word),
        }),
        encoding::OPCODE_AUIPC => Ok(Instruction::Auipc {
            rd: rd(word)?,
            imm: encoding::imm_u(word),
        }),
        encoding::OPCODE_JAL => Ok(Instruction::Jal {
            rd: rd(word)?,
            imm: encoding::imm_j(word),
        }),
        encoding::OPCODE_JALR => decode_jalr(word),
        encoding::OPCODE_BRANCH => decode_branch(word),
        encoding::OPCODE_LOAD => decode_load(word),
        encoding::OPCODE_STORE => decode_store(word),
        encoding::OPCODE_OP_IMM => decode_op_imm(word),
        encoding::OPCODE_OP => decode_op(word),
        encoding::OPCODE_MISC_MEM => decode_misc_mem(word),
        encoding::OPCODE_SYSTEM => decode_system(word),
        opcode => Err(ZkvmError::UnsupportedOpcode(opcode)),
    }
}

fn decode_jalr(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = encoding::opcode(word);
    match encoding::funct3(word) {
        0b000 => Ok(Instruction::Jalr {
            rd: rd(word)?,
            rs1: rs1(word)?,
            imm: encoding::imm_i(word),
        }),
        funct3 => Err(ZkvmError::UnsupportedFunct3 { opcode, funct3 }),
    }
}

fn decode_branch(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = encoding::opcode(word);
    let rs1 = rs1(word)?;
    let rs2 = rs2(word)?;
    let imm = encoding::imm_b(word);

    match encoding::funct3(word) {
        0b000 => Ok(Instruction::Beq { rs1, rs2, imm }),
        0b001 => Ok(Instruction::Bne { rs1, rs2, imm }),
        0b100 => Ok(Instruction::Blt { rs1, rs2, imm }),
        0b101 => Ok(Instruction::Bge { rs1, rs2, imm }),
        0b110 => Ok(Instruction::Bltu { rs1, rs2, imm }),
        0b111 => Ok(Instruction::Bgeu { rs1, rs2, imm }),
        funct3 => Err(ZkvmError::UnsupportedFunct3 { opcode, funct3 }),
    }
}

fn decode_load(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = encoding::opcode(word);
    let rd = rd(word)?;
    let rs1 = rs1(word)?;
    let imm = encoding::imm_i(word);

    match encoding::funct3(word) {
        0b000 => Ok(Instruction::Lb { rd, rs1, imm }),
        0b001 => Ok(Instruction::Lh { rd, rs1, imm }),
        0b010 => Ok(Instruction::Lw { rd, rs1, imm }),
        0b100 => Ok(Instruction::Lbu { rd, rs1, imm }),
        0b101 => Ok(Instruction::Lhu { rd, rs1, imm }),
        funct3 => Err(ZkvmError::UnsupportedFunct3 { opcode, funct3 }),
    }
}

fn decode_store(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = encoding::opcode(word);
    let rs1 = rs1(word)?;
    let rs2 = rs2(word)?;
    let imm = encoding::imm_s(word);

    match encoding::funct3(word) {
        0b000 => Ok(Instruction::Sb { rs1, rs2, imm }),
        0b001 => Ok(Instruction::Sh { rs1, rs2, imm }),
        0b010 => Ok(Instruction::Sw { rs1, rs2, imm }),
        funct3 => Err(ZkvmError::UnsupportedFunct3 { opcode, funct3 }),
    }
}

fn decode_op_imm(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = encoding::opcode(word);
    let rd = rd(word)?;
    let rs1 = rs1(word)?;
    let imm = encoding::imm_i(word);
    let funct3 = encoding::funct3(word);

    match funct3 {
        0b000 => Ok(Instruction::Addi { rd, rs1, imm }),
        0b010 => Ok(Instruction::Slti { rd, rs1, imm }),
        0b011 => Ok(Instruction::Sltiu { rd, rs1, imm }),
        0b100 => Ok(Instruction::Xori { rd, rs1, imm }),
        0b110 => Ok(Instruction::Ori { rd, rs1, imm }),
        0b111 => Ok(Instruction::Andi { rd, rs1, imm }),
        0b001 => {
            let funct7 = encoding::funct7(word);
            if funct7 == 0 {
                Ok(Instruction::Slli {
                    rd,
                    rs1,
                    shamt: ((word >> 20) & 0x1f) as u8,
                })
            } else {
                Err(ZkvmError::UnsupportedShiftEncoding { funct3, funct7 })
            }
        }
        0b101 => {
            let funct7 = encoding::funct7(word);
            let shamt = ((word >> 20) & 0x1f) as u8;
            match funct7 {
                0 => Ok(Instruction::Srli { rd, rs1, shamt }),
                0x20 => Ok(Instruction::Srai { rd, rs1, shamt }),
                _ => Err(ZkvmError::UnsupportedShiftEncoding { funct3, funct7 }),
            }
        }
        _ => Err(ZkvmError::UnsupportedFunct3 { opcode, funct3 }),
    }
}

fn decode_op(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = encoding::opcode(word);
    let funct3 = encoding::funct3(word);
    let funct7 = encoding::funct7(word);
    let rd = rd(word)?;
    let rs1 = rs1(word)?;
    let rs2 = rs2(word)?;

    match (funct3, funct7) {
        (0, 0) => Ok(Instruction::Add { rd, rs1, rs2 }),
        (0, 0x20) => Ok(Instruction::Sub { rd, rs1, rs2 }),
        (1, 0) => Ok(Instruction::Sll { rd, rs1, rs2 }),
        (2, 0) => Ok(Instruction::Slt { rd, rs1, rs2 }),
        (3, 0) => Ok(Instruction::Sltu { rd, rs1, rs2 }),
        (4, 0) => Ok(Instruction::Xor { rd, rs1, rs2 }),
        (5, 0) => Ok(Instruction::Srl { rd, rs1, rs2 }),
        (5, 0x20) => Ok(Instruction::Sra { rd, rs1, rs2 }),
        (6, 0) => Ok(Instruction::Or { rd, rs1, rs2 }),
        (7, 0) => Ok(Instruction::And { rd, rs1, rs2 }),
        _ => Err(ZkvmError::UnsupportedFunct7 {
            opcode,
            funct3,
            funct7,
        }),
    }
}

fn decode_misc_mem(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = encoding::opcode(word);
    match encoding::funct3(word) {
        0b000 => Ok(Instruction::Fence {
            pred: ((word >> 24) & 0x0f) as u8,
            succ: ((word >> 20) & 0x0f) as u8,
        }),
        funct3 => Err(ZkvmError::UnsupportedFunct3 { opcode, funct3 }),
    }
}

fn decode_system(word: u32) -> Result<Instruction, ZkvmError> {
    if encoding::funct3(word) != 0 || encoding::rd(word) != 0 || encoding::rs1(word) != 0 {
        return Err(ZkvmError::UnsupportedSystem(word));
    }

    match word >> 20 {
        0 => Ok(Instruction::Ecall),
        1 => Ok(Instruction::Ebreak),
        _ => Err(ZkvmError::UnsupportedSystem(word)),
    }
}
