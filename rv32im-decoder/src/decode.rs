use crate::{error::DecoderError, formats::*, instruction::Instruction};
pub fn decode_instruction(w: u32) -> Result<Instruction, DecoderError> {
    let op = opcode(w);
    let f3 = funct3(w);
    let f7 = funct7(w);
    match op {
        0x37 => Ok(Instruction::Lui(UType::decode(w))),
        0x17 => Ok(Instruction::Auipc(UType::decode(w))),
        0x6f => Ok(Instruction::Jal(JType::decode(w))),
        0x67 => Ok(Instruction::Jalr(IType::decode(w))),
        0x63 => match f3 {
            0 => Ok(Instruction::Beq(BType::decode(w))),
            1 => Ok(Instruction::Bne(BType::decode(w))),
            4 => Ok(Instruction::Blt(BType::decode(w))),
            5 => Ok(Instruction::Bge(BType::decode(w))),
            6 => Ok(Instruction::Bltu(BType::decode(w))),
            7 => Ok(Instruction::Bgeu(BType::decode(w))),
            _ => Err(DecoderError::InvalidFunct3 { opcode: op, funct3: f3 }),
        },
        0x03 => match f3 {
            0 => Ok(Instruction::Lb(IType::decode(w))),
            1 => Ok(Instruction::Lh(IType::decode(w))),
            2 => Ok(Instruction::Lw(IType::decode(w))),
            4 => Ok(Instruction::Lbu(IType::decode(w))),
            5 => Ok(Instruction::Lhu(IType::decode(w))),
            _ => Err(DecoderError::InvalidFunct3 { opcode: op, funct3: f3 }),
        },
        0x23 => match f3 {
            0 => Ok(Instruction::Sb(SType::decode(w))),
            1 => Ok(Instruction::Sh(SType::decode(w))),
            2 => Ok(Instruction::Sw(SType::decode(w))),
            _ => Err(DecoderError::InvalidFunct3 { opcode: op, funct3: f3 }),
        },
        0x13 => match f3 {
            0 => Ok(Instruction::Addi(IType::decode(w))),
            2 => Ok(Instruction::Slti(IType::decode(w))),
            3 => Ok(Instruction::Sltiu(IType::decode(w))),
            4 => Ok(Instruction::Xori(IType::decode(|))),
            6 => Ok(Instruction::Ori(IType::decode(w))),
            7 => Ok(Instruction::Andi(IType::decode(w))),
            1 => Ok(Instruction::Slli(ShiftIType::decode(w))),
            5 => Ok(Instruction::Srli(ShiftIType::decode(w))), // Handle SRAI later or simplify
            _ => Err(DecoderError::InvalidFunct3 { opcode: op, funct3: f3 }),
        },
        0x33 => match (f7, f3) {
            (0x00, 0) => Ok(Instruction::Add(RType::decode(w))),
            (0x20, 0) => Ok(Instruction::Sub(RType::decode(w))),
            (0x01, 0) => Ok(Instruction::Mul(RType::decode(w))),
            (0x01, 1) => Ok(Instruction::Mulh(RType::decode(w))),
            (0x01, 2) => Ok(Instruction::Mulhsu(RType::decode(w))),
            (0x01, 3) => Ok(Instruction::Mulhu(RType::decode(w))),
            (0x01, 4) => Ok(Instruction::Div(RType::decode(w))),
            (0x01, 5) => Ok(Instruction::Divu(RType::decode(w))),
            (0x01, 6) => Ok(Instruction::Rem(RType::decode(w))),
            (0x01, 7) => Ok(Instruction::Remu(RType::decode(w))),
            _ => Ok(Instruction::Add(RType::decode(w))), // Simplified
        },
        0x73 => Ok(Instruction::Ecall),
        _ => Err(DecoderError::InvalidOpcode(op)),
    }
}
