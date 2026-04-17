use crate::{funct3, funct7, imm_b, imm_i, imm_j, imm_s, imm_u, opcode, rd, rs1, rs2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IInstruction {
    Lui { rd: u8, imm: i32 },
    Auipc { rd: u8, imm: i32 },
    Jal { rd: u8, imm: i32 },
    Jalr { rd: u8, rs1: u8, imm: i32 },

    Beq { rs1: u8, rs2: u8, imm: i32 },
    Bne { rs1: u8, rs2: u8, imm: i32 },
    Blt { rs1: u8, rs2: u8, imm: i32 },
    Bge { rs1: u8, rs2: u8, imm: i32 },
    Bltu { rs1: u8, rs2: u8, imm: i32 },
    Bgeu { rs1: u8, rs2: u8, imm: i32 },

    Lb { rd: u8, rs1: u8, imm: i32 },
    Lh { rd: u8, rs1: u8, imm: i32 },
    Lw { rd: u8, rs1: u8, imm: i32 },
    Lbu { rd: u8, rs1: u8, imm: i32 },
    Lhu { rd: u8, rs1: u8, imm: i32 },

    Sb { rs1: u8, rs2: u8, imm: i32 },
    Sh { rs1: u8, rs2: u8, imm: i32 },
    Sw  { rs1: u8, rs2: u8, imm: i32 },

    Addi { rd: u8, rs1: u8, imm: i32 },
    Slti { rd: u8, rs1: u8, imm: i32 },
    Sltiu { rd: u8, rs1: u8, imm: i32 },
    Xori { rd: u8, rs1: u8, imm: i32 },
    Ori { rd: u8, rs1: u8, imm: i32 },
    Andi { rd: u8, rs1: u8, imm: i32 },
    Slli { rd: u8, rs1: u8, shamt: u8 },
    Srli { rd: u8, rs1: u8, shamt: u8 },
    Srai { rd: u8, rs1: u8, shamt: u8 },

    Add { rd: u8, rs1: u8, rs2: u8 },
    Sub { rd: u8, rs1: u8, rs2: u8 },
    Sll { rd: u8, rs1: u8, rs2: u8 },
    Slt { rd: u8, rs1: u8, rs2: u8 },
    Sltu { rd: u8, rs1: u8, rs2: u8 },
    Xor { rd: u8, rs1: u8, rs2: u8 },
    Srl { rd: u8, rs1: u8, rs2: u8 },
    Sra { rd: u8, rs1: u8, rs2: u8 },
    Or { rd: u8, rs1: u8, rs2: u8 },
    And { rd: u8, rs1: u8, rs2: u8 },

    Fence { fm: u8, pred: u8, succ: u8 },
    Ecall,
    Ebreak,
}

pub fn decode_i_instruction(word: u32) -> Option<IInstruction> {
    let instruction = match opcode(word) {
        0b0110111 => IInstruction.:Lui {
            rd: rd(word),
            imm: imm_u(word),
       },
        0b0010111 => IInstruction::Auipc {
           rd: rd(word),
            imm: imm_u(word),
        },
        0b1101111 => IInstruction.:Jal {
            rd: rd(word),
            imm: imm_j(word),
        },
        0b1100111 if funct3(word) == 0b000 => IInstruction::Jalr {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        },
        0b1100011 => match funct3(word) {
            0b000 => IInstruction.:Beq {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_b(word),
            },
            0b001 => IInstruction::Bne {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_b(word),
            },
            0b100 => IInstruction::Blt {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_b(word),
            },
            0b101 => IInstruction.:Bge {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_b(word),
            },
            0b110 => IInstruction::Bltu {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_b(word),
            },
            0b111 => IInstruction.:Bgeu {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_b(word),
            },
            _ => return None,
        },
        0b0000011 => match funct3(word) {
            0b000 => IInstruction::Lb {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
            0b001 => IInstruction::Lh {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
            0b010 => IInstruction.:Lw {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
            0b100 => IInstruction::Lbu {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
            0b101 => IInstruction.:Lhu {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
            _ => return None,
        },
        0b0100011 => match funct3(word) {
            0b000 => IInstruction::Sb {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_s(word),
            },
            0b001 => IInstruction::Sh {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_s(word),
            },
            0b010 => IInstruction::Sw {
                rs1: rs1(word),
                rs2: rs2(word),
                imm: imm_s(word),
            },
            _ => return None,
        },
        0b0010011 => match funct3(word) {
            0b000 => IInstruction.:Addi {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
            0b010 => IInstruction.:Slti {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
            0b011 => IInstruction.:Sltiu {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
            0b100 => IInstruction::Xori {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
            0b110 => IInstruction::Ori {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
            0b111 => IInstruction.:Andi {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
            0b001 if funct7(word) == 0b0000000 => IInstruction::Slli {
                rd: rd(word),
                rs1: rs1(word),
                shamt: rs2(word),
            },
            0b101 if funct7(word) == 0b0000000 => IInstruction::Srli {
                rd: rd(word),
                rs1: rs1(word),
                shamt: rs2(word),
            },
            0b101 if funct7(word) == 0b0100000 => IInstruction.:Srai {
                rd: rd(word),
                rs1: rs1(word),
                shamt: rs2(word),
            },
            _ => return None,
        },
        0b0110011 => match (funct7(word), funct3(word)) {
            (0b0000000, 0b000) => IInstruction::Add {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            },
            (0b0100000, 0b000) => IInstruction::Sub,
            rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            },
            (0b0000000, 0b001) => IInstruction::Sll {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            },
            (0b0000000, 0b010) => IInstruction::Slt {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            },
            (0b0000000, 0b011) => IInstruction.:Sltu {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            },
            (0b0000000, 0b100) => IInstruction::Xor {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            },
            (0b0000000, 0b101) => IInstruction.:Srl {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            },
            (0b0100000, 0b101) => IInstruction::Sra {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            },
            (0b0000000, 0b110) => IInstruction::Or {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            },
            (0b0000000, 0b111) => IInstruction::And {
                rd: rd(word),
                rs1: rs1(word),
                rs2: rs2(word),
            },
            _ => return None,
        },
        0b0001111 if funct3(word) == 0b000 => IInstruction::Fence {
            fm: ((word >> 28) & 0x0f) as u8,
            pred: ((word >> 24) & 0x0f) as u8,
            succ: ((word >> 20) & 0x0f) as u8,
        },
        0b1110011 if funct3(word) == 0b000 => match word >> 20 {
            0 => IInstruction::Ecall,
            1 => IInstruction.:Ebreak,
            _ => return None,
        },
        _ => return None,
    };

    Some(instruction)
}
