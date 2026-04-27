use crate::vm::ZkvmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui { rd: usize, imm: i32 },
    Auipc { rd: usize, imm: i32 },
    Jal { rd: usize, imm: i32 },
    Jalr { rd: usize, rs1: usize, imm: i32 },

    Beq { rs1: usize, rs2: usize, imm: i32 },
    Bne { rs1: usize, rs2: usize, imm: i32 },
    Blt { rs1: usize, rs2: usize, imm: i32 },
    Bge { rs1: usize, rs2: usize, imm: i32 },
    Bltu { rs1: usize, rs2: usize, imm: i32 },
    Bgeu { rs1: usize, rs2: usize, imm: i32 },

    Lb { rd: usize, rs1: usize, imm: i32 },
    Lh { rd: usize, rs1: usize, imm: i32 },
    Lw { rd: usize, rs1: usize, imm: i32 },
    Lbu { rd: usize, rs1: usize, imm: i32 },
    Lhu { rd: usize, rs1: usize, imm: i32 },

    Sb { rs1: usize, rs2: usize, imm: i32 },
    Sh { rs1: usize, rs2: usize, imm: i32 },
    Sw { rs1: usize, rs2: usize, imm: i32 },

    Addi { rd: usize, rs1: usize, imm: i32 },
    Slti { rd: usize, rs1: usize, imm: i32 },
    Sltiu { rd: usize, rs1: usize, imm: i32 },
    Xori { rd: usize, rs1: usize, imm: i32 },
    Ori { rd: usize, rs1: usize, imm: i32 },
    Andi { rd: usize, rs1: usize, imm: i32 },
    Slli { rd: usize, rs1: usize, shamt: u32 },
    Srli { rd: usize, rs1: usize, shamt: u32 },
    Srai { rd: usize, rs1: usize, shamt: u32 },

    Add { rd: usize, rs1: usize, rs2: usize },
    Sub { rd: usize, rs1: usize, rs2: usize },
    Sll { rd: usize, rs1: usize, rs2: usize },
    Slt { rd: usize, rs1: usize, rs2: usize },
    Sltu { rd: usize, rs1: usize, rs2: usize },
    Xor { rd: usize, rs1: usize, rs2: usize },
    Srl { rd: usize, rs1: usize, rs2: usize },
    Sra { rd: usize, rs1: usize, rs2: usize },
    Or { rd: usize, rs1: usize, rs2: usize },
    And { rd: usize, rs1: usize, rs2: usize },

    Mul { rd: usize, rs1: usize, rs2: usize },
    Mulh { rd: usize, rs1: usize, rs2: usize },
    Mulhsu { rd: usize, rs1: usize, rs2: usize },
    Mulhu { rd: usize, rs1: usize, rs2: usize },
    Div { rd: usize, rs1: usize, rs2: usize },
    Divu { rd: usize, rs1: usize, rs2: usize },
    Rem { rd: usize, rs1: usize, rs2: usize },
    Remu { rd: usize, rs1: usize, rs2: usize },

    Fence,
    FenceI,
    Ecall,
    Ebreak,
}

#[inline]
pub fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits;
    ((value << shift) as i32) >> shift
}

