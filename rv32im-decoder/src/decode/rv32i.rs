use crate::{bits::*, decoded::DecodedInstruction, error::DecodeError, format::InstructionFormat, instruction::Instruction, selectors::*};
fn r_type(word: u32, instruction: Instruction) -> DecodedInstruction { DecodedInstruction::new(word, instruction, InstructionFormat::R).with_rd(rd(word)).with_rs1(rs1(word)).with_rs2(rs2(word)) }
fn i_type(word: u32, instruction: Instruction, imm: i32) -> DecodedInstruction { DecodedInstruction::new(word, instruction, InstructionFormat::I).with_rd(rd(word)).with_rs1(rs1(word)).with_imm(imm) }
fn s_type(word: u32, instruction: Instruction, imm: i32) -> DecodedInstruction { DecodedInstruction::new(word, instruction, InstructionFormat::S).with_rs1(rs1(word)).with_rs2(rs2(word)).with_imm(imm) }
fn b_type(word: u32, instruction: Instruction, imm: i32) -> DecodedInstruction { DecodedInstruction::new(word, instruction, InstructionFormat::B).with_rs1(rs1(word)).with_rs2(rs2(word)).with_imm(imm) }
fn u_type(word: u32, instruction: Instruction, imm: i32) -> DecodedInstruction { DecodedInstruction::new(word, instruction, InstructionFormat::U).with_rd(rd(word)).with_imm(imm) }
fn j_type(word: u32, instruction: Instruction, imm: i32) -> DecodedInstruction { DecodedInstruction::new(word, instruction, InstructionFormat::J).with_rd(rd(word)).with_imm(imm) }
pub fn decode(word: u32) -> Result<DecodedInstruction, DecodeError> {
    match opcode(word) {
        0x37 => Ok(u_type(word, Instruction::Lui, u_imm(word))),
        0x17 => Ok(u_type(word, Instruction::Auipc, u_imm(word))),
        0x6f => Ok(j_type(word, Instruction::Jal, j_imm(word))),
        0x13 => match funct3(word) {
            0b000 => Ok(i_type(word, Instruction::Addi, i_imm(word))),
            _ => Err(DecodeError::UnsupportedOpcode(0x13)),
        },
        0x33 => match (funct3(word), funct7(word)) {
            (0, 0x00) => Ok(r_type(word, Instruction::Add)),
            (0, 0x20) => Ok(r_type(word, Instruction::Sub)),
            _ => Err(DecodeError::UnsupportedOpcode(0x33)),
        },
        _ => Err(DecodeError::UnsupportedOpcode(opcode(word))),
    }
}
