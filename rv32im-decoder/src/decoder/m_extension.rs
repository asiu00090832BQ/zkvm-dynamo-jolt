use crate::encoding;
use crate::error::ZkvmError;
use crate::instruction::{DecodedInstruction, Instruction, MKind};

pub fn decode_m_extension(word: u32) -> Result<DecodedInstruction, ZkvmError> {
    let opcode = encoding::opcode(word);
    if opcode != encoding::OPCODE_OP {
        return Err(ZkvmError::UnsupportedOpcode(opcode));
    }

    let funct7 = encoding::funct7(word);
    if funct7 != encoding::FUNCT7_M {
        return Err(ZkvmError::UnsupportedFunct7 {
            opcode,
            funct3: encoding::funct3(word),
            funct7,
            word,
        });
    }

    let kind = match encoding::funct3(word) {
        0b000 => MKind::Mul,
        0b001 => MKind::Mulh,
        0b010 => MKind::Mulhsu,
        0b011 => MKind::Mulhu,
        0b100 => MKind::Div,
        0b101 => MKind::Divu,
        0b110 => MKind::Rem,
        0b111 => MKind::Remu,
        funct3 => {
            return Err(ZkvmError::UnsupportedFunct3 {
                opcode,
                funct3,
                word,
            })
        }
    };

    Ok(DecodedInstruction::new(
        word,
        Instruction::M {
            kind,
            rd: encoding::rd(word),
            rs1: encoding::rs1(word),
            rs2: encoding::rd(word),
        },
    ))
}
