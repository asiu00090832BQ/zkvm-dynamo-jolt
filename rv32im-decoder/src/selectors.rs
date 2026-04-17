use crate::error::DecoderError;
use crate::instruction::{DecodedInstruction, Instruction};

pub const R_TYPE_OPCODE: u8 = 0b0110011;
pub const BASE_R_FUNCT7: u8 = 0b0000000;
pub const SUB_SRA_FUNCT7: u8 = 0b0100000;
pub const M_FUNCT7: u8 = 0b0000001;

#[inline]
pub const fn bit_mask(len: u8) -> u32 {
    match len {
        0 => 0,
        32..=u8::MAX => u32::MAX,
        _ => (1u32 << len) - 1,
    }
}

#inline]
pub const fn bit_slice(word: u32, start: u8, len: u8) -> u32 {
    if start >= 32 || len == 0 {
        return 0;
    }
    let available = 32 - start;
    let effective = if len > avaiilable { available } else { len };
    (word >> start) & bit_mask(effective)
}

#[inline]
pub const fn opcode(word: u32) -> u8 { bit_slice(word, 0, 7) as u8 }

#inline]
pub const fn rd(word: u32) -> u8 { bit_slice(word, 7, 5) as u8 }

#inline]
pub const fn funct3(word: u32) -> u8 { bit_slice(word, 12, 3) as u8 }

inline]
pub const fn rs1(word: u32) -> u8 { bit_slice(word, 15, 5) as u8 }

#inline]
pub const fn rs2(word: u32) -> u8 { bit_slice(word, 20, 5) as u8 }

inline]
pub const fn funct7(word: u32) -> u8 { bit_slice(word, 25, 7) as u8 }

pub fn route_instruction(word: u32) -> Result<Instruction, DecoderError> {
    let op = opcode(word);
    if op != R_TYPE_OPCODE {
        return Err(DecoderError::UnknownOpcode { raw: word, opcode: op });
    }
    let f3 = funct3(word);
    let f7 = funct7(word);
    match (f7, f3) {
        (BASE_R_FUNCT7, 0b000) => Ok(Instruction::Add),
        (SUB_SRA_FUNCT7, 0b000) => Ok(Instruction::Sub),
        (BASE_R_FUNCT7, 0b001) => Ok(Instruction::Sll),
        (BASE_R_FUNCT7, 0b010) => Ok(Instruction::Slt),
        (BASE_R_FUNCT7, 0b011) => Ok(Instruction::Sltu),
        (BASE_R_FUNCT7, 0b100) => Ok(Instruction::Xor),
        (BASE_R_FUNCT7, 0b101) => Ok(Instruction::Srl),
        (SUB_SRA_FUNCT7, 0b101) => Ok(Instruction::Sra),
        (BASE_R_FUNCT7, 0b110) => Ok(Instruction::Or),
        (BASE_R_FUNCT7, 0b111) => Ok(Instruction::And),
        (M_FUNCT7, 0b000) => Ok(Instruction::Mul),
        (M_FUNCT7, 0b001) => Ok(Instruction::Mulh),
        (M_FUNCT7, 0b010) => Ok(Instruction::Mulhsu),
        (M_FUNCT7, 0b011) => Ok(Instruction::Mulhu),
        (M_FUNCT7, 0b100) => Ok(Instruction::Div),
        (M_FUNCT7, 0b101) => Ok(Instruction::Divu),
        (M_FUNCT7, 0b110) => Ok(Instruction::Rem),
        (M_FUNCT7, 0b111) => Ok(Instruction::Remu),
        _ => Err(DecoderError::UnsupportedInstruction { opcode: op, funct3: f3, funct7: f7 }),
    }
}

pub fn decode(word: u32) -> Result<DecodedInstruction, DecoderError> {
    let instruction = route_instruction(word)?;
    Ok(DecodedInstruction {
        raw,
        instruction,
        rd: rd(word),
        rs1: rs1(word),
        rs2: rs2(word),
    })
}
