#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    LUI { rd: usize, imm: i32 },
    AUIPC { rd: usize, imm: i32 },
    JAL { rd: usize, imm: i32 },
    JALR { rd: usize, rs1: usize, imm: i32 },
    BEQ { rs1: usize, rs2: usize, imm: i32 },
    BNE { rs1: usize, rs2: usize, imm: i32 },
    BLT { rs1: usize, rs2: usize, imm: i32 },
    BGE { rs1: usize, rs2: usize, imm: i32 },
    BLTU { rs1: usize, rs2: usize, imm: i32 },
    BGEU { rs1: usize, rs2: usize, imm: i32 },
    LB { rd: usize, rs1: usize, imm: i32 },
    LH { rd: usize, rs1: usize, imm: i32 },
    LW { rd: usize, rs1: usize, imm: i32 },
    LBU { rd: usize, rs1: usize, imm: i32 },
    LHU { rd: usize, rs1: usize, imm: i32 },
    SB { rs1: usize, rs2: usize, imm: i32 },
    SH { rs1: usize, rs2: usize, imm: i32 },
    SW { rs1: usize, rs2: usize, imm: i32 },
    ADDI { rd: usize, rs1: usize, imm: i32 },
    SLTI { rd: usize, rs1: usize, imm: i32 },
    SLTIU { rd: usize, rs1: usize, imm: i32 },
    XORI { rd: usize, rs1: usize, imm: i32 },
    ORI { rd: usize, rs1: usize, imm: i32 },
    ANDI { rd: usize, rs1: usize, imm: i32 },
    SLLI { rd: usize, rs1: usize, shamt: u32 },
    SRLI { rd: usize, rs1: usize, shamt: u32 },
    SRAI { rd: usize, rs1: usize, shamt: u32 },
    ADD { rd: usize, rs1: usize, rs2: usize },
    SUB { rd: usize, rs1: usize, rs2: usize },
    SLL { rd: usize, rs1: usize, rs2: usize },
    SLT { rd: usize, rs1: usize, rs2: usize },
    SLTU { rd: usize, rs1: usize, rs2: usize },
    XOR { rd: usize, rs1: usize, rs2: usize },
    SRL { rd: usize, rs1: usize, rs2: usize },
    SRA { rd: usize, rs1: usize, rs2: usize },
    OR { rd: usize, rs1: usize, rs2: usize },
    AND { rd: usize, rs1: usize, rs2: usize },
    MUL { rd: usize, rs1: usize, rs2: usize },
    MULH { rd: usize, rs1: usize, rs2: usize },
    MULHSU { rd: usize, rs1: usize, rs2: usize },
    MULHU { rd: usize, rs1: usize, rs2: usize },
    DIV { rd: usize, rs1: usize, rs2: usize },
    DIVU { rd: usize, rs1: usize, rs2: usize },
    REM { rd: usize, rs1: usize, rs2: usize },
    REMU { rd: usize, rs1: usize, rs2: usize },
    FENCE,
    FENCEI,
    ECALL,
    EBREAK,
    INVALID(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Invalid(u32),
    Unsupported(u32),
}

