use crate::{encoding::{self, funct3, opcode}, error::{DecodeError, ZkvmResult}, instruction::{Instruction, RTypeFields}};
pub fn decode_m_extension(w: u32) -> ZkvmResult<Instruction> {
    let f3 = encoding::funct3(w); let fields = RTypeFields { rd: encoding::rd(w), rs1: encoding::rs1(w), rs2: encoding::rs2(w) };
    match f3 {
        funct3::m::MUL => Ok(Instruction::Mul(fields)),
        funct3::m::DIV => Ok(Instruction::Div(fields)),
        _ => Err(DecodeError::UnsupportedFunct3 { opcode: opcode::OP, funct3: f3, word: w })
    }
}
