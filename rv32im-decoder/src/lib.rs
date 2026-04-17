pub mod types;
pub mod util;

pub use crate::types::{Instruction, DecodeError, DecodeSelectors};

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = util::opcode(word);
    let rd = util::rd(word);
    let rs1 = util::rs1(word);
    let rs2 = util::rs2(word);
    let f3 = util::funct3(word);
    let f7 = util::funct7(word);

    match opcode {
        0x33 => { // R-type
            match (f7, f3) {
                (0x00, 0x0) => Ok(Instruction::Add { rd, rs1, rs2 }),
                (0x20, 0x0) => Ok(Instruction::Sub { rd, rs1, rs2 }),
                (0x00, 0x1) => Ok(Instruction::Sll { rd, rs1, rs2 }),
                (0x00, 0x2) => Ok(Instruction::Slt { rd, rs1, rs2 }),
                (0x00, 0x3) => Ok(Instruction::Sltu { rd, rs1, rs2 }),
                (0x00, 0x4) => Ok(Instruction::Xor { rd, rs1, rs2 }),
                (0x00, 0x5) => Ok(Instruction::Srl { rd, rs1, rs2 }),
                (0x20, 0x5) => Ok(Instruction::Sra { rd, rs1, rs2 }),
                (0x00, 0x6) => Ok(Instruction::Or { rd, rs1, rs2 }),
                (0x00, 0x7) => Ok(Instruction::And { rd, rs1, rs2 }),
                // M-extension
                (0x01, 0x0) => Ok(Instruction::Mul { rd, rs1, rs2 }),
                (0x01, 0x1) => Ok(Instruction::Mulh { rd, rs1, rs2 }),
                (0x01, 0x2) => Ok(Instruction::Mulhsu { rd, rs1, rs2 }),
                (0x01, 0x3) => Ok(Instruction::Mulhu { rd, rs1, rs2 }),
                (0x01, 0x4) => Ok(Instruction::Div { rd, rs1, rs2 }),
                (0x01, 0x5) => Ok(Instruction::Divu { rd, rs1, rs2 }),
                (0x01, 0x6) => Ok(Instruction::Rem { rd, rs1, rs2 }),
                (0x01, 0x7) => Ok(Instruction::Remu { rd, rs1, rs2 }),
                _ => Err(DecodeError::IllegalInstruction(word)),
            }
        }
        0x13 => { // I-type ALU
            let imm = util::imm_i(word);
            match f3 {
                0x0 => Ok(Instruction::Addi { rd, rs1, imm }),
                0x2 => Ok(Instruction::Slti { rd, rs1, imm }),
                0x3 => Ok(Instruction::Sltiu { rd, rs1, imm }),
                0x4 => Ok(Instruction::Xori { rd, rs1, imm }),
                0x6 => Ok(Instruction::Ori { rd, rs1, imm }),
                0x7 => Ok(Instruction::Andi { rd, rs1, imm }),
                0x1 => Ok(Instruction::Slli { rd, rs1, shamt: util::shamt(word) }),
                0x5 => {
                    if f7 == 0x00 { Ok(Instruction::Srli { rd, rs1, shamt: util::shamt(word) }) }
                    else if f7 == 0x20 { Ok(Instruction::Srai { rd, rs1, shamt: util::shamt(word) }) }
                    else { Err(DecodeError::IllegalInstruction(word)) }
                }
                _ => Err(DecodeError::IllegalInstruction(word)),
            }
        }
        0x37 => Ok(Instruction::Lui { rd, imm: util::imm_u(word) }),
        0x17 => Ok(Instruction::Auipc { rd, imm: util::imm_u(word) }),
        0x6f => Ok(Instruction::Jal { rd, imm: util::imm_j(word) }),
        0x67 => Ok(Instruction::Jalr { rd, rs1, imm: util::imm_i(word) }),
        0x63 => { // B-type
            let imm = util::imm_b(word);
            match f3 {
                0x0 => Ok(Instruction::Beq { rs1, rs2, imm }),
                0x1 => Ok(Instruction::Bne { rs1, rs2, imm }),
                0x4 => Ok(Instruction::Blt { rs1, rs2, imm }),
                0x5 => Ok(Instruction::Bge { rs1, rs2, imm }),
                0x6 => Ok(Instruction::Bltu { rs1, rs2, imm }),
                0x7 => Ok(Instruction::Bgeu { rs1, rs2, imm }),
                _ => Err(DecodeError::IllegalInstruction(word)),
            }
        }
        0x03 => { // Load
            let imm = util::imm_i(word);
            match f3 {
                0x0 => Ok(Instruction::Lb { rd, rs1, imm }),
                0x1 => Ok(Instruction::Lh { rd, rs1, imm }),
                0x2 => Ok(Instruction::Lw { rd, rs1, imm }),
                0x4 => Ok(Instruction::Lbu { rd, rs1, imm }),
                0x5 => Ok(Instruction::Lhu { rd, rs1, imm }),
                _ => Err(DecodeError::IllegalInstruction(word)),
            }
        }
        0x23 => { // Store
            let imm = util::imm_s(word);
            match f3 {
                0x0 => Ok(Instruction::Sb { rs1, rs2, imm }),
                0x1 => Ok(Instruction::Sh { rs1, rs2, imm }),
                0x2 => Ok(Instruction::Sw { rs1, rs2, imm }),
                _ => Err(DecodeError::IllegalInstruction(word)),
            }
        }
        0x73 => {
            match util::imm_i(word) {
                0x000 => Ok(Instruction::Ecall),
                0x001 => Ok(Instruction::Ebreak),
                _ => Err(DecodeError::IllegalInstruction(word)),
            }
        }
        _ => Err(DecodeError::IllegalInstruction(word)),
    }
}