fn sext(value: u32, bits: u32) -> i32 {
    let shift = 32 - bits;
    ((value << shift) as i32) >> shift
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as usize;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = ((word >> 15) & 0x1f) as usize;
    let rs2 = ((word >> 20) & 0x1f) as usize;
    let funct7 = (word >> 25) & 0x7f;

    match opcode {
        0x37 => {
            let imm = (word & 0xfffff000) as i32;
            Ok(Instruction::LUI { rd, imm })
        }
        0x17 => {
            let imm = (word & 0xfffff000) as i32;
            Ok(Instruction::AUIPC { rd, imm })
        }
        0x6f => {
            let imm20 = ((word >> 31) & 0x1) << 20;
            let imm10_1 = ((word >> 21) & 0x3ff) << 1;
            let imm11 = ((word >> 20) & 0x1) << 11;
            let imm19_12 = ((word >> 12) & 0xff) << 12;
            let imm_u = imm20 | imm19_12 | imm11 | imm10_1;
            let imm = sext(imm_u, 21);
            Ok(Instruction::JAL { rd, imm })
        }
        0x67 => {
            let imm_u = (word >> 20) & 0xfff;
            let imm = sext(imm_u, 12);
            Ok(Instruction::JALR { rd, rs1, imm })
        }
        0x63 => {
            let imm12 = ((word >> 31) & 0x1) << 12;
            let imm10_5 = ((word >> 25) & 0x3f) << 5;
            let imm4_1 = ((word >> 8) & 0x0f) << 1;
            let imm11 = ((word >> 7) & 0x1) << 11;
            let imm_u = imm12 | imm11 | imm10_5 | imm4_1;
            let imm = sext(imm_u, 13);
            match funct3 {
                0x0 => Ok(Instruction::BEQ { rs1, rs2, imm }),
                0x1 => Ok(Instruction::BNE { rs1, rs2, imm }),
                0x4 => Ok(Instruction::BLT { rs1, rs2, imm }),
                0x5 => Ok(Instruction::BGE { rs1, rs2, imm }),
                0x6 => Ok(Instruction::BLTU { rs1, rs2, imm }),
                0x7 => Ok(Instruction::BGEU { rs1, rs2, imm }),
                _ => Err(DecodeError::Invalid(word)),
            }
        }
        0x03 => {
            let imm = sext((word >> 20) & 0xfff, 12);
            match funct3 {
                0x0 => Ok(Instruction::LB { rd, rs1, imm }),
                0x1 => Ok(Instruction::LH { rd, rs1, imm }),
                0x2 => Ok(Instruction::LW { rd, rs1, imm }),
                0x4 => Ok(Instruction::LBU { rd, rs1, imm }),
                0x5 => Ok(Instruction::LHU { rd, rs1, imm }),
                _ => Err(DecodeError::Invalid(word)),
            }
        }
        0x23 => {
            let imm11_5 = (word >> 25) & 0x7f;
            let imm4_0 = (word >> 7) & 0x1f;
            let imm = sext((imm11_5 << 5) | imm4_0, 12);
            match funct3 {
                0x0 => Ok(Instruction::SB { rs1, rs2, imm }),
                0x1 => Ok(Instruction::SH { rs1, rs2, imm }),
                0x2 => Ok(Instruction::SW { rs1, rs2, imm }),
                _ => Err(DecodeError::Invalid(word)),
            }
        }
        0x13 => {
            let imm_u = (word >> 20) & 0xfff;
            let imm = sext(imm_u, 12);
            match funct3 {
                0x0 => Ok(Instruction::ADDI { rd, rs1, imm }),
                0x2 => Ok(Instruction::SLTI { rd, rs1, imm }),
                0x3 => Ok(Instruction::SLTIU { rd, rs1, imm }),
                0x4 => Ok(Instruction::XORI { rd, rs1, imm }),
                0x6 => Ok(Instruction::ORI { rd, rs1, imm }),
                0x7 => Ok(Instruction::ANDI { rd, rs1, imm }),
                0x1 => {
                    let shamt = ((word >> 20) & 0x1f) as u32;
                    if funct7 == 0x00 {
                        Ok(Instruction::SLLI { rd, rs1, shamt })
                    } else {
                        Err(DecodeError::Invalid(word))
                    }
                }
                0x5 => {
                    let shamt = ((word >> 20) & 0x1f) as u32;
                    match funct7 {
                        0x00 => Ok(Instruction::SRLI { rd, rs1, shamt }),
                        0x20 => Ok(Instruction::SRAI { rd, rs1, shamt }),
                        _ => Err(DecodeError::Invalid(word)),
                    }
                }
                _ => Err(DecodeError::Invalid(word)),
            }
        }
        0x33 => {
            match (funct7, funct3) {
                (0x00, 0x0) => Ok(Instruction::ADD { rd, rs1, rs2 }),
                (0x20, 0x0) => Ok(Instruction::SUB { rd, rs1, rs2 }),
                (0x00, 0x1) => Ok(Instruction::SLL { rd, rs1, rs2 }),
                (0x00, 0x2) => Ok(Instruction::SLT { rd, rs1, rs2 }),
                (0x00, 0x3) => Ok(Instruction::SLTU { rd, rs1, rs2 }),
                (0x00, 0x4) => Ok(Instruction::XOR { rd, rs1, rs2 }),
                (0x00, 0x5) => Ok(Instruction::SRL { rd, rs1, rs2 }),
                (0x20, 0x5) => Ok(Instruction::SRA { rd, rs1, rs2 }),
                (0x00, 0x6) => Ok(Instruction::OR { rd, rs1, rs2 }),
                (0x00, 0x7) => Ok(Instruction::AND { rd, rs1, rs2 }),
                (0x01, 0x0) => Ok(Instruction::MUL { rd, rs1, rs2 }),
                (0x01, 0x1) => Ok(Instruction::MULH { rd, rs1, rs2 }),
                (0x01, 0x2) => Ok(Instruction::MULHSU { rd, rs1, rs2 }),
                (0x01, 0x3) => Ok(Instruction::MULHU { rd, rs1, rs2 }),
                (0x01, 0x4) => Ok(Instruction::DIV { rd, rs1, rs2 }),
                (0x01, 0x5) => Ok(Instruction::DIVU { rd, rs1, rs2 }),
                (0x01, 0x6) => Ok(Instruction::REM { rd, rs1, rs2 }),
                (0x01, 0x7) => Ok(Instruction::REMU { rd, rs1, rs2 }),
                _ => Err(DecodeError::Invalid(word)),
            }
        }
        0x0f => {
            match funct3 {
                0x0 => Ok(Instruction::FENCE),
                0x1 => Ok(Instruction::FENCEI),
                _ => Err(DecodeError::Invalid(word)),
            }
        }
        0x73 => {
            if funct3 == 0 {
                let imm12 = (word >> 20) & 0xfff;
                if imm12 == 0 {
                    Ok(Instruction::ECALL)
                } else if imm12 == 1 {
                    Ok(Instruction::EBREAK)
                }
                else { Err(DecodeError::Invalid(word)) }
            }
            else { Err(DecodeError::Invalid(word)) }
        }
        _ => Err(DecodeError::Invalid(word)),
    }
}
