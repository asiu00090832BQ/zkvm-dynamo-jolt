use crate::error::DecodeError;
use crate::instruction::{
    BTypeFields, ITypeFields, Instruction, JTypeFields, RTypeFields, STypeFields,
    ShiftImmFields, UTypeFields,
};
use crate::m_extension::decode_m_extension;

#[derive(Debug, Default, Clone, Copy)]
pub struct Decoder;

impl Decoder {
    pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
        decode(word)
    }
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    if word == 0 {
        return Err(DecodeError::IllegalInstruction { word });
    }

    match opcode(word) {
        0x37 => Ok(Instruction::Lui(UTypeFields::new(rd(word), u_imm(word)))),
        0x17 => Ok(Instruction::Auipc(UTypeFields::new(rd(word), u_imm(word)))),
        0x6f => Ok(Instruction::Jal(JTypeFields::new(rd(word), j_imm(word)))),
        0x67 => decode_jalr(word),
        0x63 => decode_branch(word),
        0x03 => decode_load(word),
        0x23 => decode_store(word),
        0x13 => decode_op_imm(word),
        0x33 => decode_op(word),
        0x0f => decode_fence(word),
        0x73 => decode_system(word),
        other => Err(DecodeError::UnsupportedOpcode {
            word,
            opcode: other,
        }),
    }
}

fn decode_jalr(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = opcode(word);
    let funct3 = funct3(word);

    if funct3 != 0 {
        return Err(DecodeError::UnsupportedFunct3 {
            word,
            opcode,
            funct3,
        });
    }

    Ok(Instruction::Jalr(ITypeFields::new(
        rd(word),
        rs1(word),
        i_imm(word),
    )))
}

fn decode_branch(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = opcode(word);
    let fields = BTypeFields::new(rs1(word), rs2(word), b_imm(word));

    match funct3(word) {
        0b000 => Ok(Instruction::Beq(fields)),
        0b001 => Ok(Instruction::Bne(fields)),
        0b100 => Ok(Instruction::Blt(fields)),
        0b101 => Ok(Instruction::Bge(fields)),
        0b110 => Ok(Instruction::Bltu(fields)),
        0b111 => Ok(Instruction::Bgeu(fields)),
        funct3 => Err(DecodeError::UnsupportedFunct3 {
            word,
            opcode,
            funct3,
        }),
    }
}

fn decode_load(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = opcode(word);
    let fields = ITypeFields::new(rd(word), rs1(word), i_imm(word));

    match funct3(word) {
        0b000 => Ok(Instruction::Lb(fields)),
        0b001 => Ok(Instruction::Lh(fields)),
        0b010 => Ok(Instruction::Lw(fields)),
        0b100 => Ok(Instruction::Lbu(fields)),
        0b101 => Ok(Instruction::Lhu(fields)),
        funct3 => Err(DecodeError::UnsupportedFunct3 {
            word,
            opcode,
            funct3,
        }),
    }
}

fn decode_store(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = opcode(word);
    let fields = STypeFields::new(rs1(word), rs2(word), s_imm(word));

    match funct3(word) {
        0b000 => Ok(Instruction::Sb(fields)),
        0b001 => Ok(Instruction::Sh(fields)),
        0b010 => Ok(Instruction::Sw(fields)),
        funct3 => Err(DecodeError::UnsupportedFunct3 {
            word,
            opcode,
            funct3,
        }),
    }
}

fn decode_op_imm(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = opcode(word);
    let funct3 = funct3(word);

    match funct3 {
        0b000 => Ok(Instruction::Addi(ITypeFields::new(
            rd(word),
            rs1(word),
            i_imm(word),
        ))),
        0b010 => Ok(Instruction::Slti(ITypeFields::new(
            rd(word),
            rs1(word),
            i_imm(word),
        ))),
        0b011 => Ok(Instruction::Sltiu(ITypeFields::new(
            rd(word),
            rs1(word),
            i_imm(word),
        ))),
        0b100 => Ok(Instruction::Xori(ITypeFields::new(
            rd(word),
            rs1(word),
            i_imm(word),
        ))),
        0b110 => Ok(Instruction::Ori(ITypeFields::new(
            rd(word),
            rs1(word),
            i_imm(word),
        ))),
        0b111 => Ok(Instruction::Andi(ITypeFields::new(
            rd(word),
            rs1(word),
            i_imm(word),
        ))),
        0b001 => {
            let funct7 = funct7(word);
            if funct7 != 0 {
                return Err(DecodeError::UnsupportedFunct7 {
                    word,
                    opcode,
                    funct3,
                    funct7,
                });
            }

            Ok(Instruction::Slli(ShiftImmFields::new(
                rd(word),
                rs1(word),
                shamt(word),
            )))
        }
        0b101 => match funct7(word) {
            0b0000000 => Ok(Instruction::Srli(ShiftImmFields::new(
                rd(word),
                rs1(word),
                shamt(word),
            ))),
            0b0100000 => Ok(Instruction::Srai(ShiftImmFields::new(
                rd(word),
                rs1(word),
                shamt(word),
            ))),
            funct7 => Err(DecodeError::UnsupportedFunct7 {
                word,
                opcode,
                funct3,
                funct7,
            }),
        },
        funct3 => Err(DecodeError::UnsupportedFunct3 {
            word,
            opcode,
            funct3,
        }),
    }
}

