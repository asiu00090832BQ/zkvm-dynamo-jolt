pub mod m_extension;

use crate::error::ZkvmError;
use crate::isa::i::{Ecall, Rv32I, Sub};
use crate::isa::{
    BTypeFields, ITypeFields, Instruction, JTypeFields, RTypeFields, Register, STypeFields,
    ShiftImmFields, UTypeFields,
};

pub use m_extension::{execute_rv32m, split16};

pub fn decode_word(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = opcode(word);

    match opcode {
        0b0110111 => Ok(Instruction::I(Rv32I::Lui(UTypeFields {
            rd: rd(word)?,
            imm: word & 0xffff_f000,
        }))),
        0b0010111 => Ok(Instruction::I(Rv32I::Auipc(UTypeFields {
            rd: rd(word)?,
            imm: word & 0xffff_f000,
        }))),
        0b1101111 => Ok(Instruction::I(Rv32I::Jal(JTypeFields {
            rd: rd(word)?,
            imm: decode_j_imm(word),
        }))),
        0b1100111 => {
            let funct3 = funct3(word);
            if funct3 != 0 {
                return Err(ZkvmError::InvalidFunct3 { opcode, funct3, word });
            }
            Ok(Instruction::I(Rv32I::Jalr(decode_i_fields(word)?)))
        }
        0b1100011 => {
            let fields = decode_b_fields(word)?;
            let inst = match fields.funct3 {
                0b000 => Rv32I::Beq(fields),
                0b001 => Rv32I::Bne(fields),
                0b100 => Rv32I::Blt(fields),
                0b101 => Rv32I::Bge(fields),
                0b110 => Rv32I::Bltu(fields),
                0b111 => Rv32I::Bgeu(fields),
                value => {
                    return Err(ZkvmError::InvalidFunct3 {
                        opcode,
                        funct3: value,
                        word,
                    })
                }
            };
            Ok(Instruction::I(inst))
        }
        0b0000011 => {
            let fields = decode_i_fields(word)?;
            let inst = match fields.funct3 {
                0b000 => Rv32I::Lb(fields),
                0b001 => Rv32I::Lh(fields),
                0b010 => Rv32I::Lw(fields),
                0b100 => Rv32I::Lbu(fields),
                0b101 => Rv32I::Lhu(fields),
                value => {
                    return Err(ZkvmError::InvalidFunct3 {
                        opcode,
                        funct3: value,
                        word,
                    })
                }
            };
            Ok(Instruction::I(inst))
        }
        0b0100011 => {
            let fields = decode_s_fields(word)?;
            let inst = match fields.funct3 {
                0b000 => Rv32I::Sb(fields),
                0b001 => Rv32I::Sh(fields),
                0b010 => Rv32I::Sw(fields),
                value => {
                    return Err(ZkvmError::InvalidFunct3 {
                        opcode,
                        funct3: value,
                        word,
                    })
                }
            };
            Ok(Instruction::I(inst))
        }
        0b0010011 => decode_op_imm(word, opcode),
        0b0110011 => decode_op(word, opcode),
        0b0001111 => {
            let inst = match funct3(word) {
                0b000 => Rv32I::Fence,
                0b001 => Rv32I::FenceI,
                value => {
                    return Err(ZkvmError::InvalidFunct3 {
                        opcode,
                        funct3: value,
                        word,
                    })
                }
            };
            Ok(Instruction::I(inst))
        }
        0b1110011 => decode_system(word),
        _ => Err(ZkvmError::InvalidOpcode { opcode, word }),
    }
}

fn decode_op_imm(word: u32, opcode: u8) -> Result<Instruction, ZkvmError> {
    let funct3 = funct3(word);
    let fields = decode_i_fields(word)?;

    let inst = match funct3 {
        0b000 => Rv32I::Addi(fields),
        0b010 => Rv32I::Slti(fields),
        0b011 => Rv32I::Sltiu(fields),
        0b100 => Rv32I::Xori(fields),
        0b110 => Rv32I::Ori(fields),
        0b111 => Rv32I::Andi(fields),
        0b001 => {
            let shift = decode_shift_imm_fields(word)?;
            if shift.funct7 != 0 {
                return Err(ZkvmError::InvalidFunct7 {
                    opcode,
                    funct7: shift.funct7,
                    word,
                });
            }
            Rv32I::Slli(shift)
        }
        0b101 => {
            let shift = decode_shift_imm_fields(word)?;
            match shift.funct7 {
                0b0000000 => Rv32I::Srli(shift),
                0b0100000 => Rv32I::Srai(shift),
                value => {
                    return Err(ZkvmError::InvalidFunct7 {
                        opcode,
                        funct7: value,
                        word,
                    })
                }
            }
        }
        value => {
            return Err(ZkvmError::InvalidFunct3 {
                opcode,
                funct3: value,
                word,
            })
        }
    };

    Ok(Instruction::I(inst))
}

