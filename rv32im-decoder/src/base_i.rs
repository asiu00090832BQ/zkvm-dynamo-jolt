use crate::error::ZkvmError;
use crate::formats::{funct7, BType, IType, RType, SType, UType, JType};
use crate::instruction::{
    AluImmKind,
    AluRegKind,
    BranchKind,
    Instruction,
    LoadKind,
    StoreKind,
};

pub fn decode_base_i(word: u32) -> Result<Instruction, ZkvmError> {
    match word & 0x7f {
        0x13 => decode_op_imm(word),
        0x37 => {
            let u = UType::decode(word);
            Ok(Instruction::Lui { rd: u.rd, imm: u.imm })
        }
        0x17 => {
            let u = UType::decode(word);
            Ok(Instruction::Auipc { rd: u.rd, imm: u.imm })
        }
        0x6f => {
            let j = JType::decode(word);
            Ok(Instruction::Jal { rd: j.rd, imm: j.imm })
        }
        0x67 => {
            let i = IType::decode(word);
            if i.funct3 != 0 {
                return Err(ZkvmError::InvalidInstruction(word));
            }
            Ok(Instruction::Jalr {
                rd: i.rd,
                rs1: i.rs1,
                imm: i.imm,
            })
        }
        0x63 => decode_branch(word),
        0x03 => decode_load(word),
        0x23 => decode_store(word),
        0x33 => decode_op(word),
        _ => Err(ZkvmError::InvalidInstruction(word)),
    }
}

fn decode_op_imm(word: u32) -> Result<Instruction, ZkvmError> {
    let i = IType::decode(word);
    let kind = match i.funct3 {
        0x0 => AluImmKind::Addi,
        0x2 => AluImmKind::Slti,
        0x3 => AluImmKind::Sltiu,
        0x4 => AluImmKind::Xori,
        0x6 => AluImmKind::Ori,
        0x7 => AluImmKind::Andi,
        0x1 => {
            if funct7(word) != 0x00 {
                return Err(ZkvmError::InvalidInstruction(word));
            }
            AluImmKind::Slli
        }
        0x5 => match funct7(word) {
            0x00 => AluImmKind::Srli,
            0x20 => AluImmKind::Srai,
            _ => return Err(ZkvmError::InvalidInstruction(word)),
        },
        _ => return Err(ZkvmError::InvalidInstruction(word)),
    };

    Ok(Instruction::OpImm {
        kind,
        rd: i.rd,
        rs1: i.rs1,
        imm: i.imm,
    })
}

fn decode_op(word: u32) -> Result<Instruction, ZkvmError> {
    let r = RType::decode(word);
    let kind = match (r.funct3, r.funct7) {
        (0x0, 0x00) => AluRegKind::Add,
        (0x0, 0x20) => AluRegKind::Sub,
        (0x1, 0x00) => AluRegKind::Sll,
        (0x2, 0x00) => AluRegKind::Slt,
        (0x3, 0x00) => AluRegKind::Sltu,
        (0x4, 0x00) => AluRegKind::Xor,
        (0x5, 0x00) => AluRegKind::Srl,
        (0x5, 0x20) => AluRegKind::Sra,
        (0x6, 0x00) => AluRegKind::Or,
        (0x7, 0x00) => AluRegKind::And,
        _ => return Err(ZkvmError::InvalidInstruction(word)),
    };

    Ok(Instruction::Op {
        kind,
        rd: r.rd,
        rs1: r.rs1,
        rs2: r.rs2,
    })
}

fn decode_branch(word: u32) -> Result<Instruction, ZkvmError> {
    let b = BType::decode(word);
    let kind = match b.funct3 {
        0x0 => BranchKind::Beq,
        0x1 => BranchKind::Bne,
        0x4 => BranchKind::Blt,
        0x5 => BranchKind::Bge,
        0x6 => BranchKind::Bltu,
        0x7 => BranchKind::Bgeu,
        _ => return Err(ZkvmError::InvalidInstruction(word)),
    };

    Ok(Instruction::Branch {
        kind,
        rs1: b.rs1,
        rs2: b.rs2,
        imm: b.imm,
    })
}

fn decode_load(word: u32) -> Result<Instruction, ZkvmError> {
    let i = IType::decode(word);
    let kind = match i.funct3 {
        0x0 => LoadKind::Lb,
        0x1 => LoadKind::Lh,
        0x2 => LoadKind::Lw,
        0x4 => LoadKind::Lbu,
        0x5 => LoadKind::Lhu,
        _ => return Err(ZkvmError::InvalidInstruction(word)),
    };

    Ok(Instruction::Load {
        kind,
        rd: i.rd,
        rs1: i.rs1,
        imm: i.imm,
    })
}

fn decode_store(word: u32) -> Result<Instruction, ZkvmError> {
    let s = SType::decode(word);
    let kind = match s.funct3 {
        0x0 => StoreKind::Sb,
        0x1 => StoreKind::Sh,
        0x2 => StoreKind::Sw,
        _ => return Err(ZkvmError::InvalidInstruction(word)),
    };

    Ok(Instruction::Store {
        kind,
        rs1: s.rs1,
        rs2: s.rs2,
        imm: s.imm,
    })
}
