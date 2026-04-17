use core::fmt;
use crate::decoder::{Instruction, MulDivKind};

pub const REGISTER_COUNT: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ZkvmError {
    Halted,
    StepLimitExceeded,
    InvalidRegister(usize),
    InvalidInstruction(u32),
    UnsupportedInstruction(u32),
    InvalidElf,
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZkvmError::Halted => write!(f, "Zkvm is halted"),
            ZkvmError::StepLimitExceeded => write!(f, "step limit exceeded"),
            ZkvmError::InvalidRegister(idx) => write!(f, "invalid register: {}", idx),
            ZkvmError::InvalidInstruction(w) => write!(f, "invalid instruction: 0x{:08x}", w),
            ZkvmError::UnsupportedInstruction(w) => write!(f, "unsupported instruction: 0x{:08x}", w),
            ZkvmError::InvalidElf => write!(f, "invalid ELF file"),
        }
    }
}

pub struct Zkvm {
    pub regs: [u32; REGISTER_COUNT],
    pub pc: u32,
}

impl Zkvm {
    pub fn read_reg(&self, index: usize) -> u32 {
        if index == 0 { 0 } else { self.regs[index] }
    }

    pub fn write_reg(&mut self, index: usize, value: u32) {
        if index != 0 { self.regs[index] = value; }
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<(), ZkvmError> {
        match instruction {
            Instruction::MulDiv { kind, rd, rs1, rs2 } => {
                let lhs = self.read_reg(rs1 as usize);
                let rhs = self.read_reg(rs2 as usize);
                let val = match kind {
                    MulDivKind::Mul => mul_u32_limb(lhs, rhs) as u32,
                    MulDivKind::Mulh => (mul_i32_i32_limb(lhs as i32, rhs as i32) >> 32) as u32,
                    MulDivKind::Mulhsu => (mul_i32_u32_limb(lhs as i32, rhs) >> 32) as u32,
                    MulDivKind::Mulhu => (mul_u32_limb(lhs, rhs) >> 32) as u32,
                    MulDivKind::Div => if rhs == 0 { u32::MAX } else if (lhs as i32) == i32::MIN && (rhs as i32) == -1 { lhs } else { ((lhs as i32) / (rhs as i32)) as u32 },
                    MulDivKind::Divu => if rhs == 0 { u32::MAX } else { lhs / rhs },
                    MulDivKind::Rem => if rhs == 0 { lhs } else if (lhs as i32) == i32::MIN && (rhs as i32) == -1 { 0 } else { ((lhs as i32) % (rhs as i32)) as u32 },
                    MulDivKind::Remu => if rhs == 0 { lhs } else { lhs % rhs },
                };
                self.write_reg(rd as usize, val);
            }
            _ => {}
        }
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }
}

fn mul_u32_limb(a: u32, b: u32) -> u64 {
    let a0 = (a & 0xffff) as u64;
    let a1 = (a >> 16) as u64;
    let b0 = (b & 0xffff) as u64;
    let b1 = (b >> 16) as u64;
    a0 * b0 + ((a0 * b1 + a1 * b0) << 16) + ((a1 * b1) << 32)
}

fn mul_i32_i32_limb(a: i32, b: i32) -> i64 {
    let neg = (a < 0) ^ (b < 0);
    let p = mul_u32_limb(abs_i32(a), abs_i32(b));
    if neg { (p as i64).wrapping_neg() } else { p as i64 }
}

fn mul_i32_u32_limb(a: i32, b: u32) -> i64 {
    let p = mul_u32_limb(abs_i32(a), b);
    if a < 0 { (p as i64).wrapping_neg() } else { p as i64 }
}

fn abs_i32(x: i32) -> u32 {
    if x < 0 { (x as u32).wrapping_neg() } else { x as u32 }
}