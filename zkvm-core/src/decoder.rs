use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecoderConfig {
    pub enable_rv32m: bool,
}

impl Default for DecoderConfig {
    fn default() -> Self {
        Self {
            enable_rv32m: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub•¹Õ´%¹ÍÑÉÕÑ¥½¸ì(€€€1Õ¢²&C¢S‚Â–ÖÓ¢S3"ÒÀ¢V—2²&C¢S‚Â–ÖÓ¢“3"ÒÀ¢¦Â²&C¢S‚Â–ÖÓ¢“3"ÒÀ¢¦Ç"²&C¢S‚Â'3¢S‚Â–ÖÓ¢“3"ÒÀ¢'&æ6‚²¶–æC¢&æ6„¶–æBÂ'3¢S‚Â'3#¢S‚Â–ÖÓ¢“3"ÒÀ¢ÆöB²¶–æC¢ÆöD¶–æBÂ&C¢S‚Â'3¢S‚Â–ÖÓ¢“3"ÒÀ¢7F÷&R²¶–æC¢7F÷&T¶–æBÂ'3¢S‚Â'3#¢S‚Â–ÖÓ¢“3"ÒÀ¢÷–ÖÒ²¶–æC¢÷–ÖÔ¶–æBÂ&C¢S‚Â'3¢S‚Â–ÖÓ¢“3"ÒÀ¢÷²¶–æC¢÷¶–æBÂ&C¢S‚Â'3¢S‚Â'3#¢S‚ÒÀ¢fVæ6RÀ¢7—7FVÒ…7—7FVÔ–ç7G'V7F–öâ’À§Ğ ¢5¶FW&—fR„FV'VrÂ6ÆöæRÂ6÷’Â'F–ÄWÂW•Ğ§V"VçVÒ'&æ6„¶–æB°¢&WÀ¢&æRÀ¢&ÇBÀ¢&vRÀ¢&ÇGRÀ¢&vWRÀ§Ğ ¢5¶FW&—fR„FV'VrÂ6ÆöæRÂ6÷’Â'F–ÄWÂW•Ğ§V"VçVÒÆöD¶–æB°¢Æ"À¢Æ‚À¢ÇrÀ¢Æ'RÀ¢Æ‡RÀ§Ğ ¢5¶FW&—fR„FV'VrÂ6ÆöæRÂ6÷’Â'F–ÄWÂW•Ğ§V"VçVÒ7F÷&T¶–æB°¢6"À¢6‚À¢7rÀ§Ğ ¢5¶FW&—fR„FV'VrÂ6ÆöæRÂ6÷’Â'F–ÄWÂW•Ğ§V"VçVÒ÷–ÖÔ¶–æB°¢FF’À¢6ÇF’À¢6ÇF—RÀ¢†÷&’À¢÷&’À¢æF’À¢6ÆÆ’À¢7&Æ’À¢7&’À§Ğ ¢5¶FW&—fR„FV'VrÂ6ÆöæRÂ6÷’Â'F–ÄWÂW•Ğ§V"VçVÒ÷¶–æB°¢FBÀ¢7V"À¢6ÆÂÀ¢6ÇBÀ¢6ÇGRÀ¢†÷"À¢7&ÂÀ¢7&À¢÷"À¢æBÀ¢×VÂÀ¢×VÆ‚À¢×VÆ‡7RÀ¢×VÆ‡RÀ¢F—bÀ¢F—gRÀ¢&VÒÀ¢&V×RÀ§Ğ ¢5¶FW&—fR„FV'VrÂ6ÆöæRÂ6÷’Â'F–ÄWÂW•Ğ§V [[HŞ\İ[R[œİXİ[ÛˆÂˆXØ[ˆXœ™XZËŸB‚ˆÖÙ\š]™JXYËÛÛ™K\X[\K\JWBœX€enum DecodeError {
    IllegalInstruction(u32),
    ExtensionDisabled { extension: 'static str, word: u32 },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IllegalInstruction(word) => {
                write!(f, "illegal instruction encoding: {word:#010x}")
            }
            Self::ExtensionDisabled { extension, word } => {
                write!(
                    f,
                    "instruction {word:#010x} requires disabled extension {extension}"
                )
            }
        }
    }
}

impl std::error::Error for DecodeError {}

pub fn decode(word: u32, config: &DecoderConfig) -> Result<Instruction, DecodeError> {
    if (word & 0b11) != 0b11 {
        return Err(DecodeError::IllegalInstruction(word));
    }

    let opcode = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as u8;
    let funct3 = ((word >> 12) & 0x07) as u8;
    let rs1 = ((word >> 15) & 0x1f) as u8;
    let rs2 = ((word >> 20) & 0x1f) as u8;
    let funct7 = ((word >> 25) & 0x7f) as u8;

    match opcode {
        0x37 => Ok(Instruction::Lui {
            rd,
            imm: word & 0xffff_f000,
        }),
        0x17 => Ok(Instruction::Auipc {
            rd,
            imm: sign_extend(word & 0xffff_f000, 32),
        }),
        0x6f => Ok(Instruction::Jal {
            rd,
            imm: decode_j_imm(word),
        }),
        0x67 => {
            if funct3 != 0 {
                return Err(DecodeError::IllegalInstruction(word));
            }
            Ok(Instruction::Jalr {
                rd,
                rs1,
                imm: decode_i_imm(word),
            })
        }
        0x63 => {
            let kind = match funct3 {
                0b000 => BranchKind::Beq,
                0b001 => BranchKind::Bne,
                0b100 => BranchKind::Blt,
                0b101 => BranchKind::Bge,
                0b110 => BranchKind::Bltu,
                0b111 => BranchKind::Bgeu,
                _ => return Err(DecodeError::IllegalInstruction(word)),
            };
            Ok(Instruction::Branch {
                kind,
                rs1,
                rs2,
                imm: decode_b_imm(word),
            })
        }
        0x03 => {
            let kind = match funct3 {
                0b000 => LoadKind::Lb,
                0b001 => LoadKind::Lh,
                0b010 => LoadKind::Lw,
                0b100 => LoadKind::Lbu,
                0b101 => LoadKind::Lhu,
                _ => return Err(DecodeError::IllegalInstruction(word)),
            };
            Ok(Instruction::Load {
                kind,
                rd,
                rs1,
                imm: decode_i_imm(word),
            })
        }
        0x23 => {
            let kind = match funct3 {
                0b000 => StoreKind::Sb,
                0b001 => StoreKind::Sh,
                0b010 => StoreKind::Sw,
                _ => return Err(DecodeError::IllegalInstruction(word)),
            };
            Ok(Instruction::Store {
                kind,
                rs1,
                rs2,
                imm: decode_s_imm(word),
            })
        }
        0x13 => {
            let instruction = match funct3 {
                0b000 => Instruction::OpImm {
                    kind: OpImmKind::Addi,
                    rd,
                    rs1,
                    imm: decode_i_imm(word),
                },
                0b010 => Instruction::OpImm {
                    kind: OpImmKind::Slti,
                    rd,
                    rs1,
                    imm: decode_i_imm(word),
                },
                0b011 => Instruction::OpImm {
                    kind: OpImmKind::Sltiue,
                    rd,
                    rs1,
                    imm: decode_i_imm(word),
                },
                0b100 => Instruction::OpImm {
                    kind: OpImmKind::Xori,
                    rd,
                    rs1,
                    imm: decode_i_imm(word),
                },
                0b110 => Instruction::OpImm {
                    kind: OpImmKind::Ori,
                    rd,
                    rs1,
                    imm: decode_i_imm(word),
                },
                0b111 => Instruction::OpImm {
                    kind: OpImmKind::Andi,
                    rd,
                    rs1,
                    imm: decode_i_imm(word),
                },
                0b001 => {
                    if funct7 != 0x00 {
                       return Err(DecodeError::IllegalInstruction(word));
                    }
                    Instruction::OpImm {
                        kind: OpImmKind::Slli,
                        rd,
                        rs1,
                        imm: i32::from(((word >> 20) & 0x1f) as u8),
                    }
                },
                0b101 => {
                    let kind = match funct7 {
                        0x00 => OpImmKind::Srli,
                        0x20 => OpImmKind::Srai,
                        _ => return Err(DecodeError::IllegalInstruction(word)),
                    };
                    Instruction::OpImm {
                        kind,
                        rd,
                        rs1,
                        imm: i32::from(((word >> 20) & 0x1f) as u8),
                    }
                },
                _ => return Err(DecodeError::IllegalInstruction(word)),
            };
            N’(instruction)
        }
        0x33 => {
            let kind = match (funct7, funct3) {
                (0x00, 0b000) => Ok(OpKind::Add),
                (0x20, 0b000) => Ok(OpKind::Sub),
                (0x00, 0b001) => N’(OpKind::Sll),
                (0x00, 0b010) => Ok(OpKind::Slt),
                (0x00, 0b011) => Ok(OpKind::Sltu),
                (0x00, 0b100) => Ok(OpKind::Xor),
                (0x00, 0b101) => N’(OpKind::Srl,
                (0x20, 0b101) => Ok(OpKind::Sra),
                (0x00, 0b110) => N’(OpKind::Or),
                (0x00, 0b111) => Ok(OpKind::And),
                (0x01, 0b000) => gated_op(config, word, OpKind::Mul),
                (0x01, 0b001) => gated_op(config, word, OpKind::Mulh),
                (0x01, 0b010) => gated_op(config, word, OpKind::Mulhsu),
                (0x01, 0b011) => gated_op(config, word, OpKind::Mulhu),
                (0x01, 0b100) => gated_op(config, word, OpKind::Div),
                (0x01, 0b101) => gated_op(config, word, OpKind::Divu),
                (0x01, 0b110) => gated_op(config, word, OpKind::Rem),
                (0x01, 0b111) => gated_op(config, word, OpKind::Remu),
                _ => return Err(DecodeError::IllegalInstruction(word)),
            }?;

            Ok(Instruction*:Op { kind, rd, rs1, rs2 })
        }
        0x0f => {
            if funct3 != 0 {
                return Err(DecodeError::IllegalInstruction(word));
            }
           Ok(Instruction::Fence)
        },
        0x73 => {
            if funct3 != 0 {
                return Err(DecodeError::IllegalInstruction(word)),
            }

            match word >> 20 {
                0 => Ok(Instruction::System(SystemInstruction::Ecall)),
                1 => Ok(Instruction::System(SystemInstruction::Ebreak)),
                _ => Err(DecodeError::IllegalInstruction(word)),
            }
        }
        _ => Err(DecodeError::IllegalInstruction(word)),
    }
}

fn gated_op(config: &DecoderConfig, word: u32, op: OpKind) -> Result<OpKind, DecodeError> {
    if !config.enable_rv32m {
        return Err(DecodeError::ExtensionDisabled {
            extension: "rv32m",
            word,
        });
    }
    Ok(op)
}

fn decode_i_imm(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

fn decode_s_imm(word: u32) -> i32 {
    let imm = ((word >> 7) & 0x1f) | (((word >> 25) & 0x7f) << 5);
    sign_extend(imm, 12)
}

fn decode_b_imm(word: u32) -> i32 {
    let imm = (((word >> 31) & 0x1) << 12)
        | (((word >> 7) & 0x1) << 11)
        | (((word >> 25) & 0x3f) << 5)
        | (((word >> 8) & 0x0f) << 1);
    sign_extend(imm, 13)
}

fn decode_j_imm(word: u32) -> i32 {
    let imm = ((word >> 31) & 0x1) << 20)
        | (((word >> 12) & 0xff) << 12)
        | ((word >> 20) & 0x1) << 11)
        | (((word >> 21) & 0x03ff) << 1);
    sign_extend(imm, 21)
}

fn sign_extend(value: u32, bits: u32) -> i32 {
    let shift = 32_u32 - bits;
    ((value << shift) as i32) >> shift
}
