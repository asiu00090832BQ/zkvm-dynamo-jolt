// CAB8453E: bit selectors and RV32IM instruction routing.

use core::fmt;

pub const R_TYPE_OPCODE: u8 = 0b0110011;
pub const BASE_R_FUNCT7: u8 = 0b0000000;
pub const SUB_SRA_FUNCT7: u8 = 0b0100000;
pub const M_FUNCT7: u8 = 0b0000001;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

impl Instruction {
    #[inline]
    pub fn is_m_extension(self) -> bool {
        matches!(
            self,
            Self::Mul
                | Self::Mulh
                | Self::Mulhsu
                | Self::Mulhu
                | Self::Div
                | Self::Divu
                | Self::Rem
                | Self::Remu
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedInstruction {
    pub raw: u32,
    pub instruction: Instruction,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
}

impl DecodedInstruction {
    #[inline]
    pub const fn new(raw: u32, instruction: Instruction, rd: u8, rs1: u8, rs2: u8) -> Self {
        Self {
            raw,
            instruction,
            rd,
            rs1,
            rs2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    UnsupportedOpcode { opcode: u8 },
    UnsupportedInstruction { opcode: u8, funct3: u8, funct7: u8 },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOpcode { opcode } => {
                write!(f, "unsupported opcode 0b{opcode:07b}")
            }
            Self::UnsupportedInstruction {
                opcode,
                funct3,
                funct7,
            } => {
                write!(
                    f,
                    "unsupported rv32im routing tuple opcode=0b{opcode:07b}, funct3=0b{funct3:03b}, funct7=0b{funct7:07b}"
                )
            }
        }
    }
}

#[inline]
pub const fn bit_mask(len: u8) -> u32 {
    match len {
        0 => 0,
        32..=u8::MAX => u32::MAX,
        _ => (1u32 << len) - 1,
    }
}

#[inline]
pub const fn bit_slice(word: u32, start: u8, len: u8) -> u32 {
    if start >= 32 || len == 0 {
        return 0;
    }

    let available = 32 - start;
    let effective = if len > available { available } else { len };
    (word >> start) & bit_mask(effective)
}

#[inline]
pub const fn opcode(word: u32) -> u8 {
    bit_slice(word, 0, 7) as u8
}

#[inline]
pub const fn rd(word: u32) -> u8 {
    bit_slice(word, 7, 5) as u8
}

#[inline]
pub const fn funct3(word: u32) -> u8 {
    bit_slice(word, 12, 3) as u8
}

#[inline]
pub const fn rs1(word: u32) -> u8 {
    bit_slice(word, 15, 5) as u8
}

#[inline]
pub const fn rs2(word: u32) -> u8 {
    bit_slice(word, 20, 5) as u8
}

#[inline]
pub const fn funct7(word: u32) -> u8 {
    bit_slice(word, 25, 7) as u8
}

pub fn route_instruction(word: u32) -> Result<Instruction, DecodeError> {
    let op = opcode(word);
    if op != R_TYPE_OPCODE {
        return Err(DecodeError::UnsupportedOpcode { opcode: op });
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
        _ => Err(DecodeError::UnsupportedInstruction {
            opcode: op,
            funct3: f3,
            funct7: f7,
        }),
    }
}

pub fn decode(word: u32) -> Result<DecodedInstruction, DecodeError> {
    let instruction = route_instruction(word)?;
    Ok(DecodedInstruction::new(
        word,
        instruction,
        rd(word),
        rs1(word),
        rs2(word),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode_r(funct7: u8, rs2: u8, rs1: u8, funct3: u8, rd: u8) -> u32 {
        ((funct7 as u32) << 25)
            | ((rs2 as u32) << 20)
            | ((rs1 as u32) << 15)
            | ((funct3 as u32) << 12)
            | ((rd as u32) << 7)
            | (R_TYPE_OPCODE as u32)
    }

    #[test]
    fn slices_fields_correctly() {
        let word = encode_r(M_FUNCT7, 3, 2, 0b100, 1);
        assert_eq!(opcode(word), R_TYPE_OPCODE);
        assert_eq!(rd(word), 1);
        assert_eq!(rs1(word), 2);
        assert_eq!(rs2(word), 3);
        assert_eq!(funct3(word), 0b100);
        assert_eq!(funct7(word), M_FUNCT7);
    }

    #[test]
    fn routes_base_r_and_m_extension() {
        assert_eq!(
            route_instruction(encode_r(BASE_R_FUNCT7, 3, 2, 0b000, 1)).unwrap(),
            Instruction::Add
        );
        assert_eq!(
            route_instruction(encode_r(SUB_SRA_FUNCT7, 3, 2, 0b101, 1)).unwrap(),
            Instruction::Sra
        );
        assert_eq!(
            route_instruction(encode_r(M_FUNCT7, 3, 2, 0b111, 1)).unwrap(),
            Instruction::Remu
        );
    }
}