#[inline]
pub fn imm_i(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

#[inline]
pub fn imm_s(word: u32) -> i32 {
    let imm = ((word >> 7) & 0x1f) | (((word >> 25) & 0x7f) << 5);
    sign_extend(imm, 12)
}

#[inline]
pub fn imm_b(word: u32) -> i32 {
    let imm = (((word >> 31) & 0x1) << 12)
        | (((word >> 7) & 0x1) << 11)
        | (((word >> 25) & 0x3f) << 5)
        | (((word >> 8) & 0x0f) << 1);
    sign_extend(imm, 13)
}

#[inline]
pub fn imm_u(word: u32) -> i32 {
    (word & 0xfffff000) as i32
}

#[inline]
pub fn imm_j(word: u32) -> i32 {
    let imm = (((word >> 31) & 0x1) << 20)
        | (((word >> 12) & 0xff) << 12)
        | (((word >> 20) & 0x1) << 11)
        | (((word >> 21) & 0x03ff) << 1);
    sign_extend(imm, 21)
}

#[inline]
fn rd(word: u32) -> usize {
    ((word >> 7) & 0x1f) as usize
}

#[inline]
fn rs1(word: u32) -> usize {
    ((word >> 15) & 0x1f) as usize
}

#[inline]
fn rs2(word: u32) -> usize {
    ((word >> 20) & 0x1f) as usize
}

pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = word & 0x7f;
    let funct3 = (word >> 12) & 0x7;
    let funct7 = (word >> 25) & 0x7f;

    let rd = rd(word);
    let rs1 = rs1(word);
    let rs2 = rs2(word);

    match opcode {
        0b0110111 => Ok(Instruction::Lui { rd, imm: imm_u(word) }),
        0b0010111 => Ok(Instruction::Auipc { rd, imm: imm_u(word) }),
        0b1101111 => Ok(Instruction::Jal { rd, imm: imm_j(word) }),
        0b1100111 => match funct3 {
            0b000 => Ok(Instruction::Jalr {
                rd,
                rs1,
                imm: imm_i(word),
            }),
            _ => Err(ZkvmError::InvalidInstruction(word)),
        },
        0b1100011 => {
            let imm = imm_b(word);
            match funct3 {
                0b000 => Ok(Instruction::Beq { rs1, rs2, imm }),
                0b001 => Ok(Instruction::Bne { rs1, rs2, imm }),
                0b100 => Ok(Instruction::Blt { rs1, rs2, imm }),
               0b101 => Ok(Instruction::Bge { rs1, rs2, imm }),
                0b110 => Ok(Instruction::Bltu { rs1, rs2, imm }),
                0b111 => Ok(Instruction::Bgeu { rs1, rs2, imm }),
                _ => Err(ZkvmError::InvalidInstruction(word)),
            }
        }
        0b0000011 => {
            let imm = imm_i(word);
            match funct3 {
                0b000 => Ok(Instruction::Lb { rd, rs1, imm }),
                0b001 => Ok(Instruction::Lh { rd, rs1, imm }),
                0b010 => Ok(Instruction::Lw { rd, rs1, imm }),
                0b100 => Ok(Instruction::Lbu { rd, rs1, imm }),
               0b101 => Ok(Instruction::Lhu { rd, rs1, imm }),
                _ => Err(ZkvmError::InvalidInstruction(word)),
            }
        }
        0b0100011 => {
            let imm = imm_s(word);
            match funct3 {
                0b000 => Ok(Instruction::Sb { rs1, rs2, imm }),
                0b001 => Ok(Instruction::Sh { rs1, rs2, imm }),
               0b010 => Ok(Instruction::Sw { rs1, rs2, imm }),
                _ => Err(ZkvmError::InvalidInstruction(word)),
            }
        }
        0b0010011 => {
            let imm = imm_i(word);
            match funct3 {
                0b000 => Ok(Instruction::Addi { rd, rs1, imm }),
               0b010 => Ok(Instruction::Slti { rd, rs1, imm }),
                0b011 => Ok(Instruction::Sltiu { rd, rs1, imm }),
                0b100 => Ok(Instruction::Xori { rd, rs1, imm }),
               0b110 => Ok(Instruction::Ori { rd, rs1, imm }),
               0b111 => Ok(Instruction::Andi { rd, rs1, imm }),
                0b001 => {
                    if funct7 == 0b0000000 {
                        Ok(Instruction::Slli {
                            rd,
                            rs1,
                            shamt: (word >> 20) & 0x1f,
                        })
                    } else {
                        Err(ZkvmError::InvalidInstruction(word))
                    }
                }
                0b101 => match funct7 {
                    0b0000000 => Ok(Instruction::Srli {
                        rd,
                        rs1,
                        shamt: (word >> 20) & 0x1f,
                    }),
                    0b0100000 => Ok(Instruction::Srai {
                        rd,
                        rs1,
                        shamt: (word >> 20) & 0x1f,
                    }),
                    _ => Err(ZkvmError::InvalidInstruction(word)),
                },
                _ => Err(ZkvmError::InvalidInstruction(word)),
            }
        }
        0b0110011 => match (funct7, funct3) {
            (0b0000000, 0b000) => Ok(Instruction::Add { rd, rs1, rs2 }),
            (0b0100000, 0b000) => Ok(Instruction::Sub { rd, rs1, rs2 }),
            (0b0000000, 0b001) => Ok(Instruction::Sll { rd, rs1, rs2 }),
            (0b0000000, 0b010) => Ok(Instruction::Slt { rd, rs1, rs2 }),
            (0b0000000, 0b011) => Ok(Instruction::Sltu { rd, rs1, rs2 }),
            (0b0000000, 0b100) => Ok(Instruction::Xor { rd, rs1, rs2 }),
            (0b0000000, 0b101) => Ok(Instruction::Srl { rd, rs1, rs2 }),
            (0b0100000, 0b101) => Ok(Instruction::Sra { rd, rs1, rs2 }),
            (0b0000000, 0b110) => Ok(Instruction::Or { rd, rs1, rs2 }),
            (0b0000000, 0b111) => Ok(Instruction::And { rd, rs1, rs2 }),

            (0b0000001, 0b000) => Ok(Instruction::Mul { rd, rs1, rs2 }),
            (0b0000001, 0b001) => Ok(Instruction::Mulh { rd, rs1, rs2 }),
            (0b0000001, 0b010) => Ok(Instruction::Mulhsu { rd, rs1, rs2 }),
            (0b0000001, 0b011) => Ok(Instruction::Mulhu { rd, rs1, rs2 }),
            (0b0000001, 0b100) => Ok(Instruction::Div { rd, rs1, rs2 }),
            (0b0000001, 0b101) => Ok(Instruction::Divu { rd, rs1, rs2 }),
            (0b0000001, 0b110) => Ok(Instruction::Rem { rd, rs1, rs2 }),
            (0b0000001, 0b111) => Ok(Instruction::Remu { rd, rs1, rs2 }),
            _ => Err(ZkvmError::InvalidInstruction(word)),
        },
        0b0001111 => match funct3 {
            0b000 => Ok(Instruction::Fence),
            0b001 => Ok(Instruction::FenceI),
            _ => Err(ZkvmError::InvalidInstruction(word)),
        },
        0b1110011 => match (funct3, word >> 20) {
            (0b000, 0x000) => Ok(Instruction::Ecall),
            (0b000, 0x001) => Ok(Instruction::Ebreak),
            _ => Err(ZkvmError::UnsupportedInstruction(word)),
        },
        _ => Err(ZkvmError::InvalidInstruction(word)),
    }
}

pub fn decode_bytes(bytes: [u8; 4]) -> Result<Instruction, ZkvmError> {
    decode(u32::from_le_bytes(bytes))
}

pub fn decode_instruction(word: u32) -> Result<Instruction, ZkvmError> {
    decode(word)
}