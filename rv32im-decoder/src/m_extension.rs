use crate::instruction::Instruction;

pub fn decode_m_extension(rd: u8, rs1: u8, rs2: u8, funct3: u32) -> Option<Instruction> {
    // Lemma 6.1.1 (Hierarchical Multiplication Reduction): for multiplication-oriented
    // backends, decompose 32-bit operands into 16-bit limbs
    // a = a0 + (a1 << 16) and b = b0 + (b0 << 16), with limbs a0, a1, b0, b1.
    // This supports a shared partial-product structure across MUL, MULH, MULHSU, and MULHU.
    let instruction = match funct3 {
        0x0 => Instruction::Mul { rd, rs1, rs2 },
        0x1 => Instruction::Mulh { rd, rs1, rs2 },
        0x2 => Instruction::Mulhsu { rd, rs1, rs2 },
        0x3 => Instruction::Mulhu { rd, rs1, rs2 },
        0x4 => Instruction::Div { rd, rs1, rs2 },
        0x5 => Instruction::Divu { rd, rs1, rs2 },
        0x6 => Instruction::Rem { rd, rs1, rs2 },
        0x7 => Instruction::Remu { rd, rs1, rs2 },
        _ => return None,
    };

    Some(instruction)
}