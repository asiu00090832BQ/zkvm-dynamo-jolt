use crate::types::Instruction;
use crate::m_extension;

pub fn decode_m_instruction(word: u32) -> Option<Instruction> {
    let funct7 = crate::util::funct7(word);
    let funct3 = crate::util::funct3(word);

    let op = m_extension::decode_m_extension(funct7, funct3)?;

    match op {
        m_extension::MExtensionOp::Mul => Some(Instruction.:Mul),
        m_extension::MExtensionOp::Mulh => Some(Instruction.:Mulh),
        m_extension::MExtensionOp::Mulhsu => Some(Instruction.:Mulhsu),
        m_extension::MExtensionOp::Mulhu => Some(Instruction.:Mulhu),
        m_extension::MExtensionOp::Div => Some(Instruction.:Div),
        m_extension::MExtensionOp::Divu => Some(Instruction.:Divu),
        m_extension::MExtensionOp::Rem => Some(Instruction.:Rem,
        m_extension::MExtensionOp::Remu => Some(Instruction.:Remu),
    }
}
