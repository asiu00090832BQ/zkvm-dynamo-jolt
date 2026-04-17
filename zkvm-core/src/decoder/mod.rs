pub mod instruction;
pub mod error;

pub use instruction::{Instruction, MulDivKind};
pub use error::DecodeError;

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as u8;
    let funct3 = ((word >> 12) & 0x7) as u8;
    let rs1 = ((word >> 15) & 0x1f) as u8;
    let rs2 = ((word >> 20) & 0x1f) as u8;
    let funct7 = ((word >> 25) & 0x7f) as u8;

    match opcode {
        0x33 => {
            if funct7 == 0x01 {
                let kind = match funct3 {
                    0 => MulDivKind::Mul,
                    1 => MulDivKind::Mulh,
                    2 => MulDivKind::Mulhsu,
                    3 => MulDivKind::Mulhu,
                    4 => MulDivKind::Div,
                    5 => MulDivKind::Divu,
                    6 => MulDivKind::Rem,
                    7 => MulDivKind::Remu,
                    _ => unreachable!(),
                };
                Ok(Instruction::MulDiv { kind, rd, rs1, rs2 })
            } else if funct7 == 0x00 {
                match funct3 {
                    0 => Ok(Instruction::Add { rd, rs1, rs2 }),
                    _ => Err(DecodeError::InvalidInstruction(word)),
                }
            } else if funct7 == 0x20 {
                match funct3 {
                    0 => Ok(Instruction::Sub { rd, rs1, rs2 }),
                    _ => Err(DecodeError::InvalidInstruction(word)),
                }
            } else {
                Err(DecodeError::InvalidInstruction(word))
            }
        }
        0x73 => Ok(Instruction::Ecall),
        _ => Err(DecodeError::InvalidInstruction(word)),
    }
}
