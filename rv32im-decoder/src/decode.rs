use crate::{
    error::DecodeError,
    fields,
    imm,
    instruction::{BranchKind, Instruction, LoadKind, OpImmKind, OpKind, StoreKind, SystemKind},
    opcode::Opcode,
    validate::validate,
};

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    if (word & 0b11) != 0b11 {
        return Err(DecodeError::UnsupportedCompressed((word & 0xffff) as u16));
    }

    let opcode = Opcode::from_word(word)?;
    let instruction = match opcode {
        Opcode::Lui => Instruction::Lui {
            rd: fields::rd(word),
            imm: imm::imm_u(word),
        },
        Opcode::Auipc => Instruction::Auipc {
            rd: fields::rd(word),
            imm: imm::imm_u(word),
        },
        Opcode::Jal => Instruction::Jal {
            rd: fields::rd(word),
            imm: imm::imm_j(word),
        },
        Opcode::Jalr => {
            if fields::funct3(word) != 0 {
                return Err(DecodeError::InvalidFunct3 {
                    opcode: opcode.bits(),
                    funct3: fields::funct3(word),
                });
            }
            Instruction::Jalr {
                rd: fields::rd(word),
                rs1: fields::rs1(word),
                imm: imm::imm_i(word),
            }
        }
        Opcode::Branch => {
            let kind = match fields::funct3(word) {
                0b000 => BranchKind::Beq,
                0b001 => BranchKind::Bne,
                0b100 => BranchKind::Blt,
                0b101 => BranchKind::Bge,
                0b110 => BranchKind::Bltu,
                0b111 => BranchKind::Bgeu,
                funct3 => {
                    return Err(DecodeError::InvalidFunct3 {
                        opcode: opcode.bits(),
                        funct3,
                    })
                }
            };
            Instruction::Branch {
                kind,
                rs1: fields::rs1(word),
                rs2: fields::rs2(word),
                imm: imm::imm_b(word),
            }
        }
        Opcode::Load => {
            let kind = match fields::funct3(word) {
                0b000 => LoadKind::Lb,
                0b001 => LoadKind::Lh,
                0b010 => LoadKind::Lw,
                0b100 => LoadKind::Lbu,
                0b101 => LoadKind::Lhu,
                funct3 => {
                    return Err(DecodeError::InvalidFunct3 {
                        opcode: opcode.bits(),
                        funct3,
                    })
                }
            };
            Instruction::Load {
                kind,
                rd: fields::rd(word),
                rs1: fields::rs1(word),
                imm: imm::imm_i(word),
            }
        }
        Opcode::Store => {
            let kind = match fields::funct3(word) {
                0b000 => StoreKind::Sb,
                0b001 => StoreKind::Sh,
                0b010 => StoreKind::Sw,
                funct3 => {
                    return Err(DecodeError::InvalidFunct3 {
                        opcode: opcode.bits(),
                        funct3,
                    })
                }
            };
            Instruction::Store {
                kind,
                rs1: fields::rs1(word),
                rs2: fields::rs2(word),
                imm: imm::imm_s(word),
            }
        }
        Opcode::OpImm => {
            let funct3 = fields::funct3(word);
            let kind = match funct3 {
                0b000 => OpImmKind::Addi,
                0b010 => OpImmKind::Slti,
                0b011 => OpImmKind::Sltiu,
                0b100 => OpImmKind::Xori,
                0b110 => OpImmKind::Ori,
                0b111 => OpImmKind::Andi,
                0b001 => {
                    if fields::funct7(word) != 0 {
                        return Err(DecodeError::InvalidFunct7 {
                            opcode: opcode.bits(),
                            funct7: fields::funct7(word),
                        });
                    }
                    OpImmKind::Slli
                }
                0b101 => match fields::funct7(word) {
                    0b0000000 => OpImmKind::Srli,
                    0b0100000 => OpImmKind::Srai,
                    funct7 => {
                        return Err(DecodeError::InvalidFunct7 {
                            opcode: opcode.bits(),
                            funct7,
                        })
                    }
                },
                funct3 => {
                    return Err(DecodeError::InvalidFunct3 {
                        opcode: opcode.bits(),
                        funct3,
                    })
                }
            };

            let imm = if matches!(kind, OpImmKind::Slli | OpImmKind::Srli | OpImmKind::Srai) {
                fields::shamt(word) as i32
            } else {
                imm::imm_i(word)
            };

            Instruction::OpImm {
                kind,
                rd: fields::rd(word),
                rs1: fields::rs1(word),
                imm,
            }
        }
        Opcode::Op => {
            let kind = match (fields::funct7(word), fields::funct3(word)) {
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
                (0b0000001, 0b000) => OpKind::Mul,
                (0b0000001, 0b001) => OpKind::Mulh,
                (0b0000001, 0b010) => OpKind::Mulhsu,
                (0b0000001, 0b011) => OpKind::Mulhu,
                (0b0000001, 0b100) => OpKind::Div,
                (0b0000001, 0b101) => OpKind::Divu,
                (0b0000001, 0b110) => OpKind::Rem,
                (0b0000001, 0b111) => OpKind::Remu,
                (funct7, _) if funct7 != 0b0000000 && funct7 != 0b0100000 && funct7 != 0b0000001 => {
                    return Err(DecodeError::InvalidFunct7 {
                        opcode: opcode.bits(),
                        funct7,
                    })
                }
                (_, funct3) => {
                    return Err(DecodeError::InvalidFunct3 {
                        opcode: opcode.bits(),
                        funct3,
                    })
                }
            };
            Instruction::Op {
                kind,
                rd: fields::rd(word),
                rs1: fields::rs1(word),
                rs2: fields::rs2(word),
            }
        }
        Opcode::MiscMem => {
            if fields::funct3(word) != 0b000 {
                return Err(DecodeError::InvalidFunct3 {
                    opcode: opcode.bits(),
                    funct3: fields::funct3(word),
                });
            }
            Instruction::Fence
        }
        Opcode::System => match word {
            0x0000_0073 => Instruction::System {
                kind: SystemKind::Ecall,
            },
            0x0010_0073 => Instruction::System {
                kind: SystemKind::Ebreak,
            },
            _ => return Err(DecodeError::InvalidSystem(word)),
        },
    };

    validate(&instruction)?;
    Ok(instruction)
}
