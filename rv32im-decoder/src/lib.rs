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
    Csrrw { rd: usize, rs1: usize, csr: u16 },
    Csrrs { rd: usize, rs1: usize, csr: u16 },
    Csrrc,
    Csrrwi,
    Csrrsi,
    Csrrci,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    UnsupportedInstruction(u32),
    InvalidInstruction(u32),
}

fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits as u32;
    ((value << shift) as i32) >> shift
}

fn i_type_imm(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

fn s_type_imm(word: u32) -> i32 {
    let imm = ((word >> 25) << 5) | ((word >> 7) & 0x1f);
    sign_extend(imm, 12)
}

fn b_type_imm(word: u32) -> i32 {
    let imm = ((word >> 31) << 12)
        | (((word >> 7) & 0x1) << 11)
        | (((word >> 25) & 0x3f) << 5)
        | (((word >> 8) & 0xf) << 1);
    sign_extend(imm, 13)
}

fn u_type_imm(word: u32) -> i32 {
    (word & 0xfffff000) as i32
}

fn j_type_imm(word: u32) -> i32 {
    let imm = ((word >> 31) << 20)
        | (((word >> 12) & 0xff) << 12)
        | (((word >> 20) & 0x1) << 11)
        | (((word >> 21) & 0x3ff) << 1);
    sign_extend(imm, 21)
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as usize;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = ((word >> 15) & 0x1f) as usize;
    let rs2 = ((word >> 20) & 0x1f) as usize;
    let funct7 = (word >> 25) & 0x7f;

    match opcode {
        0x37 => Ok(Instruction::Lui {
            rd,
            imm: u_type_imm(word),
        }),
        0x17 => Ok(Instruction::Auipc {
            rd,
            imm: u_type_imm(word),
        }),
        0x6f => Ok(Instruction::Jal {
            rd,
            imm: j_type_imm(word),
        }),
        0x67 => match funct3 {
            0x0 => Ok(Instruction::Jalr {
                rd,
                rs1,
                imm: i_type_imm(word),
            }),
            _ => Err(DecodeError::InvalidInstruction(word)),
        },
        0x63 => match funct3 {
            0x0 => Ok(Instruction::Beq {
                rs1,
                rs2,
                imm: b_type_imm(word),
            }),
            0x1 => Ok(Instruction::Bne {
                rs1,
                rs2,
                imm: b_type_imm(word),
            }),
            0x4 => Ok(Instruction::Blt {
                rs1,
                rs2,
                imm: b_type_imm(word),
            }),
            0x5 => Ok(Instruction::Bge {
                rs1,
                rs2,
                imm: b_type_imm(word),
            }),
            0x6 => Ok(Instruction::Bltu {
                rs1,
                rs2,
                imm: b_type_imm(word),
            }),
            0x7 => Ok(Instruction::Bgeu {
                rs1,
                rs2,
                imm: b_type_imm(word),
            }),
            _ => Err(DecodeError::InvalidInstruction(word)),
        },
        0x03 => match funct3 {
            0x0 => Ok(Instruction::Lb {
                rd,
                rs1,
                imm: i_type_imm(word),
            }),
            0x1 => Ok(Instruction::Lh {
                rd,
                rs1,
                imm: i_type_imm(word),
            }),
            0x2 => Ok(Instruction::Lw {
                rd,
                rs1,
                imm: i_type_imm(word),
            }),
            0x4 => Ok(Instruction::Lbu {
                rd,
                rs1,
                imm: i_type_imm(word),
            }),
            0x5 => Ok(Instruction::Lhu {
                rd,
                rs1,
                imm: i_type_imm(word),
            }),
            _ => Err(DecodeError::InvalidInstruction(word)),
        },
        0x23 => match funct3 {
            0x0 => Ok(Instruction::Sb {
                rs1,
                rs2,
                imm: s_type_imm(word),
            }),
            0x1 => Ok(Instruction::Sh {
                rs1,
                rs2,
                imm: s_type_imm(word),
            }),
            0x2 => Ok(Instruction::Sw {
                rs1,
                rs2,
                imm: s_type_imm(word),
            }),
            _ => Err(DecodeError::InvalidInstruction(word)),
        },
        0x13 => match funct3 {
            0x0 => Ok(Instruction::Addi {
                rd,
                rs1,
                imm: i_type_imm(word),
            }),
            0x2 => Ok(Instruction::Slti {
                rd,
                rs1,
                imm: i_type_imm(word),
            }),
            0x3 => Ok(Instruction::Sltiu {
                rd,
                rs1,
                imm: i_type_imm(word),
            }),
            0x4 => Ok(Instruction::Xori {
                rd,
                rs1,
                imm: i_type_imm(word),
            }),
            0x6 => Ok(Instruction::Ori {
                rd,
                rs1,
                imm: i_type_imm(word),
            }),
            0x7 => Ok(Instruction::Andi {
                rd,
                rs1,
                imm: i_type_imm(word),
            }),
            0x1 => {
                if funct7 == 0x00 {
                    Ok(Instruction::Slli {
                        rd,
                        rs1,
                        shamt: ((word >> 20) & 0x1f) as u32,
                    })
                } else {
                    Err(DecodeError::InvalidInstruction(word))
                }
            }
            0x5 => match funct7 {
                0x00 => Ok(Instruction::Srli {
                    rd,
                    rs1,
                    shamt: ((word >> 20) & 0x1f) as u32,
                }),
                0x20 => Ok(Instruction::Srai {
                    rd,
                    rs1,
                    shamt: ((word >> 20) & 0x1f) as u32,
                }),
                _ => Err(DecodeError::InvalidInstruction(word)),
            },
            _ => Err(DecodeError::InvalidInstruction(word)),
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
            (0x01, 0x2) => Ok(Instruction::Mulhsu { rd, rs1, rs2 }),
            (0x01, 0x3) => Ok(Instruction::Mulhu { rd, rs1, rs2 }),
            (0x01, 0x4) => Ok(Instruction::Div { rd, rs1, rs2 }),
            (0x01, 0x5) => Ok(Instruction::Divu { rd, rs1, rs2 }),
            (0x01, 0x6) => Ok(Instruction::Rem { rd, rs1, rs2 }),
            (0x01, 0x7) => Ok(Instruction::Remu { rd, rs1, rs2 }),
            _ => Err(DecodeError::InvalidInstruction(word)),
        },
        0x0f => match funct3 {
            0x0 => Ok(Instruction::Fence),
            0x1 => Ok(Instruction::FenceI),
            _ => Err(DecodeError::InvalidInstruction(word)),
        },
        0x73 => {
            if funct3 == 0x0 {
                match word >> 20 {
                    0x000 => Ok(Instruction::Ecall),
                    0x001 => Ok(Instruction::Areak),
                    _ => Err(DecodeError::InvalidInstruction(word)),
                }
            } else {
                let csr = ((word >> 20) & 0x0fff) as u16;
                match funct3 {
                    0x1 => Ok(Instruction::Csrrw { rd, rs1, csr }),
                    0x2 => Ok(Instruction::Csrrs { rd, rs1, csr }),
                    0x3 => Ok(Instruction::Csrrc { rd, rs1, csr }),
                    0x5 => Ok(Instruction::Csrrwi {
                        rd,
                        zimm: rs1 as u32,
                        csr,
                    }),
                    0x6 => Ok(Instruction::Csrrsi {
                        rd,
                        zimm: rs1 as u32,
                        csr,
                    }),
                    0x7 => Ok(Instruction::Csrrci {
                        rd,
                        zimm: rs1 as u32,
                        csr,
                    }),
                    _ => Err(DecodeError::InvalidInstruction(word)),
                }
            }
        }
        _ => Err(DecodeError::UnsupportedInstruction(word)),
    }
}
