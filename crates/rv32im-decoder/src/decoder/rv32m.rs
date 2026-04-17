use crate::types::*;
use crate::error::*;
pub fn decode(word: u32) -> ZkvmResult<Instruction> {
    let funct3 = (word >> 12) & 0x7;
    let rd = ((word >> 7) & 0x1f) as u8;
    let rs1 = ((word >> 15) & 0x1f) as u8;
    let rs2 = ((word >> 20) & 0x1f) as u8;
    let op = match funct3 {
        0 => Rv32mOp::Mul, 1 => Rv32mOp::Mulh, 2 => Rv32mOp::Mulhsu, 3 => Rv32mOp::Mulhu,
        4 => Rv32mOp::Div, 5 => Rv32mOp::Divu, 6 => Rv32mOp::Rem, 7 => Rv32mOp::Remu,
        _ => unreachable!(),
    };
    Ok(Instruction::M { op, rd, rs1, rs2 })
}
pub fn mul_low(a: u32, b: u32) -> u32 { (a as u64 * b as u64) as u32 }
pub fn mul_high_signed(a: i32, b: i32) -> u32 { ((a as i64 * b as i64) >> 32) as u32 }
pub fn mul_high_signed_unsigned(a: i32, b: u32) -> u32 { ((a as i64 * b as u64 as i64) >> 32) as u32 }
pub fn mul_high_unsigned(a: u32, b: u32) -> u32 { ((a as u64 * b as u64) >> 32) as u32 }
pub fn div_signed(a: i32, b: i32) -> u32 { if b == 0 { !0 } else { a.wrapping_div(b) as u32 } }
pub fn div_unsigned(a: u32, b: u32) -> u32 { if b == 0 { !0 } else { a / b } }
pub fn rem_signed(a: i32, b: i32) -> u32 { if b == 0 { a as u32 } else { a.wrapping_rem(b) as u32 } }
pub fn rem_unsigned(a: u32, b: u32) -> u32 { if b == 0 { a } else { a % b } }
