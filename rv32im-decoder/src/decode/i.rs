use crate::{
    error::{DecodeError, DecodeResult},
    fields::Fields,
    immediate,
    instruction::{
        BranchKind, CsrKind, Instruction, LoadKind, OpImmKind, OpKind, ShiftImmKind,
        StoreKind, SystemKind,
    },
    opcode::Opcode,
    word::Word,
};

pub fn decode(word: Word, fields: Fields, opcode: Opcode) -> DecodeResult<Instruction> {
    match opcode {
        Opcode::Lui => Ok(Instruction::Lui {
            rd: fields.rd_register(),
            imm: immediate::u_type(word.raw()),
        }),
        Opcode::Auipc => Ok(Instruction::Auipc {
            rd: fields.rd_register(),
            imm: immediate::u_type(word.raw()),
        }),
        Opcode::Jal => Ok(Instruction::Jal {
            rd: fields.rd_register(),
            imm: immediate::j_type(word.raw()),
        }),
        Opcode::Jalr => decode_jalr(word, fields),
        Opcode::Branch => decode_branch(word, fields),
        Opcode::Load => decode_load(word, fields),
        Opcode::Store => decode_store(word, fields),
        Opcode::OpImm => decode_op_imm(word, fields),
        Opcode::Op => decode_op(fields),
        Opcode::MiscMem => decode_misc_mem(word, fields),
        Opcode::System => decode_system(word),
    }
}

fn decode_jalr(word: Word, fields: Fields) -> DecodeResult<Instruction> {
    if fields.funct3 != 0 {
        return Err(DecodeError::UnsupportedFunct3 {
            opcode: fields.opcode,
            funct3: fields.funct3,
        });
    }

    Ok(Instruction::Jalr {
        rd: fields.rd_register(),
        rs1: fields.rs1_register(),
        imm: immediate::i_type(word.raw()),
    })
}

fn decode_branch(word: Word, fields: Fields) -> DecodeResult<Instruction> {
    let kind = match fields.funct3 {
        0b000 => BranchKind::Beq,
        0b001 => BranchKind::Bne,
        0b100 => BranchKind::Blt,
        0b101 => BranchKind::Bge,
        0b110 => BranchKind::Bltu,
        0b111 => BranchKind::Bgeu,
        _ => {
            return Err(DecodeError::UnsupportedFunct3 {
                opcode: fields.opcode,
                funct3: fields.funct3,
            })
        }
    };

    Ok(Instruction::Branch {
        kind,
        rs1: fields.rs1_register(),
        rs2: fields.rs2_register(),
        imm: immediate::b_type(word.raw()),
    })
}

fn decode_load(word: Word, fields: Fields) -> DecodeResult<Instruction> {
    let kind = match fields.funct3 {
        0b000 => LoadKind::Lb,
        0b001 => LoadKind::Lh,
        0b010 => LoadKind::Lw,
        0b100 => LoadKind::Lbu,
        0b101 => LoadKind::Lhu,
        _ => {
            return Err(DecodeError::UnsupportedFunct3 {
                opcode: fields.opcode,
                funct3: fields.funct3,
            })
        }
    };

    Ok(Instruction::Load {
        kind,
        rd: fields.rd_register(),
        rs1: fields.rs1_register(),
        imm: immediate::i_type(word.raw()),
    })
}

fn decode_store(word: Word, fields: Fields) -> DecodeResult<Instruction> {
    let kind = match fields.funct3 {
        0b000 => StoreKind::Sb,
        0b001 => StoreKind::Sh,
        0b010 => StoreKind::Sw,
        _ => {
            return Err(DecodeError::UnsupportedFunct3 {
                opcode: fields.opcode,
                funct3: fields.funct3,
            })
        }
    };

    Ok(Instruction::Store {
        kind,
        rs1: fields.rs1_register(),
        rs2: fields.rs2_register(),
        imm: immediate::s_type(word.raw()),
    })
}

