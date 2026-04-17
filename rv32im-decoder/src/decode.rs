use crate::decoder::DecodeError;
use crate::instruction::Instruction;
use crate::m_extension::decode_m_instruction;

pub(crate) fn decode_word(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = (word & 0x7f) as u8;

    match opcode {
        0b0110111 => Ok(Instruction::Lui {
            rd: ((word >> 7) & 0x1f) as u8,
            imm: (word & 0xffff_f000) as i32,
        }),
        0b0010111 => Ok(Instruction::Auipc {
            rd: ((word >> 7) & 0x1f) as u8,
            imm: (word & 0xffff_f000) as i32,
        }),
        0b1101111 => Ok(Instruction::Jal {
            rd: ((word >> 7) & 0x1f) as u8,
            imm: sign_extend(((word >> 31) << 20) | (((word >> 12) & 0xff) << 12) | (((word >> 20) & 0x01) << 11) | (((word >> 21) & 0x03ff) << 1), 21),
        }),
        0b1100111 => {
            let funct3 = ((word >> 12) & 0x07) as u8;
            if funct3 != 0b000 {
                return Err(DecodeError::InvalidFunct3 { opcode, funct3 });
            }

            Ok(Instruction::Jalr {
                rd: ((word >> 7) & 0x1f) as u8,
                rs1: ((word >> 15) & 0x1f) as u8,
                imm: sign_extend(word >> 20, 12),
            })
        }
        0b1100011 => {
            let rs1 = ((word >> 15) & 0x1f) as u8;
            let rs2 = ((word >> 20) & 0x1f) as u8;
            let imm = sign_extend(((word >> 31) << 12) | (((word >> 7) & 0x01) << 11) | (((word >> 25) & 0x3f) << 5) | (((word >> 8) & 0x0f) << 1), 13);

            match ((word >> 12) & 0x07) as u8 {
                0b000 => Ok(Instruction::Beq { rs1, rs2, imm }),
                0b001 => Ok(Instruction::Bne { rs1, rs2, imm }),
                0b100 => Ok(Instruction::Blt { rs1, rs2, imm }),
                0b101 => Ok(Instruction::Bge { rs1, rs2, imm }),
                0b110 => Ok(Instruction::Bltu { rs1, rs2, imm }),
                0b111 => Ok(Instruction::Bgeu { rs1, rs2, imm }),
                funct3 => Err(DecodeError::InvalidFunct3 { opcode, funct3 }),
            }
        }
        0b0000011 => {
            let rd = ((word >> 7) & 0x1f) as u8;
            let rs1 = ((word >> 15) & 0x1f) as u8;
            let imm = sign_extend(word >> 20, 12);

            match ((word >> 12) & 0x07) as u8 {
                0b000 => Ok(Instruction::Lb { rd, rs1, imm }),
                0b001 => Ok(Instruction::Lh { rd, rs1, imm }),
                0b010 => Ok(Instruction::Lw { rd, rs1, imm }),
                0b100 => Ok(Instruction::Lbu { rd, rs1, imm }),
                0b101 => Ok(Instruction::Lhu { rd, rs1, imm }),
                funct3 => Err(DecodeError::InvalidFunct3 { opcode, funct3 }),
            }
        }
        0b0100011 => {
            let rs1 = ((word >> 15) & 0x1f) as u8;
            let rs2 = ((word >> 20) & 0x1f) as u8;
            let imm = sign_extend(((word >> 25) << 5) | ((word >> 7) & 0x1f), 12);

            match ((word >> 12) & 0x07) as u8 {
                0b000 => Ok(Instruction::Sb { rs1, rs2, imm }),
                0b001 => Ok(Instruction::Sh { rs1, rs2, imm }),
                0b010 => Ok(Instruction::Sw { rs1, rs2, imm }),
                funct3 => Err(DecodeError::InvalidFunct3 { opcode, funct3 }),
            }
        }
        0b0010011 => {
            let rd = ((word >> 7) & 0x1f) as u8;
            let rs1 = ((word >> 15) & 0x1f) as u8;

            match ((word >> 12) & 0x07) as u8 {
                0b000 => Ok(Instruction::Addi { rd, rs1, imm: sign_extend(word >> 20, 12) }),
                0b010 => Ok(Instruction::Slti { rd, rs1, imm: sign_extend(word >> 20, 12) }),
                0b011 => Ok(Instruction::Sltiu { rd, rs1, imm: sign_extend(word >> 20, 12) }),
                0b100 => Ok(Instruction::Xori { rd, rs1, imm: sign_extend(word >> 20, 12) }),
                0b110 => Ok(Instruction::Ori { rd, rs1, imm: sign_extend(word >> 20, 12) }),
                0b111 => Ok(Instruction::Andi { rd, rs1, imm: sign_extend(word >> 20, 12) }),
                0b001 => {
                    let funct7 = ((word >> 25) & 0x7f) as u8;
                    if funct7 != 0b0000000 {
                        return Err(DecodeError::InvalidFunct7 { opcode, funct3: 0b001, funct7 });
                    }
                    Ok(Instruction::Slli { rd, rs1, shamt: ((word >> 20) & 0x1f) as u8 })
                }
                0b101 => match ((word >> 25) & 0x7f) as u8 {
                    0b0000000 => Ok(Instruction::Srli { rd, rs1, shamt: ((word >> 20) & 0x1f) as u8 }),
                    0b0100000 => Ok(Instruction::Srai { rd, rs1, shamt: ((word >> 20) & 0x1f) as u8 }),
                    funct7 => Err(DecodeError::InvalidFunct7 { opcode, funct3: 0b101, funct7 }),
                },
                funct3 => Err(DecodeError::InvalidFunct3 { opcode, funct3 }),
            }
        }
        0b0110011 => {
            let rd = ((word >> 7) & 0x1f) as u8;
            let rs1 = ((word >> 15) & 0x1f) as u8;
            let rs2 = ((word >> 20) & 0x1f) as u8;
            let funct3 = ((word >> 12) & 0x07) as u8;
            let funct7 = ((word >> 25) & 0x7f) as u8;

            if funct7 == 0b0000001 {
                return decode_m_instruction(funct3, rd, rs1, rs2);
            }

            match (funct3, funct7) {
                (0b000, 0b0000000) => Ok(Instruction::Add { rd, rs1, rs2 }),
                (0b000, 0b0100000) => Ok(Instruction::Sub { rd, rs1, rs2 }),
                (0b001, 0b0000000) => Ok(Instruction::Sll { rd, rs1, rs2 }),
                (0b010, 0b0000000) => Ok(Instruction::Slt { rd, rs1, rs2 }),
                (0b011, 0b0000000) => Ok(Instruction::Sltu { rd, rs1, rs2 }),
                (0b100, 0b0000000) => Ok(Instruction::Xor { rd, rs1, rs2 }),
                (0b101, 0b0000000) => Ok(Instruction::Srl { rd, rs1, rs2 }),
                (0b101, 0b0100000) => Ok(Instruction::Sra { rd, rs1, rs2 }),
                (0b110, 0b0000000) => Ok(Instruction::Or { rd, rs1, rs2 }),
                (0b111, 0b0000000) => Ok(Instruction::And { rd, rs1, rs2 }),
                _ => Err(DecodeError::InvalidFunct7 { opcode, funct3, funct7 }),
            }
        }
        0b0001111 => Ok(Instruction::Fence),
        0b1110011 => match word {
            0x0000_0073 => Ok(Instruction::Ecall),
            0x0010_0073 => Ok(Instruction::Ebreak),
            _ => Err(DecodeError::ReservedEncoding(word)),
        },
        _ => Err(DecodeError::UnsupportedOpcode(opcode)),
    }
}

fn sign_extend(value: u32, bits: u32) -> i32 {
    let shift = 32 - bits;
    ((value << shift) as i32) >> shift
}
