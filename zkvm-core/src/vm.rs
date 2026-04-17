// zkvm-core executor wired to the rebuilt RV32IM decoder.

use core::fmt;

use rv32im_decoder::{
    decode, div as rv_div, divu as rv_divu, mul as rv_mul, mulh as rv_mulh,
    mulhsu as rv_mulhsu, mulhu as rv_mulhu, rem as rv_rem, remu as rv_remu, DecodeError,
    DecodedInstruction, Instruction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkvmError {
    Decode(DecodeError),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(error) => write!(f, "decode error: {error}"),
        }
    }
}

impl From<DecodeError> for ZkvmError {
    fn from(value: DecodeError) -> Self {
        Self::Decode(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zkvm {
    regs: [u32; 32],
    pc: u32,
}

impl Default for Zkvm {
    fn default() -> Self {
        Self::new()
    }
}

impl Zkvm {
    pub const REGISTER_COUNT: usize = 32;

    #[inline]
    pub const fn new() -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
        }
    }

    #[inline]
    pub const fn pc(&self) -> u32 {
        self.pc
    }

    #[inline]
    pub fn registers(&self) -> &[u32; 32] {
        &self.regs
    }

    #[inline]
    pub fn read_reg(&self, index: usize) -> u32 {
        if index == 0 {
            0
        } else {
            self.regs[index]
        }
    }

    #[inline]
    pub fn write_reg(&mut self, index: usize, value: u32) {
        if index != 0 {
            self.regs[index] = value;
        }
    }

    pub fn execute(&mut self, decoded: DecodedInstruction) {
        let lhs = self.read_reg(decoded.rs1 as usize);
        let rhs = self.read_reg(decoded.rs2 as usize);

        let value = match decoded.instruction {
            Instruction::Add => lhs.wrapping_add(rhs),
            Instruction::Sub => lhs.wrapping_sub(rhs),
            Instruction::Sll => lhs.wrapping_shl(rhs & 0x1F),
            Instruction::Slt => ((lhs as i32) < (rhs as i32)) as u32,
            Instruction::Sltu => (lhs < rhs) as u32,
            Instruction::Xor => lhs ^ rhs,
            Instruction::Srl => lhs.wrapping_shr(rhs & 0x1F),
            Instruction::Sra => ((lhs as i32) >> (rhs & 0x1F)) as u32,
            Instruction::Or => lhs | rhs,
            Instruction::And => lhs & rhs,
            Instruction::Mul => rv_mul(lhs, rhs),
            Instruction::Mulh => rv_mulh(lhs, rhs),
            Instruction::Mulhsu => rv_mulhsu(lhs, rhs),
            Instruction::Mulhu => rv_mulhu(lhs, rhs),
            Instruction::Div => rv_div(lhs, rhs),
            Instruction::Divu => rv_divu(lhs, rhs),
            Instruction::Rem => rv_rem(lhs, rhs),
            Instruction::Remu => rv_remu(lhs, rhs),
        };

        self.write_reg(decoded.rd as usize, value);
        self.regs[0] = 0;
    }

    pub fn step(&mut self, word: u32) -> Result<(), ZkvmError> {
        let decoded = decode(word)?;
        self.execute(decoded);
        self.pc = self.pc.wrapping_add(4);
        self.regs[0] = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode_r(funct7: u8, rs2: u8, rs1: u8, funct3: u8, rd: u8) -> u32 {
        ((funct7 as u32) << 25)
            | ((rs2 as u32) << 20)
            | ((rs1 as u32) << 15)
            | ((funct3 as u32) << 12)
            | ((rd as u32) << 7)
            | 0b0110011
    }

    #[test]
    fn executes_mul_div_and_rem_correctly() {
        let mut vm = Zkvm::new();
        vm.write_reg(1, 6);
        vm.write_reg(2, 7);

        vm.step(encode_r(0b0000001, 2, 1, 0b000, 3)).unwrap();
        assert_eq!(vm.read_reg(3), 42);

        vm.step(encode_r(0b0000001, 2, 1, 0b100, 4)).unwrap();
        assert_eq!(vm.read_reg(4), 0);

        vm.step(encode_r(0b0000001, 2, 1, 0b110, 5)).unwrap();
        assert_eq!(vm.read_reg(5), 6);
    }

    #[test]
    fn x0_remains_zero() {
        let mut vm = Zkvm::new();
        vm.write_reg(1, 1);
        vm.write_reg(2, 2);
        vm.step(encode_r(0b0000000, 2, 1, 0b000, 0)).unwrap();
        assert_eq!(vm.read_reg(0), 0);
    }
}
