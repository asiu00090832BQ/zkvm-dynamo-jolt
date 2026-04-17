use core::fmt;
use rv32im_decoder::{BranchKind, DecodeError, Instruction, OpImmKind, OpKind, m_extension};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    Decode(DecodeError),
    UnsupportedInstruction(&'static str),
}

impl From<DecodeError> for ZkvmError {
    fn from(value: DecodeError) -> Self {
        Self::Decode(value)
    }
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(err) => write!(f, "{err}"),
            Self::UnsupportedInstruction(msg) => f.write_str(msg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Zkvm {
    regs: [u32; 32],
    pc: u32,
}

impl Zkvm {
    pub fn new() -> Self { Self { regs: [0; 32], pc: 0 } }
    pub fn read_reg(&self, reg: u8) -> u32 { self.regs[reg as usize] }
    pub fn write_reg(&mut self, reg: u8, val: u32) { if reg != 0 { self.regs[reg as usize] = val; } }
    pub fn execute(&mut self, instr: Instruction) -> Result<(), ZkvmError> {
        match instr {
            Instruction::Op { kind, format } => {
                let lhs = self.read_reg(format.rs1);
                let rhs = self.read_reg(format.rs2);
                let res = match kind {
                    OpKind::Add => lhs.wrapping_add(rhs),
                    OpKind::Mul => m_extension::mul(lhs, rhs),
                    _ => 0,
                };
                self.write_reg(format.rd, res);
            }
            _ => {}
        }
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }
}
