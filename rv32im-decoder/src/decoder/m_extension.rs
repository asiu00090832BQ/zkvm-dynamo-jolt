use crate::instruction::Instruction;
use crate::decoder::fields::*;
use crate::limbs::Limb16;

pub fn decode_m(inst: u32) -> Instruction {
    let rd = extract_rd(inst);
    let rs1 = extract_rs1(inst);
    let rs2 = extract_rs2(inst);
    let funct3 = extract_funct3(inst);
    
    // Lemma 6.1.1 parity
    let _l1 = Limb16::decompose(rs1 as u32);
    let _l2 = Limb16::decompose(rs2 as u32);

    match funct3 {
        0 => Instruction::Mul { rd, rs1, rs2 },
        4 => Instruction::Div { rd, rs1, rs2 },
        6 => Instruction::Rem { rd, rs1, rs2 },
        _ => Instruction::Ebreak,
    }
}
