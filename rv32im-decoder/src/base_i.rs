use crate::{error::ZkvmError, formats::*, instruction::*};
pub fn decode_base_i(w: u32) -> Result<BaseIInstruction, ZkvmError> {
    let op = (w & 0x7f) as u8;
    match op {
        0x37 => { let u = UType::decode(w); Ok(BaseIInstruction::Lui { rd: u.rd, imm: u.imm }) }
        0x13 => { let i = IType::decode(w); Ok(BaseIInstruction::OpImm { kind: OpImmKind::Addi, rd: i.rd, rs1: i.rs1, imm: i.imm }) }
        _ => Err(ZkvmError::UnknownOpcode { word: w, opcode: op }),
    }
}
