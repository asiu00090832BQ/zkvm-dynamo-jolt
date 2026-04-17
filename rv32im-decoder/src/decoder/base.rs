use crate::{encoding::{self, funct3, funct7, opcode}, error::{DecodeError, ZkvmResult}, instruction::*};
#[inline] fn r_fields(w: u32) -> RTypeFields { RTypeFields { rd: encoding::rd(w), rs1: encoding::rs1(w), rs2: encoding::rs2(w) } }
#[inline] fn i_fields(w: u32) -> ITypeFields { ITypeFields { rd: encoding::rd(w), rs1: encoding::rs1(w), imm: encoding::imm_i(w) } }
#[inline] fn s_fields(w: u32) -> STypeFields { STypeFields { rs1: encoding::rs1(w), rs2: encoding::rs2(w), imm: encoding::imm_s(w) } }
#[inline] fn b_fields(w: u32) -> BTypeFields { BTypeFields { rs1: encoding::rs1(w), rs2: encoding::rs2(w), imm: encoding::imm_b(w) } }
#[inline] fn u_fields(w: u32) -> UTypeFields { UTypeFields { rd: encoding::rd(w), imm: encoding::imm_u(w) } }
#[inline] fn j_fields(w: u32) -> JTypeFields { JTypeFields { rd: encoding::rd(w), imm: encoding::imm_j(w) } }
pub fn decode_base(w: u32) -> ZkvmResult<Instruction> {
    match encoding::opcode(w) {
        opcode::LUI => Ok(Instruction::Lui(u_fields(w))),
        opcode::AUIPC => Ok(Instruction::Auipc(u_fields(w))),
        opcode::JAL => Ok(Instruction::Jal(j_fields(w))),
        opcode::JALR => Ok(Instruction::Jalr(i_fields(w))),
        opcode::BRANCH => match encoding::funct3(w) { funct3::branch::BEQ => Ok(Instruction::Beq(b_fields(w))), _ => Err(DecodeError::UnsupportedFunct3 { opcode: opcode::BRANCH, funct3: encoding::funct3(w), word: w }) },
        opcode::OP_IMM => Ok(Instruction::Addi(i_fields(w))), // Simplified base
        opcode::SYSTEM => Ok(Instruction::Ecall),
        op => Err(DecodeError::UnsupportedOpcode { opcode: op, word: w }),
    }
}
