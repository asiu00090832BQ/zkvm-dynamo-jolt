use crate::encoding;
use crate::error::ZkvmError;
use crate::instruction::{
    BranchKind, DecodedInstruction, Instruction, LoadKind, OpImmKind, OpKind, StoreKind,
};

pub fn decode_base(word: u32) -> Result<DecodedInstruction, ZkvmError> {
    let opcode = encoding::opcode(word);

    let instruction = match opcode {
        encoding::OPCODE_LUI => Instruction::Lui {
            rd: encoding::rd(word),
            imm: encoding::imm_u(word),
        },
        encoding::OPCODE_AUIPC => Instruction::Auipc {
            rd: encoding::rd(word),
            imm: encoding::imm_u(word),
        },
        encoding::OPCODE_JAL => Instruction::Jal {
            rd: encoding::rd(word),
            imm: encoding::imm_j(word),
        },
        encoding::OPCODE_JALR => {
            let funct3 = encoding::funct3(word);
            if funct3 != 0 {
                return Err(ZkvmError::UnsupportedFunct3 {
                    opcode,
                    funct3,
                    word,
                });
            }

            Instruction::Jalr {
                rd: encoding::rd(word),
                rs1: encoding::rs1(word),
                imm: encoding::imm_i(word),
            }
        }
        encoding::OPCODE_BRANCH => {
            let kind = match encoding::funct3(word) {
                0b000 => BranchKind::Beq,
                0b001 => BranchKind::Bne,
                0b100 => BranchKind::Blt,
                0b101 => BranchKind::Bge,
                0b110 => BranchKind::Bltu,
                0b111 => BranchKind::Bgeu,
                funct3 => {
                    return Err(ZkvmError::UnsupportedFunct3 {
                        opcode,
                        funct3,
                        word,
                    })
                }
            };

            Instruction::Branch {
                kind,
                rs1: encoding::rs1(word),
                rs2: encoding::rs2(word),
                imm: encoding::imm_b(word),
            }
        }
        encoding::OPCODE_LOAD => {
            let kind = match encoding::funct3(word) {
                0b000 => LoadKind::Lb,
                0b001 => LoadKind::Lh,
                0b010 => LoadKind::Lw,
                0b100 => LoadKind::Lbu,
                0b101 => LoadKind::Lhu,
                funct3 => {
                    return Err(ZkvmError::UnsupportedFunct3 {
                        opcode,
                        funct3,
                        word,
                    })
                }
            };

            Instruction::Load {
                kind,
                rd: encoding::rd(word),
                rs1: encoding::rs1(word),
                imm: encoding::imm_i(word),
            }
        }
        encoding::OPCODE_STORE => {
            let kind = match encoding::funct3(word) {
                0b000 => StoreKind::Sb,
                0b001 => StoreKind::Sh,
                0b010 => StoreKind::Sw,
                funct3 => {
                    return Err(ZkvmError::UnsupportedFunct3 {
                        opcode,
                        funct3,
                        word,
                    })
                }
            };

            Instruction::Store {
                kind,
                rs1: encoding::rs1(word),
                rs2: encoding::rs2(word),
                imm: encoding::imm_s(word),
            }
        }
        encoding::OPCODE_OP_IMM => {
            let rd = encoding::rd(word);
            let rs1 = encoding::rs1(word);
            let funct3 = encoding::funct3(word);

            match funct3 {
                0b000 => Instruction::OpImm {
                    kind: OpImmKind::Addi,
                    rd,
                    rs1,
                    imm: encoding::imm_i(word),
                },
                0b010 => Instruction::OpImm {
                    kind: OpImmKind::Slti,
                    rd,
                    rs1,
                    imm: encoding::imm_i(word),
                },
                0b011 => Instruction::OpImm {
                    kind: OpImmKind::Sltiu,
                    rd,
                    rs1,
                    imm: encoding::imm_i(word),
                },
                0b100 => Instruction::OpImm {
                    kind: OpImmKind::Xori,
                    rd,
                    rs1,
                    imm: encoding::imm_i(word),
                },
                0b110 => Instruction::OpImm {
                    kind: OpImmKind::Ori,
                    rd,
                    rs1,
                    imm: encoding::imm_i(word),
                },
                0b111 => Instruction::OpImm {
                    kind: OpImmKind::Andi,
                    rd,
                    rs1,
                    imm: encoding::imm_i(word),
                },
                0b001 => {
                    if encoding::funct7(word) != 0 {
                        return Err(ZkvmError::InvalidShiftEncoding(word));
                    }

                    Instruction::OpImm {
                        kind: OpImmKind::Slli,
                        rd,
                        rs1,
                        imm: encoding::shamt(word) as i32,
                    }
                }
                0b101 => match encoding::funct7(word) {
                    0 => Instruction::OpImm {
                        kind: OpImmKind::Srli,
                        rd,
                        rs1,
                        imm: encoding::shamt(word) as i32,
                    },
                    encoding::FUNCT7_SUB_SRA => Instruction::OpImm {
                        kind: OpImmKind::Srai,
                        rd,
                        rs1,
                        imm: encoding::shamt(word) as i32,
                    },
                    _ => return Err(ZkvmError::InvalidShiftEncoding(word)),
                },
                funct3 => {
                    return Err(ZkvmError::UnsupportedFunct3 {
                        opcode,
                        funct3,
                        word,
                    })
                }
            }
        }
        encoding::OPCODE_OP => {
            let rd = encoding::rd(word);
            let rs1 = encoding::rs1(word);
            let rs2 = encoding::rs2(word);
            let funct3 = encoding::funct3(word);
            let funct7 = encoding::funct7(word);

            let kind = match (funct3, funct7) {
                (0b000, 0x00) => OpKind::Add,
                (0b000, encoding::FUNCT7_SUB_SRA) => OpKind::Sub,
                (0b001, 0x00) => OpKind::Sll,
                (0b010, 0x00) => OpKind::Slt,
                (0b011, 0x00) => OpKind::Sltu,
                (0b100, 0x00) => OpKind::Xor,
                (0b101, 0x00) => OpKind::Srl,
                (0b101, encoding::FUNCT7_SUB_SRA) => OpKind::Sra,
                (0b110, 0x00) => OpKind::Or,
                (0b111, 0x00) => OpKind::And,
                _ => {
                    return Err(ZkvmError::UnsupportedFunct7 {
                        opcode,
                        funct3,
                        funct7,
                        word,
                    })
                }
            };

            Instruction::Op { kind, rd, rs1, rs2 }
        }
        encoding::OPCODE_MISC_MEM => {
            let funct3 = encoding::funct3(word);
            if funct3 != 0 {
                return Err(ZkvmError::UnsupportedFunct3 {
                    opcode,
                    funct3,
                    word,
                });
            }
            if encoding::rd(word) != 0 || encoding::rs1(word) != 0 {
                return Err(ZkvmError::InvalidInstruction(word));
            }

            Instruction::Fence {
                pred: encoding::fence_pred(word),
                succ: encoding::fence_succ(word),
                fm: encoding::fence_fm(word),
            }
        }
        encoding::OPCODE_SYSTEM => {
            let funct3 = encoding::funct3(word);
            if funct3 != 0 {
                return Err(ZkvmError::UnsupportedFunct3 {
                    opcode,
                    funct3,
                    word,
                });
            }
            if encoding::rd(word) != 0 || encoding::rs1(word) != 0 {
                return Err(ZkvmError::InvalidInstruction(word));
            }

            match word >> 20 {
                0 => Instruction::Ecall,
                1 => Instruction::Ebreak,
                _ => return Err(ZkvmError::InvalidInstruction(word)),
            }
        }
        _ => return Err(ZkvmError::UnsupportedOpcode(opcode)),
    };

    Ok(DecodedInstruction::new(word, instruction))
}
