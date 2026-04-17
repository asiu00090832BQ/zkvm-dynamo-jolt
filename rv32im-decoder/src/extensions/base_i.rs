use crate::instruction::Instruction;
use crate::fields;
use crate::error::{ZkvmResult, ZkvmError};

pub fn decode(word: u32) -> ZkvmResult<Instruction> {
    let op = fields::opcode(word);
    let f3 = fields::funct3(word);
    let rd = fields::rd(word);
    let rs1 = fields::rs1(word);
    let rs2 = fields::rs2(word);

    match op {
        0x33 => match f3 {
            0 => Ok(Instruction::Add { rd, rs1, rs2 }),
            _ => Err(ZkvmError::InvalidFunct3(f3)),
        },
        0x13 => match f3 {
            0 => Ok(Instruction::Addi { rd, rs1, imm: fields::i_imm(word) }),
            _ => Err(ZkvmError::InvalidFunct3(f3)),
        },
        0x37 => Ok(Instruction::Lui { rd, imm: fields::u_imm(word) }),
        0x6f => Ok(Instruction::Jal { rd, imm: fields::j_imm(word) }),
        0x73 => Ok(Instruction::Ecall),
        _ => Err(ZkvmError::InvalidOpcode(op)),
    }
}
