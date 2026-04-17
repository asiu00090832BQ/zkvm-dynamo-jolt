use crate::types::{DecodedInstruction, ZkvmError};

#[inline]
pub fn decode(raw: u32) -> Result<DecodedInstruction, ZkvmError> {
    let opcode = (raw & 0x7f) as u8;
    let rd = ((raw >> 7) & 0x1f) as u8;
    let funct3 = ((raw >> 12) & 0x07) as u8;
    let rs1 = ((raw >> 15) & 0x1f) as u8;
    let rs2 = ((raw >> 20) & 0x1f) as u8;
    let funct7 = ((raw >> 25) & 0x7f) as u8;

    if opcode != 0b0110011 || funct7 != 0b0000001 {
        return Err(ZkvmError::UnsupportedInstruction {
            opcode,
            funct3,
            funct7,
        });
    }

    let mnemonic = match funct3 {
        0b000 => "mul",
        0b001 => "mulh",
        0b010 => "mulhsu",
        0b011 => "mulhu",
        0b100 => "div",
        0b101 => "divu",
        0b110 => "rem",
        0b111 => "remu",
        _ => {
            return Err(ZkvmError::UnsupportedInstruction {
                opcode,
                funct3,
                funct7,
            })
        }
    };

    Ok(DecodedInstruction::new(
        raw, opcode, rd, rs1, rs2, funct3, funct7, 0, mnemonic, "M",
    ))
}