fn decode_op(word: u32, opcode: u8) -> Result<Instruction, ZkvmError> {
    let fields = decode_r_fields(word)?;

    match fields.funct7 {
        0b0000000 => {
            let inst = match fields.funct3 {
                0b000 => Rv32I::Add(fields),
                0b001 => Rv32I::Sll(fields),
                0b010 => Rv32I::Slt(fields),
                0b011 => Rv32I::Sltu(fields),
                0b100 => Rv32I::Xor(fields),
                0b101 => Rv32I::Srl(fields),
                0b110 => Rv32I::Or(fields),
                0b111 => Rv32I::And(fields),
                value => {
                    return Err(ZkvmError::InvalidFunct3 {
                        opcode,
                        funct3: value,
                        word,
                    })
                }
            };
            Ok(Instruction::I(inst))
        }
        0b0100000 => {
            let inst = match fields.funct3 {
                0b000 => Rv32I::Sub(Sub {
                    rd: fields.rd,
                    rs1: fields.rs1,
                    rs2: fields.rs2,
                }),
                0b101 => Rv32I::Sra(fields),
                value => {
                    return Err(ZkvmError::InvalidFunct3 {
                        opcode,
                        funct3: value,
                        word,
                    })
                }
            };
            Ok(Instruction::I(inst))
        }
        0b0000001 => Ok(Instruction::M(m_extension::decode_rv32m(fields, opcode, word)?)),
        value => Err(ZkvmError::InvalidFunct7 {
            opcode,
            funct7: value,
            word,
        }),
    }
}

fn decode_system(word: u32) -> Result<Instruction, ZkvmError> {
    match word {
        0x0000_0073 => Ok(Instruction::I(Rv32I::Ecall(Ecall))),
        0x0010_0073 => Ok(Instruction::I(Rv32I::Ebreak)),
        _ => Err(ZkvmError::UnsupportedInstruction { word }),
    }
}

pub(crate) const fn opcode(word: u32) -> u8 {
    (word & 0x7f) as u8
}

pub(crate) const fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x7) as u8
}

pub(crate) const fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7f) as u8
}

pub(crate) fn rd(word: u32) -> Result<Register, ZkvmError> {
    register(((word >> 7) & 0x1f) as u8)
}

pub(crate) fn rs1(word: u32) -> Result<Register, ZkvmError> {
    register(((word >> 15) & 0x1f) as u8)
}

pub(crate) fn rs2(word: u32) -> Result<Register, ZkvmError> {
    register(((word >> 20) & 0x1f) as u8)
}

pub(crate) fn register(index: u8) -> Result<Register, ZkvmError> {
    Register::new(index)
}

pub(crate) const fn sign_extend(value: u32, width: u8) -> i32 {
    let shift = 32 - width as u32;
    ((value << shift) as i32) >> shift
}

pub(crate) fn decode_r_fields(word: u32) -> Result<RTypeFields, ZkvmError> {
    Ok(RTypeFields {
        rd: rd(word)?,
        rs1: rs1(word)?,
        rs2: rs2(word)?,
        funct3: funct3(word),
        funct7: funct7(word),
    })
}

pub(crate) fn decode_i_fields(word: u32) -> Result<ITypeFields, ZkvmError> {
    Ok(ITypeFields {
        rd: rd(word)?,
        rs1: rs1(word)?,
        imm: sign_extend(word >> 20, 12),
        funct3: funct3(word),
    })
}

pub(crate) fn decode_s_fields(word: u32) -> Result<STypeFields, ZkvmError> {
    let imm = ((word >> 7) & 0x1f) | (((word >> 25) & 0x7f) << 5);
    Ok(STypeFields {
        rs1: rs1(word)?,
        rs2: rs2(word)?,
        imm: sign_extend(imm, 12),
        funct3: funct3(word),
    })
}

pub(crate) fn decode_b_fields(word: u32) -> Result<BTypeFields, ZkvmError> {
    let imm = (((word >> 8) & 0x0f) << 1)
        | (((word >> 25) & 0x3f) << 5)
        | (((word >> 7) & 0x01) << 11)
        | (((word >> 31) & 0x01) << 12);

    Ok(BTypeFields {
        rs1: rs1(word)?,
        rs2: rs2(word)?,
        imm: sign_extend(imm, 13),
        funct3: funct3(word),
    })
}

pub(crate) const fn decode_j_imm(word: u32) -> i32 {
    let imm = (((word >> 21) & 0x03ff) << 1)
        | (((word >> 20) & 0x0001) << 11)
        | (((word >> 12) & 0x00ff) << 12)
        | (((word >> 31) & 0x0001) << 20);

    sign_extend(imm, 21)
}

pub(crate) fn decode_shift_imm_fields(word: u32) -> Result<ShiftImmFields, ZkvmError> {
    let shamt = ((word >> 20) & 0x1f) as u8;

    Ok(ShiftImmFields {
        rd: rd(word)?,
        rs1: rs1(word)?,
        shamt,
        funct3: funct3(word),
        funct7: funct7(word),
    })
}
