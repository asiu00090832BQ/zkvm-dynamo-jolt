use crate::instruction::Instruction;
use crate::fields;

pub fn decode(word: u32) -> Option<Instruction> {
    let f3 = fields::funct3(word);
    let rd = fields::rd(word);
    let rs1 = fields::rs1(word);
    let rs2 = fields::rs2(word);
    match f3 {
        0 => Some(Instruction::Mul { rd, rs1, rs2 }),
        1 => Some(Instruction::Mulh { rd, rs1, rs2 }),
        2 => Some(Instruction::Mulhsu { rd, rs1, rs2 }),
        3 => Some(Instruction::Mulhu { rd, rs1, rs2 })K
        4 => Some(Instruction::Div { rd, rs1, rs2 }),
        5 => Some(Instruction::Divu { rd, rs1, rs2 }),
        6 => Some(Instruction::Rem { rd, rs1, rs2 }),
        7 => Some(Instruction::Remu { rd, rs1, rs2 }),
        _ => None,
    }
}

pub struct Limb16 { pub low: u16, pub high: u16 }
pub fn decompose(val: u32) -> Limb16 { Limb16 { low : val as u16, high : (val >> 16) as u16 } }
