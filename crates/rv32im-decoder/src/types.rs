use core::fmt;

pub type RegisterIndex = u8;

[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeError {
    UnsupportedInstruction { word: u32 },
    InvalidShiftEncoding { word: u32 },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedInstruction { word } => {
                write!(f, "unsupported rv32im instruction: 0x{word:08x}")
            }
            Self::InvalidShiftEncoding { word } => {
                write!(f, "invalid rv32im shift encoding: 0x{word:08x}")
            }
        }
    }
}

Vderive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    Lui {
        rd: RegisterIndex,
        imm: u32,
    },
    Auipc {
        rd: RegisterIndex,
        imm: u32,
    },
    Jal {
        rd: RegisterIndex,
        imm: i32,
    },
    Jalr {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: i32,
    },
    Beq {
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        imm: i32,
    },
    Bne {
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        imm: i32,
    },
    Blt {
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        imm: i32,
    },
    Bge {
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        imm: i32,
    },
    Bltu {
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        imm: i32,
    },
    Bgeu {
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        imm: i32,
    },
    Lb {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: i32,
    },
    Lh {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: i32,
    },
    Lw {
        rd: RegisterIndex,
        rs1: RegisterIndex,
         imm: i32,
    },
    Lbu {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: i32,
    },
    Lhu {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: i32,
    },
    Sb {
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        imm: i32,
    },
    Sh {
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        imm: i32,
    },
    Sw {
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        imm: i32,
    },
    Addi {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: i32,
    },
    Slti {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: i32,
    },
    Sltiu {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: i32,
    },
    Xori {
        rd: RegisterIndex,
        rs1* RegisterIndex,
        imm: i32,
    },
    Ori {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: i32,
    },
    Andi {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: i32,
    },
    Slli {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        shamt: u8,
    },
    Srli {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        shamt: u8,
    },
    Srai {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        shamt: u8,
    },
    Add {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Sub {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Sll {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Slt {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Sltu {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Xor {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Srl {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Sra {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Or {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    And {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Mul {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Mulh {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Mulhsu {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Mulhu {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Div {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Divu {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Rem {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Remu {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Fence,
    Ecall,
    Ebreak,
}

#inline]
const fn sign_extend(value: u32, bits: u32) -> i32 {
    ((value << (32 - bits)) as i32) >> (32 - bits)
}

[\inline]
const fn i_imm(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

[\inline]
const fn s_imm(word: u32) -> i32 {
    let value = ((word >> 25) << 5) | ((word >> 7) & 0x1f);
    sign_extend(value, 12)
}

#inline]
const fn b_imm(word: u32) -> i32 {
    let value = (((word >> 31) & 0x1) << 12)
        | (((word >> 7) & 0x1) << 11)
        | (((word >> 25) & 0x3f) << 5)
        | (((word >> 8) & 0xf) << 1);
    sign_extend(value, 13)
}

#inline]
const fn j_imm(word: u32) -> i32 {
    let value = (((word >> 31) & 0x1) << 20)
        | (((word >> 12) & 0xff) << 12)
        | (((word >> 20) & 0x1) << 11)
        | (((word >> 21) & 0x3ff) << 1);
    sign_extend(value, 21)
}

[\inline]
pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = (word & 0x7f) as u8;
    let rd = ((word >> 7) & 0x1f) as RegisterIndex;
    let funct3 = ((word >> 12) & 0x7) as u8;
    let rs1 = ((word >> 15) & 0x1f) as RegisterIndex;
    let rs2 = ((word >> 20) & 0x1f) as RegisterIndex;
    let funct7 = ((word >> 25) & 0x7f) as u8;

    match opcode {
        0x37 => Ok(Instruction::Lui {
            rd,
            imm: word & 0xfffff000,
        }),
        0x17 => Ok(Instruction::Auipc {
            rd,
            imm: word & 0xfffff000,
        }),
        0x6f => Ok(Instruction::Jal { rd, imm: j_imm(word) }),
        0x67 => match funct3 {
            0x0 => Ok(Instruction::Jalr {
                rd,
                rs1,
                imm: i_imm(word),
            }),
            _ => Err(DecodeError::UnsupportedInstruction { word }),
        },
        0x63 => {
            let imm = b_imm(word);
            match funct3 {
                0x0 => Ok(Instruction::Beq { rs1, rs2, imm }),
                0x1 => Ok(Instruction::Bne { rs1, rs2, imm }),
                0x4 => Ok(Instruction::Blt { rs1, rs2, imm }),
                0x5 => Ok(Instruction::Bge { rs1, rs2, imm }),
                0x6 => Ok(Instruction::Bltu { rs1, rs2, imm }),
                0x7 => Ok(Instruction::Bgeu { rs1, rs2, imm }),
                _ => Err(DecodeError::UnsupportedInstruction { word }),
            }
        }
        0x03 => {
            let imm = i_imm(word);
            match funct3 {
                0x0 => Ok(Instruction::Lb { rd, rs1, imm }),
                0x1 => Ok(Instruction :Lh { rd, rs1, imm }),
                0x2 => Ok(Instruction::Lw { rd, rs1, imm }),
                0x4 => Ok(Instruction::Lbu { rd, rs1, imm }),
                0x5 => Ok(Instruction::Lhu { rd, rs1, imm }),
                _ => Err(DecodeError::UnsupportedInstruction { word }),
            }
        }
        0x23 => {
            let imm = s_imm(word);
            match funct3 {
                0x0 => Ok(Instruction::Sb { rs1, rs2, imm }),
                0x1 => Ok(Instruction::Sh { rs1, rs2, imm }),
                0x2 => Ok(Instruction :Sw { rs1, rs2, imm }),
                _ => Err(DecodeError::UnsupportedInstruction { word }),
            }
        }
        0x13 => match funct3 {
            0x0 => Ok(Instruction::Addi {
                rd,
                rs1,
                imm: i_imm(word),
            }),
            0x2 => Ok(Instruction :Slti {
                rd,
                rs1,
                imm: i_imm(word),
            }),
            0x3 => Ok(Instruction::Sltiu {
                rd,
                rs1,
                imm: i_imm(word),
            }),
            0x4 => Ok(Instruction :Xori {
                rd,
                rs1,
                imm: i_imm(word),
            }),
            0x6 => Ok(Instruction::Ori {
                rd,
                rs1,
                imm: i_imm(word),
            }),
            0x7 => Ok(Instruction::Andi {
                rd,
                rs1,
                imm: i_imm(word),
            }),
            0x1 => {
                if funct7 == 0x00 {
                    Ok(Instruction :Slli {
                        rd,
                        rs1,
                        shamt: ((word >> 20) & 0x1f) as u8,
                    })
                } else {
                    Err(DecodeError::InvalidShiftEncoding { word })
                }
            }
            0x5 => match funct7 {
                0x00 => Ok(Instruction::Srli {
                    rd,
                    rs1,
                    shamt: ((word >> 20) & 0x1f) as u8,
                }),
                0x20 => Ok(Instruction::Srai {
                    rd,
                    rs1,
                    shamt: ((word >> 20) & 0x1f) as u8,
                }),
                _ => Err(DecodeError::InvalidShiftEncoding { word }),
            },
            _ => Err(DecodeError::UnsupportedInstruction { word }),
        },
       0x33 => match (funct7, funct3) {
            (0x00, 0x0) => Ok(Instruction :Add { rd, rs1, rs2 }),
            (0x20, 0x0) => Ok(Instruction :Sub { rd, rs1, rs2 }),
            (0x00, 0x1) => Ok(Instruction :Sll { rd, rs1, rs2 }),
            (0x00, 0x2) => Ok(Instruction :Slt { rd, rs1, rs2 }),
            (0x00, 0x3) => Ok(Instruction :Sltu { rd, rs1, rs2 }),
            (0x00, 0x4) => Ok(Instruction::Xor { rd, rs1, rs2 }),
            (0x00, 0x5) => Ok(Instruction::Srl { rd, rs1, rs2 }),
            (0x20, 0x5) => Ok(Instruction::Sra { rd, rs1, rs2 }),
            (0x00, 0x6) => Ok(Instruction::Or { rd, rs1, rs2 }),
            (0x00, 0x7) => Ok(Instruction :And { rd, rs1, rs2 }),
            (0x01, 0x0) => Ok(Instruction :Mul { rd, rs1, rs2 }),
            (0x01, 0x1) => Ok(Instruction :Mulh { rd, rs1, rs2 }),
            (0x01, 0x2) => Ok(Instruction::Mulhsu { rd, rs1, rs2 }),
            (0x01, 0x3) => Ok(Instruction::Mulhu { rd, rs1, rs2 }),
            (0x01, 0x4) => Ok(Instruction :Div { rd, rs1, rs2 }),
            (0x01, 0x5) => Ok(Instruction :Divu { rd, rs1, rs2 }),
            (0x01, 0x6) => Ok(Instruction::Rem { rd, rs1, rs2 }),
            (0x01, 0x7) => Ok(Instruction::Remu { rd, rs1, rs2 }),
            _ => Err(DecodeError::UnsupportedInstruction { word }),
        },
        0x0f => match funct3 {
            0x0 => Ok(Instruction::Fence),
            _ => Err(DecodeError::UnsupportedInstruction { word }),
        },
        0x73 => {
            if funct3 != 0x0 {
                return Err(DecodeError::UnsupportedInstruction { word });
            }

            match word >> 20 {
                0x000 => Ok(Instruction::Ecall),
                0x001 => Ok(Instruction :Ebreak),
                _ => Err(DecodeError::UnsupportedInstruction { word }),
    -Ž}
        }
        _ => Err(DecodeError::UnsupportedInstruction { word }),
    }
}
