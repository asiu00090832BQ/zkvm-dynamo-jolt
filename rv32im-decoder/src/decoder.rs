use crate::error::DecodeError;
use crate::instruction::Instruction;
use crate::m_extension::{decode_m_instruction, M_FUNCT7};
use crate::selectors::{
    funct3, funct7, imm_b, imm_i, imm_j, imm_s, imm_u, opcode, rd, rs1, rs2, shamt,
    OPCODE_AUIPC, OPCODE_BRANCH, OPCODE_JAL, OPCODE_JALR, OPCODE_LOAD, OPCODE_LUI,
    OPCODE_MISC_MEM, OPCODE_OP, OPCODE_OP_IMM, OPCODE_STORE, OPCODE_SYSTEM,
};

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    match opcode(word) {
        OPCODE_LUI => Ok(Instruction::Lui {
            rd: rd(word),
            imm: imm_u(word),
        }),
        OPCODE_AUIPC => Ok(Instruction::Auipc {
            rd: rd(word),
            imm: imm_u(word),
        }),
        OPCODE_JAL => Ok(Instruction::Jal {
            rd: rd(word),
            imm: imm_j(word),
        }),
        OPCODE_JALR => decode_jalr(word),
        OPCODE_BRANCH => decode_branch(word),
        OPCODE_LOAD => decode_load(word),
        OPCODE_STORE => decode_store(word),
        OPCODE_OP_IMM => decode_op_imm(word),
        OPCODE_OP => decode_op(word),
        OPCODE_MISC_MEM => decode_misc_mem(word),
        OPCODE_SYSTEM => decode_system(word),
        value => Err(DecodeError::InvalidOpcode(value)),
    }
}

fn decode_jalr(word: u32) -> Result<Instruction, DecodeError> {
    match funct3(word) {
        0b000 => Ok(Instruction::Jalr {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        }),
        value => Err(DecodeError::InvalidFunct3 {
            word,
            opcode: opcode(word),
            funct3: value,
        }),
    }
}

fn decode_branch(word: u32) -> Result<Instruction, DecodeError> {
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
        value => Err(DecodeError::InvalidFunct3 {
            word,
            opcode: opcode(word),
            funct3: value,
        }),
    }
}

fn decode_load(word: u32) -> Result<Instruction, DecodeError> {
    let rd = rd(word);
    let rs1 = rs1(word);
    let imm = imm_i(word);

    match funct3(word) {
        0b000 => Ok(Instruction::Lb { rd, rs1, imm }),
        0b001 => Ok(Instruction::Lh { rd, rs1, imm }),
        0b010 => Ok(Instruction::Lw { rd, rs1, imm }),
        0b100 => Ok(Instruction::Lbu { rd, rs1, imm }),
        0b101 => Ok(Instruction::Lhu { rd, rs1, imm }),
        value => Err(DecodeError::InvalidFunct3 {
            word,
            opcode: opcode(word),
            funct3: value,
        }),
    }
}

fn decode_store(word: u32) -> Result<Instruction, DecodeError> {
    let rs1 = rs1(word);
    let rs2 = rs2(word);
    let imm = imm_s(word);

    match funct3(word) {
        0b000 => Ok(Instruction::Sb { rs1, rs2, imm }),
        0b001 => Ok(Instruction::Sh { rs1, rs2, imm }),
        0b010 => Ok(Instruction::Sw { rs1, rs2, imm }),
        value => Err(DecodeError::InvalidFunct3 {
            word,
            opcode: opcode(word),
            funct3: value,
        }),
    }
}

fn decode_op_imm(word: u32) -> Result<Instruction, DecodeError> {
    let rd = rd(word);
    let rs1 = rs1(word);

    match funct3(word) {
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
        0b001 => match funct7(word) {
            0b0000000 => Ok(Instruction::Slli {
                rd,
                rs1,
                shamt: shamt(word),
            }),
            value => Err(DecodeError::InvalidFunct7 {
                word,
                opcode: opcode(word),
                funct3: funct3(word),
                funct7: value,
            }),
        },
        0b101 => match funct7(word) {
            0b0000000 => Ok(Instruction::Srli {
                rd,
                rs1,
                shamt: shamt(word),
            }),
            0b0100000 => Ok(Instruction::Srai {
                rd,
                rs1,
                shamt: shamt(word),
            }),
            value => Err(DecodeError::InvalidFunct7 {
                word,
                opcode: opcode(word),
                funct3: funct3(word),
                funct7: value,
            }),
        },
        value => Err(DecodeError::InvalidFunct3 {
            word,
            opcode: opcode(word),
            funct3: value,
        }),
    }
}

fn decode_op(word: u32) -> Result<Instruction, DecodeError> {
    let rd = rd(word);
    let rs1 = rs1(word);
    let rs2 = rs2(word);

    match (funct7(word), funct3(word)) {
        (0b0000000, 0b000) => Ok(Instruction::Add { rd, rs1, rs2 }),
        (0b0100000, 0b000) => Ok(Instruction::Sub { rd, rs1, rs2 }),
        (0b0000000, 0b001) => Ok(Instruction::Sll { rd, rs1, rs2 }),
        (0b0000000, 0b010) => Ok(Instruction::Slt { rd, rs1, rs2 }),
        (0b0000000, 0b011) => Ok(Instruction::Sltu { rd, rs1, rs2 }),
        (0b0000000, 0b100) => Ok(Instruction::Xor { rd, rs1, rs2 }),
        (0b0000000, 0b101) => Ok(Instruction::Srl { rd, rs1, rs2 }),
        (0b0100000, 0b101) => Ok(Instruction::Sra { rd, rs1, rs2 }),
        (0b0000000, 0b110) => Ok(Instruction::Or { rd, rs1, rs2 }),
        (0b0000000, 0b111) => Ok(Instruction::And { rd, rs1, rs2 }),
        (M_FUNCT7, _) => decode_m_instruction(word),
        (_, value) => Err(DecodeError::InvalidFunct7 {
            word,
            opcode: opcode(word),
            funct3: value,
            funct7: funct7(word),
        }),
    }
}

fn decode_misc_mem(word: u32) -> Result<Instruction, DecodeError> {
    match funct3(word) {
        0b000 => Ok(Instruction::Fence),
        0b001 => Ok(Instruction::FenceI),
        value => Err(DecodeError::InvalidFunct3 {
            word,
            opcode: opcode(word),
            funct3: value,
        }),
    }
}

fn decode_system(word: u32) -> Result<Instruction, DecodeError> {
    match word {
        0x0000_0073 => Ok(Instruction::Ecall),
        0x0010_0073 => Ok(Instruction::Ebreak),
        _ => Err(DecodeError::InvalidInstruction(word)),
    }
}