fn decode_op(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = opcode(word);
    let funct3 = funct3(word);
    let funct7 = funct7(word);
    let fields = RTypeFields::new(rd(word), rs1(word), rs2(word));

    match funct7 {
        0b0000000 => match funct3 {
            0b000 => Ok(Instruction::Add(fields)),
            0b001 => Ok(Instruction::Sll(fields)),
            0b010 => Ok(Instruction::Slt(fields)),
            0b011 => Ok(Instruction::Sltu(fields)),
            0b100 => Ok(Instruction::Xor(fields)),
            0b101 => Ok(Instruction::Srl(fields)),
            0b110 => Ok(Instruction::Or(fields)),
            0b111 => Ok(Instruction::And(fields)),
            funct3 => Err(DecodeError::UnsupportedFunct3 {
                word,
                opcode,
                funct3,
            }),
        },
        0b0100000 => match funct3 {
            0b000 => Ok(Instruction::Sub(fields)),
            0b101 => Ok(Instruction::Sra(fields)),
            funct3 => Err(DecodeError::UnsupportedFunct3 {
                word,
                opcode,
                funct3,
            }),
        },
        0b0000001 => decode_m_extension(word, funct3, fields),
        funct7 => Err(DecodeError::UnsupportedFunct7 {
            word,
            opcode,
            funct3,
            funct7,
        }),
    }
}

fn decode_fence(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = opcode(word);

    match funct3(word) {
        0b000 => Ok(Instruction::Fence),
        0b001 => Ok(Instruction::FenceI),
        funct3 => Err(DecodeError::UnsupportedFunct3 {
            word,
            opcode,
            funct3,
        }),
    }
}

fn decode_system(word: u32) -> Result<Instruction, DecodeError> {
    let funct3 = funct3(word);

    if funct3 != 0 {
        return Err(DecodeError::UnsupportedSystem {
            word,
            funct3,
            csr: ((word >> 20) & 0x0fff) as u16,
        });
    }

    match word {
        0x0000_0073 => Ok(Instruction::Ecall),
        0x0010_0073 => Ok(Instruction::Ebreak),
        _ => Err(DecodeError::ReservedEncoding { word }),
    }
}

const fn opcode(word: u32) -> u8 {
    (word & 0x7f) as u8
}

const fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1f) as u8
}

const fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x07) as u8
}

const fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}

const fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

const fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7f) as u8
}

const fn shamt(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

fn i_imm(word: u32) -> i32 {
    sign_extend((word >> 20) & 0x0fff, 12)
}

fn s_imm(word: u32) -> i32 {
    let imm = ((word >> 7) & 0x1f) | (((word >> 25) & 0x7f) << 5);
    sign_extend(imm, 12)
}

fn b_imm(word: u32) -> i32 {
    let imm = (((word >> 31) & 0x1) << 12)
        | (((word >> 7) & 0x1) << 11)
        | (((word >> 25) & 0x3f) << 5)
        | (((word >> 8) & 0x0f) << 1);
    sign_extend(imm, 13)
}

fn u_imm(word: u32) -> i32 {
    (word & 0xffff_f000) as i32
}

fn j_imm(word: u32) -> i32 {
    let imm = (((word >> 31) & 0x1) << 20)
        | (((word >> 12) & 0xff) << 12)
        | (((word >> 20) & 0x1) << 11)
        | (((word >> 21) & 0x03ff) << 1);
    sign_extend(imm, 21)
}

fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - u32::from(bits);
    ((value << shift) as i32) >> shift
}