fn decode_op_imm(word: Word, fields: Fields) -> DecodeResult<Instruction> {
    match fields.funct3 {
        0b000 => Ok(Instruction::OpImm {
            kind: OpImmKind::Addi,
            rd: fields.rd_register(),
            rs1: fields.rs1_register(),
            imm: immediate::i_type(word.raw()),
        }),
        0b010 => Ok(Instruction::OpImm {
            kind: OpImmKind::Slti,
            rd: fields.rd_register(),
            rs1: fields.rs1_register(),
            imm: immediate::i_type(word.raw()),
        }),
        0b011 => Ok(Instruction::OpImm {
            kind: OpImmKind::Sltiu,
            rd: fields.rd_register(),
            rs1: fields.rs1_register(),
            imm: immediate::i_type(word.raw()),
        }),
        0b100 => Ok(Instruction::OpImm {
            kind: OpImmKind::Xori,
            rd: fields.rd_register(),
            rs1: fields.rs1_register(),
            imm: immediate::i_type(word.raw()),
        }),
        0b110 => Ok(Instruction::OpImm {
            kind: OpImmKind::Ori,
            rd: fields.rd_register(),
            rs1: fields.rs1_register(),
            imm: immediate::i_type(word.raw()),
        }),
        0b111 => Ok(Instruction::OpImm {
            kind: OpImmKind::Andi,
            rd: fields.rd_register(),
            rs1: fields.rs1_register(),
            imm: immediate::i_type(word.raw()),
        }),
        0b001 => {
            if fields.funct7 != 0b0000000 {
                return Err(DecodeError::UnsupportedFunct7 {
                    opcode: fields.opcode,
                    funct3: fields.funct3,
                    funct7: fields.funct7,
                });
            }

            Ok(Instruction::ShiftImm {
                kind: ShiftImmKind::Slli,
                rd: fields.rd_register(),
                rs1: fields.rs1_register(),
                shamt: immediate::shamt(word.raw()),
            })
        }
        0b101 => {
            let kind = match fields.funct7 {
                0b0000000 => ShiftImmKind::Srli,
                0b0100000 => ShiftImmKind::Srai,
                _ => {
                    return Err(DecodeError::UnsupportedFunct7 {
                        opcode: fields.opcode,
                        funct3: fields.funct3,
                        funct7: fields.funct7,
                    })
                }
            };

            Ok(Instruction::ShiftImm {
                kind,
                rd: fields.rd_register(),
                rs1: fields.rs1_register(),
                shamt: immediate::shamt(word.raw()),
            })
        }
        _ => Err(DecodeError::UnsupportedFunct3 {
            opcode: fields.opcode,
            funct3: fields.funct3,
        }),
    }
}

fn decode_op(fields: Fields) -> DecodeResult<Instruction> {
    let kind = match (fields.funct7, fields.funct3) {
        (0b0000000, 0b000) => OpKind::Add,
        (0b0100000, 0b000) => OpKind::Sub,
        (0b0000000, 0b001) => OpKind::Sll,
        (0b0000000, 0b010) => OpKind::Slt,
        (0b0000000, 0b011) => OpKind::Sltu,
        (0b0000000, 0b100) => OpKind::Xor,
        (0b0000000, 0b101) => OpKind::Srl,
        (0b0100000, 0b101) => OpKind::Sra,
        (0b0000000, 0b110) => OpKind::Or,
        (0b0000000, 0b111) => OpKind::And,
        _ => {
            return Err(DecodeError::UnsupportedFunct7 {
                opcode: fields.opcode,
                funct3: fields.funct3,
                funct7: fields.funct7,
            })
        }
    };

    Ok(Instruction::Op {
        kind,
        rd: fields.rd_register(),
        rs1: fields.rs1_register(),
        rs2: fields.rs2_register(),
    })
}

fn decode_misc_mem(word: Word, fields: Fields) -> DecodeResult<Instruction> {
    match fields.funct3 {
        0b000 => Ok(Instruction::Fence {
            fm: ((word.raw() >> 28) & 0x0f) as u8,
            pred: ((word.raw() >> 24) & 0x0f) as u8,
            succ: ((word.raw() >> 20) & 0x0f) as u8,
        }),
        0b001 => Ok(Instruction::FenceI),
        _ => Err(DecodeError::UnsupportedFunct3 {
            opcode: fields.opcode,
            funct3: fields.funct3,
        }),
    }
}

fn decode_system(word: Word) -> DecodeResult<Instruction> {
    let fields = Fields::from_word(word);

    match fields.funct3 {
        0b000 => match word.raw() {
            0x0000_0073 => Ok(Instruction::System {
                kind: SystemKind::Ecall,
            }),
            0x0010_0073 => Ok(Instruction::System {
                kind: SystemKind::Ebreak,
            }),
            _ => Err(DecodeError::InvalidEncoding(
                "unsupported SYSTEM instruction with funct3=0",
            )),
        },
        0b001 => Ok(Instruction::Csr {
            kind: CsrKind::Csrrw,
            rd: fields.rd_register(),
            rs1: fields.rs1_register(),
            csr: immediate::csr(word.raw()),
        }),
        0b010 => Ok(Instruction::Csr {
            kind: CsrKind::Csrrs,
            rd: fields.rd_register(),
            rs1: fields.rs1_register(),
            csr: immediate::csr(word.raw()),
        }),
        0b011 => Ok(Instruction::Csr {
            kind: CsrKind::Csrrc,
            rd: fields.rd_register(),
            rs1: fields.rs1_register(),
            csr: immediate::csr(word.raw()),
        }),
        0b101 => Ok(Instruction::CsrImm {
            kind: CsrKind::Csrrwi,
            rd: fields.rd_register(),
            zimm: immediate::zimm(word.raw()),
            csr: immediate::csr(word.raw()),
        }),
        0b110 => Ok(Instruction::CsrImm {
            kind: CsrKind::Csrrsi,
            rd: fields.rd_register(),
            zimm: immediate::zimm(word.raw()),
            csr: immediate::csr(word.raw()),
        }),
        0b111 => Ok(Instruction::CsrImm {
            kind: CsrKind::Csrrci,
            rd: fields.rd_register(),
            zimm: immediate::zimm(word.raw()),
            csr: immediate::csr(word.raw()),
        }),
        _ => Err(DecodeError::UnsupportedFunct3 {
            opcode: fields.opcode,
            funct3: fields.funct3,
        }),
    }
}
