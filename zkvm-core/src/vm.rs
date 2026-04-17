use core::fmt;
use rv32im_decoder::{decode, div, divu, mul, mulh, mulhsu, mulhu, rem, remu, DecodedInstruction, DecoderError, Instruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkvmError {
    Decode(DecoderError),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(err) => write!(f, "decode error: {err}"),
        }
    }
}

impl From<DecoderError> for ZkvmError {
    fn from(err: DecoderError) -> Self {
        Self::Decode(err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zkvm {
    pub regs: [u32; 32],
    pub pc: u32,
}

impl Zkvm {
    pub fn new() -> Self { Self { regs: [0; 32], pc: 0 } }

    pub fn read_reg(&self, index: usize) -> u32 { if index == 0 { 0 } else { self.regs[index] } }

    pub fn write_reg(&mut self, index: usize, value: u32) { if index != 0 { self.regs[index] = value; } }

    pub fn execute(&mut self, decoded: DecodedInstruction) {
        let rs1 = decoded.rs1.unwrap_or(0);
        let rs2 = decoded.rs2.unwrap_or(0);
        let lhs = self.read_reg(rs1 as usize);
        let rhs = self.read_reg(rs2 as usize);

        let value = match decoded.instruction {
            Instruction::Add => lhs.wrapping_add(rhs),
            Instruction::Sub => lhs.wrapping_sub(rhs),
            Instruction::Mul => mul(lhs, rhs),
            Instruction::Mulh => mulh(lhs, rhs),
            Instruction::Mulhsu => mulhsu(lhs, rhs),
            Instruction::Mulhu => mulhu(lhs, rhs),
            Instruction::Div => div(lhs, rhs),
            Instruction::Divu => divu(lhs, rhs),
            Instruction::Rem => rem(lhs, rhs),
            Instruction::Remu => remu(lhs, rhs),
            _ => 0, // Placeholder for other instructions
        };

        if let Some(rd) = decoded.rd {
            self.write_reg(rd as usize, value);
        }
    }

    pub fn step(&mut self, word: u32) -> Result<(), ZkvmError> {
        let decoded = decode(word)?;
        self.execute(decoded);
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }
}
