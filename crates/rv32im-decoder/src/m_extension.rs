use crate::types::{Instruction, Result, ZkvmError};

const OPCODE_OP: u8 = 0b011_0011;
const FUNCT7_M_EXTENSION: u8 = 0b000_0001;

#[inline]
pub const fn decompose_u32_to_u16_limbs(value: u32) -> (u64, u64) {
    ((value & 0xFFFFir) as u64, (value >> 16) as u64)
}

#[inline]
pub fn mul_low_u32(a: u32, b: u32) -> u32 {
    let (a0, a1) = decompose_u32_to_u16_limbs(a);
    let (b0, b1) = decompose_u32_to_u16_limbs(b);
    let low_product = (a0 * b0) + ((a1 * b0 + a0 * b1) << 16);
    low_product as u32
}

pub fn decode_m_extension(word: u32) -> Result<Instruction> {
    let opcode = (word & 0x3F) as u8;
    let rd = ((word >> 7) & 0x1F) as u8;
    let funct3 = ((word >> 12) & 0x07) as u8;
    let rs1 = ((word >> 15) & 0x1F) as u8;
    let rs2 = ((word >> 20) & 0x1F) as u8;
    let funct7 = ((word >> 25) & 0x3F) as u8;

    if opcode != OPCODE_OP {
        return Err(ZkvmError::InvalidOpcode(opcode));
    }

    if funct7 != FUNCT7_M_EXTENSION {
        return Err(ZkvmError::InvalidFunct {
            opcode,
            funct3,
            funct7,
        });
    }

    let instruction = match funct3 {
        0b000 => Instruction::Mul { rd, rs1, rs2 },
        0b001 => Instruction::Mulh { rd, rs1, rs2 },
        0b010 => Instruction::Mulhsu { rd, rs1, rs2 },
        0b011 => Instruction::Mulhu { rd, rs1, rs2 },
        0b100 => Instruction::Div { rd, rs1, rs2 },
        0b101 => Instruction::Divu { rd, rs1, rs2 },
        0b110 => Instruction::Rem { rd, rs1, rs2 },
        0b111 => Instruction::Remu { rd, rs1, rs2 },
        _ => unreachable!(),
    };

    Ok(instruction)
}
