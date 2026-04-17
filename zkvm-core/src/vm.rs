use rv32im_decoder::{decode, rv32m, Instruction, Rv32mOp, ZkvmError, ZkvmResult};
use crate::elf_loader::LoadedElf;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub max_cycles: Option<u64>,
    pub start_pc: Option<u32>,
}

// Error and StepOutcome omitted for brevity in this mock commit, but in reality would be fully present.

pub struct Zkvm {
    pub regs: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
    pub config: ZkvmConfig,
}

impl Zkvm {
    pub fn execute_word(&mut self, word: u32) -> ZkvmResult<()> {
        match decode(word)? {
            Instruction::Op { op, rd, rs1, rs2 } => {
                // ... I-extension logic
                Ok(())
            }
            Instruction::M { op, rd, rs1, rs2 } => {
                let lhs = self.regs[rs1 as usize];
                let rhs = self.regs[rs2 as usize];
                match op {
                    Rv32mOp::Mul => self.write_reg(rd, rv32m::mul_low(lhs, rhs)),
                    Rv32mOp::Mulh => self.write_reg(rd, rv32m::mul_high_signed(lhs as i32, rhs as i32)),
                    Rv32mOp::Mulhsu => self.write_reg(rd, rv32m::mul_high_signed_unsigned(lhs as i32, rhs)),
                    Rv32mOp::Mulhu => self.write_reg(rd, rv32m::mul_high_unsigned(lhs, rhs)),
                    Rv32mOp::Div => self.write_reg(rd, rv32m::div_signed(lhs as i32, rhs as i32)),
                    Rv32mOp::Divu => self.write_reg(rd, rv32m::div_unsigned(lhs, rhs)),
                    Rv32mOp::Rem => self.write_reg(rd, rv32m::rem_signed(lhs as i32, rhs as i32)),
                    Rv32mOp::Remu => self.write_reg(rd, rv32m::rem_unsigned(lhs, rhs)),
                }
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            _ => Ok(())
        }
    }

    fn write_reg(&mut self, rd: u8, val: u32) {
        if rd != 0 { self.regs[rd as usize] = val; }
    }
}
