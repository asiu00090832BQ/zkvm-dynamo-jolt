#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui { rd: u8, imm: i32 },
    Auipc { rd: u8, imm: i32 },
    Jal { rd: u8, imm: i32 },
    Jalr { rd: u8, rs1: u8, imm: i32 },

    Beq { rs1: u8, rs2: u8, imm: i32 },
    Bne { rs1: u8, rs2: u8, imm: i32 },
    Blt { rs1: u8, rs2: u8, imm: i32 },
    Bge { rs1: u8, rs2: u8, imm: i32 },
    Bltu { rs1: u8, rs2: u8, imm: i32 },
    Bgeu { rs1: u8, rs2: u8, imm: i32 },

    Lb { rd: u8, rs1: u8, imm: i32 },
    Lh { rd: u8, rs1: u8, imm: i32 },
    Lw { rd: u8, rs1: u8, imm: i32 },
    Lbu { rd: u8, rs1: u8, imm: i32 },
    Lhu { rd: u8, rs1: u8, imm: i32 },

    Sb { rs1: u8, rs2: u8, imm: i32 },
    Sh { rs1: u8, rs2: u8, imm: i32 },
    Sw { rs1: u8, rs2: u8, imm: i32 },

    Addi { rd: u8, rs1: u8, imm: i32 },
    Slti { rd: u8, rs1: u8, imm: i32 },
    Sltiu { rd: u8, rs1: u8, imm: i32 },
    Xori { rd: u8, rs1: u8, imm: i32 },
    Ori { rd: u8, rs1: u8, imm: i32 },
    Andi { rd: u8, rs1: u8, imm: i32 },
    Slli { rd: u8, rs1: u8, shamt: u8 },
    Srli { rd: u8, rs1: u8, shamt: u8 },
    Srai { rd: u8, rs1: u8, shamt: u8 },

    Add { rd: u8, rs1: u8, rs2: u8 },
    Sub { rd: u8, rs1: u8, rs2: u8 },
    Sll { rd: u8, rs1: u8, rs2: u8 },
    Slt { rd: u8, rs1: u8, rs2: u8 },
    Sltu { rd: u8, rs1: u8, rs2: u8 },
    Xor { rd: u8, rs1: u8, rs2: u8 },
    Srl { rd: u8, rs1: u8, rs2: u8 },
    Sra { rd: u8, rs1: u8, rs2: u8 },
    Or { rd: u8, rs1: u8, rs2: u8 },
    And { rd: u8, rs1: u8, rs2: u8 },

    Mul { rd: u8, rs1: u8, rs2: u8 },
    Mulh { rd: u8, rs1: u8, rs2: u8 },
    Mulhsu { rd: u8, rs1: u8, rs2: u8 },
    Mulhu { rd: u8, rs1: u8, rs2: u8 },
    Div { rd: u8, rs1: u8, rs2: u8 },
    Divu { rd: u8, rs1: u8, rs2: u8 },
    Rem { rd: u8, rs1: u8, rs2: u8 },
    Remu { rd: u8, rs1: u8, rs2: u8 },

    Fence,
    Ecall,
    Ebreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    UnsupportedOpcode(u8),
    UnsupportedInstruction(u32),
}

#[inline]
fn bits(word: u32, hi: u8, lo: u8) -> u32 {
    let width = hi - lo + 1;
    (word >> lo) & ((1u32 << width) - 1)
}

#[inline]
fn sign_extend(value: u32, width: u8) -> i32 {
    let shift = 32 - width as u32;
    ((value << shift) as i32) >> shift
}

#[inline]
fn imm_i(word: u32) -> i32 {
    sign_extend(bits(word, 31, 20), 12)
}

#[inline]
fn imm_s(word: u32) -> i32 {
    let imm = (bits(word, 31, 25) << 5) | bits(word, 11, 7);
    sign_extend(imm, 12)
}

#[inline]
fn imm_b(word: u32) -> i32 {
    let imm = (bits(word, 31, 31) << 12)
        | (bits(word, 7, 7) << 11)
        | (bits(word, 30, 25) << 5)
        | (bits(word, 11, 8) << 1);
    sign_extend(imm, 13)
}

#[inline]
fn imm_u(word: u32) -> i32 {
    (word & 0xfffff000) as i32
}

