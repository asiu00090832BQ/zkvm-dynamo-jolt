use crate::{
    decoder::{
        fields::{funct3, funct7, imm_b, imm_i, imm_j, imm_s, imm_u, opcode, rd, rs1, rs2, shamt},
        invariants,
    },
    error::ZkvmError,
    types::{BranchKind, Instruction, LoadKind, Op, OpImm, StoreKind},
};

const OPCODE_LUI: u8 = 0b0110111;
const OPCODE_AUIPC: u8 = 0b0010111;
const OPCODE_JAL: u8 = 0b1101111;
const OPCODE_JALR: u8 = 0b1100111;
const OPCODE_BRANCH: u8 = 0b1100011;
const OPCODE_LOAD: u8 = 0b0000011;
const OPCODE_STORE: u8 = 0b0100011;
const OPCODE_OP_IMM: u8 = 0b0010011;
const OPCODE_OP: u8 = 0b0110011;
const OPCODE_FENCE: u8 = 0b0001111;
const OPCODE_SYSTEM: u8 = 0b1110011;

pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    match opcode(word) {
        OPCODE_LUI => decode_lui(word),
        OPCODE_AUIPC => decode_auipc(word),
        OPCODE_JAL => decode_jal(word),
        OPCODE_JALR => decode_jalr(word),
        OPCODE_BRANCH => decode_branch(word),
        OPCODE_LOAD => decode_load(word),
        OPCODE_STORE => decode_store(word),
        OPCODE_OP_IMM => decode_op_imm(word),
        OPCODE_OP => decode_op(word),
        OPCODE_FENCE => decode_fence(word),
        OPCODE_SYSTEM => decode_system(word),
        op => Err(ZkvmError::InvalidOpcode { opcode: op, word }),
    }
}

fn decode_lui(word: u32) -> Result<Instruction, ZkvmError> {
    Ok(Instruction::Lui {
        rd: invariants::register(rd(word), "lui rd")?,
        imm: imm_u(word),
    })
}

fn decode_auipc(word: u32) -> Result<Instruction, ZkvmError> {
    Ok(Instruction::Auipc {
        rd: invariants::register(rd(word), "auipc rd")?,
        imm: imm_u(word),
    })
}

fn decode_jal(word: u32) -> Result<Instruction, ZkvmError> {
    Ok(Instruction::Jal {
        rd: invariants::register(rd(word), "jal rd")?,
        imm: imm_j(word),
    })
}

fn decode_jalr(word: u32) -> Result<Instruction, ZkvmError> {
    if funct3(word) != 0b000 {
        return Err(ZkvmError::InvalidFunct3 {
            funct3: funct3(word),
            opcode: OPCODE_JALR,
            word,
        });
    }

    Ok(Instruction::Jalr {
        rd: invariants::register(rd(word), "jalr rd")?,
        rs1: invariants::register(rs1(word), "jalr rs1")?,
        imm: imm_i(word),
    })
}

fn decode_branch(word: u32) -> Result<Instruction, ZkvmError> {
    let kind = match funct3(word) {
        0b000 => BranchKind::Beq,
        0b001 => BranchKind::Bne,
        0b100 => BranchKind::Blt,
        0b101 => BranchKind::Bge,
        0b110 => BranchKind::Bltu,
        0b111 => BranchKind::Bgeu,
        other => {
            return Err(ZkvmError::InvalidFunct3 {
                funct3: other,
                opcode: OPCODE_BRANCH,
                word,
            })
        }
    };

    Ok(Instruction::Branch {
        kind,
        rs1: invariants::register(rs1(word), "branch rs1")?,
        rs2: invariants::register(rs2(word), "branch rs2")?,
        imm: imm_b(word),
    })
}

fn decode_load(word: u32) -> Result<Instruction, ZkvmError> {
    let kind = match funct3(word) {
        0b000 => LoadKind::Lb,
        0b001 => LoadKind::Lh,
        0b010 => LoadKind::Lw,
        0b100 => LoadKind::Lbu,
        0b101 => LoadKind::Lhu,
        other => {
            return Err(ZkvmError::InvalidFunct3 {
                funct3: other,
                opcode: OPCODE_LOAD,
                word,
            })
        }
    };

    Ok(Instruction::Load {
        kind,
        rd: invariants::register(rd(word), "load rd")?,
        rs1: invariants::register(rs1(word), "load rs1")?,
        imm: imm_i(word),
    })
}

