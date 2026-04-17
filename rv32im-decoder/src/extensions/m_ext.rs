use crate::{funct3, funct7, opcode, rd, rs1, rs2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MInstruction {
    Mul { rd: u8, rs1: u8, rs2: u8 },
    Mulh { rd: u8, rs1: u8, rs2: u8 },
    Mulhsu { rd: u8, rs1: u8, rs2: u8 },
    Mulhu { rd: u8, rs1: u8, rs2: u8 },
    Div { rd: u8, rs1: u8, rs2: u8 },
    Divu { rd: u8, rs1: u8, rs2: u8 },
    Rem { rd: u8, rs1: u8, rs2: u8 },
    Remu { rd: u8, rs1: u8, rs2: u8 },
}

pub fn decode_m_instruction(word: u32) -> Option<MInstruction> {
    if opcode(word) != 0b0110011 || funct7(word) != 0b0000001 {
        return None;
    }

    let instruction = match funct3(word) {
        0b000 => MInstruction::Mul {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        0b001 => MInstruction::Mulh {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        0b010 => MInstruction::Mulhsu {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        0b011 => MInstruction::Mulhu {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        0b100 => MInstruction::Div {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        0b110 => MInstruction::Rem {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        0b111 => MInstruction::Remu {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
        _ => return None,
    };

    Some(instruction)
}
