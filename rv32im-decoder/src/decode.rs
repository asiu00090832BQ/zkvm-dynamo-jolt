use crate::{
    error::{DecodeError, Result},
    formats::{funct7, opcode, BType, IType, JType, RType, SType, UType},
    instruction::{BranchKind, Instruction, LoadKind, OpImmKind, OpKind, StoreKind, SystemKind},
    invariants,
};

pub fn decode(raw: u32) -> Result<Instruction> {
    if (raw & 0b11) != 0b11 {
        return Err(DecodeError::InvariantViolation("compressed or non-32-bit instruction"));
    }

    let op = opcode(raw);

    match op {
        0b0110111 => {
            let format = UType::from_raw(raw);
            invariants::validate_utype(&format)?;
            Ok(Instruction::Lui(format))
        }
        0b0010111 => {
            let format = UType::from_raw(raw);
            invariants::validate_utype(&format)?;
            Ok(Instruction::Auipc(format))
        }
        0b1101111 => {
            let format = JType::from_raw(raw);
            invariants::validate_jtype(&format)?;
            Ok(Instruction::Jal(format))
        }
        0b1100111 => {
            let format = IType::from_raw(raw);
            invariants::validate_itype(&format)?;
            if format.funct3 != 0b000 {
                return Err(DecodeError::UnsupportedFunct3 {
                    opcode: op,
                    funct3: format.funct3,
                });
            }
            Ok(Instruction::Jalr(format))
        }
        0b1100011 => {
            let format = BType::from_raw(raw);
            invariants::validate_btype(&format)?;
            let kind = match format.funct3 {
                0b000 => BranchKind::Beq,
                0b001 => BranchKind::Bne,
                0b100 => BranchKind::Blt,
                0b101 => BranchKind::Bge,
                0b110 => BranchKind::Bltu,
                0b111 => BranchKind::Bgeu,
                funct3 => {
                    return Err(DecodeError::UnsupportedFunct3 {
                        opcode: op,
                        funct3,
                    })
                }
            };
            Ok(Instruction::Branch { kind, format })
        }
        0b0000011 => {
            let format = IType::from_raw(raw);
            invariants::validate_itype(&format)?;
            let kind = match format.funct3 {
                0b000 => LoadKind::Lb,
                0b001 => LoadKind::Lh,
                0b010 => LoadKind::Lw,
                0b100 => LoadKind::Lbu,
                0b101 => LoadKind::Lhu,
                funct3 => {
                    return Err(DecodeError::UnsupportedFunct3 {
                        opcode: op,
                        funct3,
                    })
                }
            };
            Ok(Instruction::Load { kind, format })
        }
        0b0100011 => {
            let format = SType::from_raw(raw);
            invariants::validate_stype(&format)?;
            let kind = match format.funct3 {
                0b000 => StoreKind::Sb,
                0b001 => StoreKind::Sh,
                0b010 => StoreKind::Sw,
                funct3 => {
                    return Err(DecodeError::UnsupportedFunct3 {
                        opcode: op,
                        funct3,
                    })
                }
            };
            Ok(Instruction::Store { kind, format })
        }
        0b0010011 => {
            let format = IType::from_raw(raw);
            invariants::validate_itype(&format)?;
            let kind = match format.funct3 {
                0b000 => OpImmKind::Addi,
                0b010 => OpImmKind::Slti,
                0b011 => OpImmKind::Sltiu,
                0b100 => OpImmKind::Xori,
                0b110 => OpImmKind::Ori,
                0b111 => OpImmKind::Andi,
                0b001 => {
                    invariants::ensure_shift_amount(format.shamt())?;
                    let f7 = funct7(raw);
                    if f7 != 0b0000000 {
                        return Err(DecodeError::UnsupportedFunct7 {
                            opcode: op,
                            funct3: format.funct3,
                            funct7: f7,
                        });
                    }
                    OpImmKind::Slli
                }
                0b101 => {
                    invariants::ensure_shift_amount(format.shamt())?;
                    match funct7(raw) {
                        0b0000000 => OpImmKind::Srli,
                        0b0100000 => OpImmKind::Srai,
                        funct7 => {
                            return Err(DecodeError::UnsupportedFunct7 {
                                opcode: op,
                                funct3: format.funct3,
                                funct7,
                            })
                        }
                    }
                }
                funct3 => {
                    return Err(DecodeError::UnsupportedFunct3 {
                        opcode: op,
                        funct3,
                    })
                }
            };
            Ok(Instruction::OpImm { kind, format })
        }
        0b0110011 => {
            let format = RType::from_raw(raw);
            invariants::validate_rtype(&format)?;
            let kind = match format.funct3 {
                0b000 => match format.funct7 {
                    0b0000000 => OpKind::Add,
                    0b0100000 => OpKind::Sub,
                    0b0000001 => OpKind::Mul,
                    funct7 => {
                        return Err(DecodeError::UnsupportedFunct7 {
                            opcode: op,
                            funct3: format.funct3,
                            funct7,
                        })
                    }
                },
                0b001 => match format.funct7 {
                    0b0000000 => OpKind::Sll,
                    0b0000001 => OpKind::Mulh,
                    funct7 => {
                        return Err(DecodeError::UnsupportedFunct7 {
                            opcode: op,
                            funct3: format.funct3,
                            funct7,
                        })
                    }
                },
                0b010 => match format.funct7 {
                    0b0000000 => OpKind::Slt,
                    0b0000001 => OpKind::Mulhsu,
                    funct7 => {
                        return Err(DecodeError::UnsupportedFunct7 {
                            opcode: op,
                            funct3: format.funct3,
                            funct7,
                        })
                    }
                },
                0b011 => match format.funct7 {
                    0b0000000 => OpKind::Sltu,
                    0b0000001 => OpKind::Mulhu,
                    funct7 => {
                        return Err(DecodeError::UnsupportedFunct7 {
                            opcode: op,
                            funct3: format.funct3,
                            funct7,
                        })
                    }
                },
                0b100 => match format.funct7 {
                    0b0000000 => OpKind::Xor,
                    0b0000001 => OpKind::Div,
                    funct7 => {
                        return Err(DecodeError::UnsupportedFunct7 {
                            opcode: op,
                            funct3: format.funct3,
                            funct7,
                        })
                    }
                },
                0b101 => match format.funct7 {
                    0b0000000 => OpKind::Srl,
                    0b0100000 => OpKind::Sra,
                    0b0000001 => OpKind::Divu,
                    funct7 => {
                        return Err(DecodeError::UnsupportedFunct7 {
                            opcode: op,
                            funct3: format.funct3,
                            funct7,
                        })
                    }
                },
                0b110 => match format.funct7 {
                    0b0000000 => OpKind::Or,
                    0b0000001 => OpKind::Rem,
                    funct7 => {
                        return Err(DecodeError::UnsupportedFunct7 {
                            opcode: op,
                            funct3: format.funct3,
                            funct7,
                        })
                    }
                },
                0b111 => match format.funct7 {
                    0b0000000 => OpKind::And,
                    0b0000001 => OpKind::Remu,
                    funct7 => {
                        return Err(DecodeError::UnsupportedFunct7 {
                            opcode: op,
                            funct3: format.funct3,
                            funct7,
                        })
                    }
                },
                funct3 => {
                    return Err(DecodeError::UnsupportedFunct3 {
                        opcode: op,
                        funct3,
                    })
                }
            };
            Ok(Instruction::Op { kind, format })
        }
        0b0001111 => Ok(Instruction::Fence),
        0b1110011 => match raw {
            0x0000_0073 => Ok(Instruction::System(SystemKind::Ecall)),
            0x0010_0073 => Ok(Instruction::System(SystemKind::Ebreak)),
            _ => Err(DecodeError::InvalidSystemEncoding(raw)),
        },
        _ => Err(DecodeError::UnsupportedOpcode(op)),
    }
}
