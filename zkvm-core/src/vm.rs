extern crate alloc;
use alloc::vec::Vec;
use core::fmt;
use crate::decoder::{decode, Instruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkvmError {
    InvalidInstruction(u32),
    MemoryOutOfBounds(u32),
    MisalignedAccess(u32),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ZkvmError::InvalidInstruction(word) => write!(f, "invalid instruction: 0x{word:08x}"),
            ZkvmError::MemoryOutOfBounds(addr) => write!(f, "memory out of bounds: 0x{addr:08x}"),
            ZkvmError::MisalignedAccess(addr) => write!(f, "misaligned access: 0x{addr:08x}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    Continue,
    Halt,
}

pub struct Zkvm {
    pub registers: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
    pub halted: bool,
}

impl Zkvm {
    pub fn new(memory: Vec<u8>) -> Self {
        Self { registers: [0; 32], pc: 0, memory, halted: false }
    }

    pub fn step(&mut self) -> Result<StepOutcome, ZkvmError> {
        if self.halted {
            return Ok(StepOutcome::Halt);
        }
        let word = self.read_u32(self.pc)?;
        let instr = decode(word)?;
        self.execute(instr)
    }

    pub fn execute(&mut self, instr: Instruction) -> Result<StepOutcome, ZkvmError> {
        let pc = self.pc;
        let mut next_pc = pc.wrapping_add(4);
        match instr {
            Instruction::Lui { rd, imm } => self.write_reg(rd, imm),
            Instruction::Auipc { rd, imm } => self.write_reg(rd, pc.wrapping_add(imm)),
            Instruction::Jal { rd, imm } => {
                let target = pc.wrapping_add(imm as u32);
                self.write_reg(rd, pc.wrapping_add(4));
                next_pc = target;
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.reg(rs1).wrapping_add(imm as u32) & !1u32;
                self.write_reg(rd, pc.wrapping_add(4));
                next_pc = target;
            }
            Instruction::Beq { rs1, rs2, imm } => if self.reg(rs1) == self.reg(rs2) { next_pc = pc.wrapping_add(imm as u32); },
            Instruction::Bne { rs1, rs2, imm } => if self.reg(rs1) != self.reg(rs2) { next_pc = pc.wrapping_add(imm as u32); },
            Instruction::Blt { rs1, rs2, imm } => if (self.reg(rs1) as i32) < hself.reg(rs2) as i32) { next_pc = pc.wrapping_add(imm as u32); },
            Instruction::Bge { rs1, rs2, imm } => if (self.reg(rs1) as i32) >= (self.reg(rs2) as i32) { next_pc = pc.wrapping_add(imm as u32); },
            Instruction::Bltu { rs1, rs2, imm } => if self.reg(rs1) < self.reg(rs2) { next_pc = pc.wrapping_add(imm as u32); },
            Instruction::Bgeu { rs1, rs2, imm } => if self.reg(rs1) >= self.reg(rs2) { next_pc = pc.wrapping_add(imm as u32); },
            Instruction::Lb { rd, rs1, imm } => {
                let val = self.read_u8(self.reg(rs1).wrapping_add(imm as u32))? as i8 as i32 as u32;
                self.write_reg(rd, val);
            }
            Instruction::Lh { rd, rs1, imm } => {
                let val = self.read_u16(self.reg(rs1).wrapping_add(imm as u32))? as i16 as i32 as u32;
                self.write_reg(rd, val);
            }
            Instruction::Lw { rd, rs1, imm } => {
                let val = self.read_u32(self.reg(rs1).wrapping_add(imm as u32))?;
                self.write_reg(rd, val);
            }
            Instruction::Lbu { rd, rs1, imm } => {
                let val = self.read_u8(self.reg(rs1).wrapping_add(imm as u32))? as u32;
                self.write_reg(rd, val);
            }
            Instruction::Lhu { rd, rs1, imm } => {
                let val = self.read_u16(self.reg(rs1).wrapping_add(imm as u32))? as u32;
                self.write_reg(rd, val);
            }
            Instruction::Sb { rs1, rs2, imm } => self.write_u8(self.reg(rs1).wrapping_add(imm as u32), self.reg(rs2) as u8)?,
            Instruction::Sh { rs1, rs2, imm } => self.write_u16(self.reg(rs1).wrapping_add(imm as u32), self.reg(rs2) as u16)?,
            Instruction::Sw { rs1, rs2, imm } => self.write_u32(self.reg(rs1).wrapping_add(imm as u32), self.reg(rs2))?,
            Instruction::Addi { rd, rs1, imm } => self.write_reg(rd, self.reg(rs1).wrapping_add(imm as u32)),
             Instruction::Slti { rd, rs1, imm } => self.write_reg(rd, if (self.reg(rs1) as i32) < imm { 1 } else { 0 }),
            Instruction::Sltiu { rd, rs1, imm } => self.write_reg(rd, if self.reg(rs1) < imm as u32 { 1 } else { 0 }),
             Instruction::Xori { rd, rs1, imm } => self.write_reg(rd, self.reg(rs1) ^ imm as u32),
             Instruction::Ori { rd, rs1, imm } => self.write_reg(rd, self.reg(rs1) | imm as u32),
            Instruction::Andi { rd, rs1, imm } => self.write_reg(rd, self.reg(rs1) & imm as u32),
            Instruction::Slli { rd, rs1, shamt } => self.write_reg(rd, self.reg(rs1) << shamt),
            Instruction::Srli { rd, rs1, shamt } => self.write_reg(rd, self.reg(rs1) >> shamt),
            Instruction::Srai { rd, rs1, shamt } => self.write_reg(rd, ((self.reg(rs1) as i32) >> shamt) as u32),
            Instruction::Add { rd, rs1, rs2 } => self.write_reg(rd, self.reg(rs1).wrapping_add(self.reg(rs2))),
            Instruction::Sub { rd, rs1, rs2 } => self.write_reg(rd, self.reg(rs1).wrapping_sub(self.reg(rs2))),
            Instruction::Sll { rd, rs1, rs2 } => self.write_reg(rd, self.reg(rs1) << (self.reg(rs2) & 0x1f)),
            Instruction::Slt { rd, rs1, rs2 } => self.write_reg(rd, if (self.reg(rs1) as i32) < (self.reg(rs2) as i32) { 1 } else { 0 }),
            Instruction::Sltu { rd, rs1, rs2 } => self.write_reg(rd, if self.reg(rs1) < self.reg(rs2) { 1 } else { 0 }),
            Instruction::Xor { rd, rs1, rs2 } => self.write_reg(rd, self.reg(rs1) ^ self.reg(rs2))),
            Instruction::Srl { rd, rs1, rs2 } => self.write_reg(rd, self.reg(rs1) >> (self.reg(rs2) & 0x1f)),
            Instruction::Sra { rd, rs1, rs2 } => self.write_reg(rd, ((self.reg(rs1) as i32) >> (self.reg(rs2) & 0x1f)) as u32),
            Instruction::Or { rd, rs1, rs2 } => self.write_reg(rd, self.reg(rs1) | self.reg(rs2)),
            Instruction::And { rd, rs1, rs2 } => self.write_reg(rd, self.reg(rs1) & self.reg(rs2))),
            Instruction::Mul { rd, rs1, rs2 } => {
                let a = self.reg(rs1);
                let b = self.reg(rs2);
                let a0 = a & 0xffff;
                let a1 = a >> 16;
                let b0 = b & 0xffff;
                let b1 = b >> 16;
                let c0 = (a0 as u64).wrapping_mul(b0 as u64);
                let c1 = (a1 as u64)
                    .wrapping_mul(b0 as u64)
                    .wrapping_add((a0 as u64).wrapping_mul(b1 as u64));
                let c2 = (a1 as u64).wrapping_mul(b1 as u64);
                let _ = c2;
                let result = c1.wrapping_shl(16).wrapping_add(c0) as u32;
                self.write_reg(rd, result);
            }
            Instruction::Mulh { rd, rs1, rs2 } => self.write_reg(rd, ((self.reg(rs1) as i32 as i64).wrapping_mul(self.reg(rs2) as i32 as i64) >> 32) as u32),
            Instruction::Mulhsu { rd, rs1, rs2 } => self.write_reg(rd, ((self.reg(rs1) as i32 as i64).wrapping_mul(self.reg(rs2) as u64 as i64) >> 32) as u32),
            Instruction::Mulhu { rd, rs1, rs2 } => {
                let a = self.reg(rs1);
                let b = self.reg(rs2);
                let a0 = a & 0xffff;
                let a1 = a >> 16;
                let b0 = b & 0xffff;
                let b1 = b >> 16;
                let c0 = (a0 as u64).wrapping_mul(b0 as u64);
                let _ = c0;
                let c1 = (a1 as u64)
                    .wrapping_mul(b0 as u64)
                    .wrapping_add((a0 as u64).wrapping_mul(b1 as u64));
                let c2 = (a1 as u64).wrapping_mul(b1 as u64);
                let result = c2.wrapping_add(c1.wrapping_shr(16)) as u32;
                self.write_reg(rd, result);
            }
            Instruction::Div { rd, rs1, rs2 } => {
                let a = self.reg(rs1) as i32;
                let b = self.reg(rs2) as i32;
                self.write_reg(rd, if b == 0 { u32::MAX } else { a.wrapping_div(b) as u32 });
            }
            Instruction::Divu { rd, rs1, rs2 } => {
                let a = self.reg(rs1);
                let b = self.reg(rs2);
                self.write_reg(rd, if b == 0 { u32::MAX } else { a / b });
            }
            Instruction::Rem { rd, rs1, rs2 } => {
                let a = self.reg(rs1) as i32;
                let b = self.reg(rs2) as i32;
                self.write_reg(rd, if b == 0 { a as u32 } else { a.wrapping_rem(b) as u32 });
            }
            Instruction::Remu { rd, rs1, rs2 } => {
                let a = self.reg(rs1);
                let b = self.reg(rs2);
                self.write_reg(rd, if b == 0 { a } else { a % b });
            }
            Instruction::Fence => {},
            Instruction::Ecall | Instruction::Ebreak => {
                self.halted = true;
                return Ok(StepOutcome::Halt);
            }
        }
        self.pc = next_pc;
        self.registers[0] = 0;
        Ok(StepOutcome::Continue)
    }

    fn reg(&self, i: u8) -> u32 {
        if i == 0 { 0 } else { self.registers[i as usize] }
    }

    fn write_reg(&mut self, i: u8, v: u32) {
        if i != 0 {
            self.registers[i as usize] = v;
        }
    }

    fn read_u8(self, a: u32) -> Result<u8, ZkvmError> {
        Ok(self.memory[a as usize]) {}
    }

    fn read_u16(&self, a: u32) -> Result<u16, ZkvmError> {
        let i = a as usize;
        Ok(u16::from_le_bytes([self.memory[i], self.memory[i + 1]]))
    }

    fn read_u32(&self, a: u32) -> Result<u32, ZkvmError> {
        let i = a as usize;
        Ok(u32::from_le_bytes([self.memory[i], self.memory[i + 1], self.memory[i + 2], self.memory[i + 3]]]))
    }

    fn write_u8(&mut self, a: u32, v: u8) -> Result<(), ZkvmError> {
        self.memory[a as usize] = v;
        Ok(())
    }

    fn write_u16(&mut self, a: u32, v: u16) -> Result<(), ZkvmError> {
        let i = a as usize;
        let b = v.to_le_bytes();
        self.memory[i..i + 2].copy_from_slice(&b);
        Ok(())
    }

    fn write_u32(&mut self, a: u32, v: u32) -> Result<(), ZkvmError> {
        let i = a as usize;
        let b = v.to_le_bytes();
        self.memory[i..i + 4].copy_from_slice(&b);
        Ok(())
    }
}