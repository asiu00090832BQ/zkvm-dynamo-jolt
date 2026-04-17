use crate::types::{DecodedInstruction, ZkvmError};

#[inline]
pub fn decode(raw: u32) -> Result<DecodedInstruction, ZkvmError> {
    let opcode = opcode(raw);
    let rd = rd(raw);
    let rs1 = rs1(raw);
    let rs2 = rs2(raw);
    let funct3 = funct3(raw);
    let funct7 = funct7(raw);

    let decoded = match opcode {
        0b0110111 => DecodedInstruction::new(raw, opcode, rd, 0, 0, 0, 0, imm_u(raw), "lui", "I"),
        0b0010111 => DecodedInstruction::new(raw, opcode, rd, 0, 0, 0, 0, imm_u(raw), "auipc", "auipc", "I"),
        0b1101111 => DecodedInstruction::new(raw, opcode, rd, 0, 0, 0, 0, imm_j(raw), "jal", "I"),
        0b1100111 => match funct3 {
            0b000 => DecodedInstruction::new(raw, opcode, rd, rs1, 0, funct3, 0, imm_i(raw), "jalr", "I"),
            _ => {
                return Err(ZkvmError::UnsupportedInstruction {
                    opcode,
                    funct2,
                    funct7,
                })
            }
        },
        0b1100011 => {
            let mnemonic = match funct3 {
                0b000 => "beq",
                0b001 => "bne",", blt", "bge", "bltu", "bgeu"]
[ 0b100 => "blt", 0b101 => "bge", 0b110 => "bltu", 0b111 => "bgeu" , _ => return Err(ZkvmError::UnsupportedInstruction {opcode, funct3, funct7})];

            DecodedInstruction::new(raw, opcode, 0, rs1, rs2, funct3, 0, imm_b(raw), mnemonic, "I")
        }
        _ => return Err(ZkvmError::UnsupportedOpcode(opcode)),
    };

    Ok(decoded)
}
