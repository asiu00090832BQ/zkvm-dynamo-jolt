use crate::ZkvmError;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction { Lui { rd: u8, imm: u32 }, Auipc { rd: u8, imm: u32 }, Jal { rd: u8, imm: i32 }, Jalr { rd: u8, rs1: u8, imm: i32 }, Addi { rd: u8, rs1: u8, imm: i32 }, Ecall, Ebreak, Invalid }
pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as u8;
    let rs1 = ((word >> 15) & 0x1f) as u8;
    let imm_i = (word as i32) >> 20;
    match opcode {
        0x37 => Ok(Instruction::Lui { rd, imm: word & 0xffff_f000 }),
        0x17 => Ok(Instruction::Auipc { rd, imm: word & 0xffff_f000 }),
        0x13 => Ok(Instruction::Addi { rd, rs1, imm: imm_i }),
        0x73 => Ok(Instruction::Ecall),
        _ => Ok(Instruction::Invalid)
    }
}