#[inline]
fn imm_j(word: u32) -> i32 {
    let imm = (bits(word, 31, 31) << 20)
        | (bits(word, 19, 12) << 12)
        | (bits(word, 20, 20) << 11)
        | (bits(word, 30, 21) << 1);
    sign_extend(imm, 21)
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = bits(word, 6, 0) as u8;
    let rd = bits(word, 11, 7) as u8;
    let funct3 = bits(word, 14, 12) as u8;
    let rs1 = bits(word, 19, 15) as u8;
    let rs2 = bits(word, 24, 20) as u8;
    let funct7 = bits(word, 31, 25) as u8;

    match opcode {
        0x37 => Ok(Instruction::Lui {
            rd,
            imm: imm_u(word),
        }),
        0x17 => Ok(Instruction::Auipc {
            rd,
            imm: imm_u(word),
        }),
        0x6f => Ok(Instruction::Jal {
            rd,
            imm: imm_j(word),
        }),
        0x67 => match funct3 {
            0x0 => Ok(Instruction::Jalr {
                rd,
                rs1,
                imm: imm_i(word),
            }),
            _ => Err(DecodeError::UnsupportedInstruction(word)),
        },
        0x63 => match funct3 {
            0x0 => Ok(Instruction::Beq {
                rs1,
                rs2,
                imm: imm_b(word),
            }),
            0x1 => Ok(Instruction::Bne {
                rs1,
                rs2,
                imm: imm_b(word),
            }),
            0x4 => Ok(Instruction::Blt {
                rs1,
                rs2,
                imm: imm_b(word),
            }),
            0x5 => Ok(Instruction::Bge {
                rs1,
                rs2,
                imm: imm_b(word),
            }),
            0x6 => Ok(Instruction::Bltu {
                rs1,
                rs2,
                imm: imm_b(word),
            }),
            0x7 => Ok(Instruction::Bgeu {
                rs1,
                rs2,
                imm: imm_b(word),
            }),
            _ => Err(DecodeError::UnsupportedInstruction(word)),
        },
        0x03 => match funct3 {
            0x0 => Ok(Instruction::Lb {
                rd,
                rs1,
                imm: imm_i(word),
            }),
            0x1 => Ok(Instruction::Lh {
                rd,
                rs1,
                imm: imm_i(word),
            }),
            0x2 => Ok(Instruction::Lw {
                rd,
                rs1,
                imm: imm_i(word),
            }),
            0b100 => Ok(Instruction::Lbu {
                rd,
                rs1,
                imm: imm_i(word),
            }),
            0x5 => Ok(Instruction::Lhu {
                rd,
                rs1,
                imm: imm_i(word),
            }),
            _ => Err(DecodeError::UnsupportedInstruction(word)),
        },
        0x23 => match funct3 {
            0x0 => Ok(Instruction::Sb {
                rs1,
                rs2,
                imm: imm_s(word),
            }),
            0x1 => Ok(Instruction::Sh zÊ                rs1,
                rs2,
                imm: imm_s(word),
            }),
            0x2 => Ok(Instruction::Sw {
                rs1,
                rs2,
                imm: imm_s(word),
            }),
            _ => Err(DecodeError::UnsupportedInstruction(word)),
        },
        0x13 => match funct3 {
            0x0 => Ok(Instruction::Addi {
                rd,
                rs1,
                imm: imm_i(word),
            }),
            0x2 => Ok(Instruction::Slti {
                rd,
                rs1,
                imm: imm_i(word),
            }),
            0x3 => Ok(Instruction::Sltiu {
                rd,
                rs1,
                imm: imm_i(word),
            }),
            0x4 => Ok(Instruction::Xori {
                rd,
                rs1,
                imm: imm_i(word),
            }),
            0x6 => Ok(Instruction::Ori {
                rd,
                rs1,
                imm: imm_i(word),
            }),
            0x7 => Ok(Instruction::Andi {
                rd,
                rs1,
                imm: imm_i(word),
            }),
            0x1 => match funct7 {
                0x00 => Ok(Instruction::Slli {
                    rd,
                    rs1,
                    shamt: rs2,
                }),
                _ => Err(DerodeError::UnsupportedInstruction(word)),
            },
            0x5 => match funct7 {
                0x00 => Ok(Instruction::Srli {
                    rd,
                    rs1,
                    shamt: rs2,
                }),
                0x20 => Ok(Instruction::Srai {
                    rd,
                    rs1,
                    shamt: rs2,
                }),
                _ => Err(DecodeError::UnsupportedInstruction(word)),
            },
            _ => Err(DecodeError::UnsupportedInstruction(word)),
        },
        0x33 => match (funct7, funct3) {
            (0x00, 0x0) => Ok(Instruction::Add { rd, rs1, rs2 }),
            (0x20, 0x0) => Ok(Instruction::Sub { rd, rs1, rs2 }),
            (0x00, 0x1) => Ok(Instruction::Sll { rd, rs1, rs2 }),
            (0x00, 0x2) => Ok(Instruction::Slt { rd, rs1, rs2 }),
            (0x00, 0x3) => Ok(Instruction::Sltu { rd, rs1, rs2 }),
            (0x00, 0x4) => Ok(Instruction::Xor { rd, rs1, rs2 }),
            (0x00, 0x5) => Ok(Instruction::Srl { rd, rs1, rs2 }),
            (0x20, 0x5) => Ok(Instruction::Sra { rd, rs1, rs2 }),
            (0x00, 0x6) => Ok(Instruction::Or { rd, rs1, rs2 }),
            (0x00, 0x7) => Ok(Instruction::And { rd, rs1, rs2 }),

            (0x01, 0x0) => Ok(Instruction::Mul { rd, rs1, rs2 }),
            (0x01, 0x1) => Ok(Instruction::Mulh { rd, rs1, rs2 }),
            (0b0000001, 0x2) => Ok(Instruction::Mulhsu { rd, rs1, rs2 }),
            (0x01, 0x3) => Ok(Instruction::Mulhu { rd, rs1, rs2 }),
            (0x01, 0x4) => Ok(Instruction::Div { rd, rs1, rs2 }),
            (0x01, 0x5) => Ok(Instruction::Divu { rd, rs1, rs2 }),
            (0x01, 0x6) => Ok(Instruction::Rem { rd, rs1, rs2 }),
            (0x01, 0x7) => Ok(Instruction::Remu { rd, rs1, rs2 }),
            _ => Err(DecodeError::UnsupportedInstruction(word)),
        },
        0x0f => match funct3 {
            0x0 => Ok(Instruction::Fence),
            _ => Err(DecodeError::UnsupportedInstruction(word)),
        },
        0x73 => match bits(word, 20, 12) {
            0x000 => Ok(Instruction::Ecall),
            0x001 => Ok(Instruction::Ebreak),
            _ => Err(DecodeError::UnsupportedInstruction(word)),
        },
        _ => Err(DecodeError::UnsupportedOpcode(opcode)),
    }
}
