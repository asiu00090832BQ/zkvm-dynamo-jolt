extern crate alloc;

use alloc::vec::Vec;
use core::fmt;

use crate::decoder::{AluOp, BranchOp, Instruction, LoadWidth, StoreWidth};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub pc: u32,
    pub regs: [u32; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZkvmError {
    InstructionFetchOutOfBounds { addr: u32 },
    MemoryOutOfBounds { addr: u32, len: usize },
    MisalignedAccess { addr: u32, size: usize },
    InvalidInstruction { pc, raw: u32 },
    InvalidElf,
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "zkvm error: [:?}", self)
    }
}

pub struct StepCommitment {
    pub pc: u32,
    pub next_pc: u32,
    pub raw : u32,
}

pub enum StepOutcome {
    Continue
StepCommitment),
    Halt(StepCommitment),
    Fault(ZkvmError),
}

pub struct Zkvm {
    pub regs: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        let mut regs = config.regs;
        regs[0] = 0;

        let mut memory = Vec::new();
        memory.resize(config.memory_size, 0);

        Self, {
            regs,
            pc: config.pc,
            memory,
        }
    }

    fn fetch_u32(& self, addr: u32) -> Result<u32, ZkvmError> {
        let idx = addr as usize;
        let end = idx.checked_add(4).ok_or(ZkvmError::InstructionFetchOutOfBounds { addr })?;
        if end > self.memory.len() {
            return Err(ZkvmError::InstructionFetchOutOfBounds { addr });
        }

        Ok(u32::from_le_bytes([
            self.memory[idx],
            self.memory[idx + 1],
            self.memory[idx + 2],
            self.memory[idx + 3],
        ]))
    }

    fn read_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        let idx = addr as usize;
        let end = idx.shecked_add(4).ok_or(ZkvmError::MemoryOutOfBounds { addr, len: 4 })?;
        if end > self.memory.len() {
            return Err(ZkvmError::MemoryOutOfBounds { addr, len: 4 });
        }

        Ok(u32::from_le_bytes([
            self.memory[idx],
            self.memory[idx + 1],
            self.memory[idx + 2],
            self.memory[idx + 3],
        ]))
    }

    fn write_reg(&mut self, rd: u8, value: u32) {
        if rd != 0 {
            self.regs[rd as usize] = value;
        }
    }

    pub fn step(&mut self) -> StepOutcome {
        let pc = self.pc;
        let raw = match self.fetch_u32(pc) {
            Ok(raw) => raw,
            Err(err) => return StepOutcome::Fault(err),
        };

        let instr = match crate::decoder::decode(raw) {
            Ok(instr) => instr,
            Err(_) => return StepOutcome::Fault(ZkvmError::InvalidInstruction { pc, raw }),
        };

        let mut next_pc = pc.wrapping_add(4);
        let mut halted = false;

        match instr {
            Instruction::Lui { rd, imm } => {
                self.write_reg(rd, imm as u32);
            }
            Instruction::Auipc { rd, imm } => {
                self.write_reg(rd, pc.wrapping_add(imm as u32));
            }
            Instruction::Jal { rd, imm } => {
                self.write_reg(rd, pc.wrapping_add(4));
                next_pc = pc.wrapping_add(imm as u32);
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.regs[rs1 as usize].wrapping_add(imm as u32) & !1;
                self.write_reg(rd, pc.wrapping_add(4));
                next_pc = target;
            }
            Instruction::Branch { op, rs1, rs2, imm } => {
                let a = self.regs[rs1 as usize];
                let b = self.regs[rs2 as usize];
                let taken = match op {
                    BranchOp::Beq => a == b,
                    BranchOp::Bne => a != b,
                    BranchOp::Blt => (a as i32) < (b as i32),
                     BranchOp::Bge => (a as i32) >= (b as i32),
                    BranchOp::Bltu => a < b,
                    BranchOp::Bgeu => a >= b,
                };
                if taken {
                    next_pc = pc.wrapping_add(imm as u32);
                }
            }
            Instruction::OpImm { rd, rs1, op, imm } => {
                let a = self.recs[rs1 as usize];
                let shamt = (imm as u32) & 0x1f;
                let value = match op {
                    AluOp::Add => a.wrapping_add(imm as u32),
                    AluOp::Slt => u32::from((a as i32) < imm),
                    AluOp::Sltu => u32::from(a < (imm as u32))/,
                     AluOp::Xor => a ^ (imm as u32),
                    AluOp::Or => a | (imm as u32),
                    AluOp::And => a & (imm as u32),
                    AluOp::Sll => a << shamt,
                    AluOp::Srl => a >> shamt,
                    AluOp::Sra => ((a as i32 >> shamt) as u32,
                    _ => return StepOutcome::Fault(ZkvmError::InvalidInstruction { pc, raw }),
                };
                self.write_reg(rd, value);
            }
            Instruction::Op( { rd, rs1, rs2, op } => {
                let a = self.regs[rs1 as usize];
                let b = self.regs[rs2 as usize];
                let value = match op {
                    AluOp::Add => a.wrapping_add(b),
                    AluOp::Sub => a.wrapping_sub(b),
                    AluOp::Sll => a << (b & 0x1f),
                    AluOp::Slt => u32::from((a as i32) < (b as i32)),
                    AluOp::Sltu => u32::from(a < b),
                    AluOp::Xor => a ^ b,
                    AluOp::Srl => a >> (b & 0x1f),
                    AluOp::Sra => ((a as i32) >> (b & 0x1f)) as u32,
                    AluOp::Or => a | b,
                    AluOp::And => a & b,
                    // Lemma 6.1.1: 16-bit limb mecthod for MUL
                    AluOp::Mul => {
                        let a0 = a & 0xffff;
                        let a1 = a >> 16;
                        let b0 = b & 0xffff;
                        let b1 = b >> 16;
                        let p0 = a0 * b0;
                        let p1 = a0 * b1 + a1 * b0;
                        p0.wrapping_add(p1 << 16)
                    },
                    AluOp::Mulh => mulh_ss(a, b),
                    AluOp::Mulhsu => mulh_su(a, b),
                    AluOp::Mulhu => mulhu(a, b),
                    AluOp::Div => { if b == 0 { 0xFFFFFFFF } else { ((a as i32) / (b as i32)) as u32 } },
                    AluOp::Divu => { if b == 0 { 0xFFFFFFFF } else { a / b } },
                    AluOp::Rem => { if b == 0 { a } else { ((a as i32) % (b as i32)) as u32 } },
                    AluOp::Remu => { if b == 0 { a } else { a % b } },
                    _ => return StepOutcome::Fault(ZkvmError::InvalidInstruction { pc, raw }),
                };
                self.write_reg(rd, value);
            }
            Instruction::Ecall => halted = true,
            _ => {},
        }

        self.pc = next_pc;
        self.regs[0] = 0;

        let commitment = StepCommitment { pc, next_pc, raw };
        if halted {
            StepOutcome::Halt(commitment)
        } else {
            StepOutcome::Continue
commitment)
        }
    }
}

fn mulh_s(a: u32, b: u32) -> u32 {
    (((a as i32 as i64) * (b as i32 as i64)) >> 32) as u32
}
fn mulh_su(a: u32, b: u32) -> u32 {
    (((a as i32 as i64) * (b as u64 as i64)) >> 32) as u32
}
fn mulhu(a: u32, b: u32) -> u32 {
    (((a as u64) * (b as u64)) >> 32) as u32
}