fn decode_store(word: u32) -> Result<Instruction, ZkvmError> {
    let kind = match funct3(word) {
        0b000 => StoreKind::Sb,
        0b001 => StoreKind::Sh,
        0b010 => StoreKind::Sw,
        other => {
            return Err(ZkvmError::InvalidFunct3 {
                funct3: other,
                opcode: OPCODE_STORE,
                word,
            })
        }
    };

    Ok(Instruction::Store {
        kind,
        rs1: invariants::register(rs1(word), "store rs1")?,
        rs2: invariants::register(rs2(word), "store rs2")?,
        imm: imm_s(word),
    })
}

fn decode_op_imm(word: u32) -> Result<Instruction, ZkvmError> {
    let rd = invariants::register(rd(word), "op-imm rd")?;
    let rs1 = invariants::register(rs1(word), "op-imm rs1")?;

    let (kind, imm) = match funct3(word) {
        0b000 => (OpImm::Addi, imm_i(word)),
        0b010 => (OpImm::Slti, imm_i(word)),
        0b011 => (OpImm::Sltiu, imm_i(word)),
        0b100 => (OpImm::Xori, imm_i(word)),
        0b110 => (OpImm::Ori, imm_i(word)),
        0b111 => (OpImm::Andi, imm_i(word)),
        0b001 => {
            invariants::expect_funct7(word, OPCODE_OP_IMM, funct7(word), &[0b0000000])?;
            (OpImm::Slli, shamt(word) as i32)
        }
        0b101 => match funct7(word) {
            0b0000000 => (OpImm::Srli, shamt(word) as i32),
            0b0100000 => (OpImm::Srai, shamt(word) as i32),
            other => {
                return Err(ZkvmError::InvalidFunct7 {
                    funct7: other,
                    opcode: OPCODE_OP_IMM,
                    word,
                })
            }
        },
        other => {
            return Err(ZkvmError::InvalidFunct3 {
                funct3: other,
                opcode: OPCODE_OP_IMM,
                word,
            })
        }
    };

    Ok(Instruction::OpImm { kind, rd, rs1, imm })
}

fn decode_op(word: u32) -> Result<Instruction, ZkvmError> {
    let rd = invariants::register(rd(word), "op rd")?;
    let rs1 = invariants::register(rs1(word), "op rs1")?;
    let rs2 = invariants::register(rs2(word), "op rs2")?;

    let kind = match (funct3(word), funct7(word)) {
        (0b000, 0b0000000) => Op::Add,
        (0b000, 0b0100000) => Op::Sub,
        (0b001, 0b0000000) => Op::Sll,
        (0b010, 0b0000000) => Op::Slt,
        (0b011, 0b0000000) => Op::Sltu,
        (0b100, 0b0000000) => Op::Xor,
        (0b101, 0b0000000) => Op::Srl,
        (0b101, 0b0100000) => Op::Sra,
        (0b110, 0b0000000) => Op::Or,
        (0b111, 0b0000000) => Op::And,
        (funct3, other_funct7) => {
            return Err(ZkvmError::InvalidFunct7 {
                funct7: other_funct7,
                opcode: if funct3 <= 0b111 { OPCODE_OP } else { opcode(word) },
                word,
            })
        }
    };

    Ok(Instruction::Op { kind, rd, rs1, rs2 })
}

fn decode_fence(word: u32) -> Result<Instruction, ZkvmError> {
    if funct3(word) == 0b000 {
        Ok(Instruction::Fence)
    } else {
        Err(ZkvmError::InvalidFunct3 {
            funct3: funct3(word),
            opcode: OPCODE_FENCE,
            word,
        })
    }
}

fn decode_system(word: u32) -> Result<Instruction, ZkvmError> {
    if funct3(word) != 0b000 {
        return Err(ZkvmError::UnsupportedInstruction {
            word,
            reason: "CSR/system sub-opcodes beyond ECALL/EBREAK are not implemented",
        });
    }

    match (word >> 20) & 0x0fff {
        0 => Ok(Instruction::Ecall),
        1 => Ok(Instruction::Ebreak),
        _ => Err(ZkvmError::UnsupportedInstruction {
            word,
            reason: "system immediate is not ECALL or EBREAK",
        }),
    }
}
