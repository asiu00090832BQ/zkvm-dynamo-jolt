use crate::error::DecoderError;
use crate::instruction::{
    BranchKind, Instruction, LoadKind, OpImmKind, OpKind, StoreKind, SystemKind,
};
use crate::selectors::{FUNCT7_ALT, FUNCT7_BASE, OPCODE_MISC_MEM, OPCODE_OP, OPCODE_OPIMM};
use crate::util::{funct3, funct7, imm_b, imm_i, imm_s, rd, rs1, rs2, shamt};
use crate::types::Word;

pub fn decode_branch(word: Word) -> Result<Instruction, DecoderError> {
    let kind = match funct3(word) {
        0b000 => BranchKind::Beq,
        0b001 => BranchKind::Bne,
        0b100 => BranchKind::Blt,
        0b101 => BranchKind::Bge,
        0b110 => BranchKind::Bltu,
        0b111 => BranchKind::Bgeu,
        other => {
            return Err(DecoderError::InvalidFunct3 {
                opcode: crate::selectors::OPCODE_BRANCH,
                funct3: other,
            });
        }
    };

    Ok(Instruction::Branch {
        kind,
        rs1: rs1(word),
        rs2: rs2(word),
        imm: imm_b(word),
    })
}

pub fn decode_load(word: Word) -> Result<Instruction, DecoderError> {
    let kind = match funct3(word) {
        0b000 => LoadKind::Byte,
        0b001 => LoadKind::Half,
        0b010 => LoadKind::Word,
        0b100 => LoadKind::ByteUnsigned,
        0b101 => LoadKind::HalfUnsigned,
        other => {
            return Err(DecoderError::InvalidFunct3 {
                opcode: crate::selectors::OPCODE_LOAD,
                funct3: other,
            });
        }
    };

    Ok(Instruction::Load {
        kind,
        rd: rd(word),
        rs1: rs1(word),
        imm: imm_i(word),
    })
}

pub fn decode_store(word: Word) -> Result<Instruction, DecoderError> {
    let kind = match funct3(word) {
        0b000 => StoreKind::Byte,
        0b001 => StoreKind::Half,
        0b010 => StoreKind::Word,
        other => {
            return Err(DecoderError::InvalidFunct3 {
                opcode: crate::selectors::OPCODE_STORE,
                funct3: other,
            });
        }
    };

    Ok(Instruction::Store {
        kind,
        rs1: rs1(word),
        rs2: rs2(word),
        imm: imm_s(word),
    })
}

pub fn decode_op_imm(word: Word) -> Result<Instruction, DecoderError> {
    let kind = match funct3(word) {
        0b000 => OpImmKind::Addi,
        0b010 => OpImmKind::Slti,
        0b011 => OpImmKind::Sltiu,
        0b100 => OpImmKind::Xori,
        0b110 => OpImmKind::Ori,
        0b111 => OpImmKind::Andi,
        0b001 => {
            if funct7(word) != FUNCT7_BASE {
                return Err(DecoderError::InvalidFunct7 {
                    opcode: OPCODE_OPIMM,
                    funct3: funct3(word),
                    funct7: funct7(word),
                });
            }
            OpImmKind::Slli
        }
        0b101 => match funct7(word) {
            FUNCT7_BASE => OpImmKind::Srli,
            FUNCT7_ALT => OpImmKind::Srai,
            other => {
                return Err(DecoderError::InvalidFunct7 {
                    opcode: OPCODE_OPIMM,
                    funct3: funct3(word),
                    funct7: other,
                });
            }
        },
        other => {
            return Err(DecoderError::InvalidFunct3 {
                opcode: OPCODE_OPIMM,
                funct3: other,
            });
        }
    };

    let imm = match kind {
        OpImmKind::Slli | OpImmKind::Srli | OpImmKind::Srai => shamt(word) as i32,
        _ => imm_i(word),
    };

    Ok(Instruction::OpImm {
        kind,
        rd: rd(word),
        rs1: rs1(word),
        imm,
    })
}

pub fn decode_op(word: Word) -> Result<Instruction, DecoderError> {
    let kind = match (funct3(word), funct7(word)) {
        (0b000, FUNCT7_BASE) => OpKind::Add,
        (0b000, FUNCT7_ALT) => OpKind::Sub,
        (0b001, FUNCT7_BASE) => OpKind::Sll,
        (0b010, FUNCT7_BASE) => OpKind::Slt,
        (0b011, FUNCT7_BASE) => OpKind::Sltu,
        (0b100, FUNCT7_BASE) => OpKind::Xor,
        (0b101, FUNCT7_BASE) => OpKind::Srl,
        (0b101, FUNCT7_ALT) => OpKind::Sra,
        (0b110, FUNCT7_BASE) => OpKind::Or,
        (0b111, FUNCT7_BASE) => OpKind::And,
        (funct3_value, funct7_value) => {
            return Err(DecoderError::InvalidFunct7 {
                opcode: OPCODE_OP,
                funct3: funct3_value,
                funct7: funct7_value,
            });
        }
    };

    Ok(Instruction::Op {
        kind,
        rd: rd(word),
        rs1: rs1(word),
        rs2: rs2(word),
    })
}

pub fn decode_misc_mem(word: Word) -> Result<Instruction, DecoderError> {
    match funct3(word) {
        0b000 => Ok(Instruction::Fence),
        0b001 => Ok(Instruction::FenceI),
        other => Err(DecoderError::InvalidFunct3 {
            opcode: OPCODE_MISC_MEM,
            funct3: other,
        }),
    }
}

pub fn decode_system(word: Word) -> Result<Instruction, DecoderError> {
    match word {
        0x0000_0073 => Ok(Instruction::System(SystemKind::Ecall)),
        0x0010_0073 => Ok(Instruction::System(SystemKind::Ebreak)),
        _ => Err(DecoderError::ReservedInstruction(word)),
    }
}
