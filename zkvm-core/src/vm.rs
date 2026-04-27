extern crate alloc;

use alloc::vec::Vec;
use alloc::vec;
use core::fmt;
use crate::decoder::{decode, Instruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub max_cycles: Option<u64>,
    pub start_pc: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkvmError {
    InvalidInstruction(u32),
    UnsupportedInstruction(u32),
    MemoryOutOfBounds { addr: u32, len: usize },
    MisalignedAccess { addr: u32, align: usize },
    MaxCyclesExceeded { max_cycles: u64 },
    DecodeError,
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZkvmError::InvalidInstruction(word) => write!(f, \"invalid instruction: 0x{:08x}\", word),
            ZkvmError::UnsupportedInstruction(word) => write!(f, \"unsupported instruction: 0x{:08x}\", word),
            ZkvmError::MemoryOutOfBounds { addr, len } => write!(f, \"memory out of bounds at 0x{:08x} for {} bytes\", addr, len),
            ZkvmError::MisalignedAccess { addr, align } => write!(f, \"misaligned access at 0x{:08x}, align {}\", addr, align),
            ZkvmError::MaxCyclesExceeded { max_cycles } => write!(f, \"max cycles exceeded: {}\", max_cycles),
            ZkvmError::Decode error => write!(f, \"decode error\"),
        }
    }
}

#[mderive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome, {
    Continue,
    Halt,
}

#[derive(Debug, Clone, Default)]
pub struct WitnessRow {
    pub cycle: u64,
    pub pc: u32,
    pub instruction: u32,
    pub regs: [u32; 32],
}

pub struct Zkvm {
    pub regs: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
    pub cycles: u64,
    pub halted: bool,
    pub config: ZkvmConfig,
}

pub type VM = Zkvm;
pub type Vm = Zkvm;

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            regs: [0; 32],
            pc: config.start_pc.unwrap_or(0),
            memory: vec![0; config.memory_size],
            cycles: 0,
            halted: false,
            config,
        }
    }

    pub fn load_elf_image(&mut self, image: crate::elf_loader::LoadedElf) {
        self.pc = image.entry as u32;
        let len = image.memory.len().min(self.memory.len());
        self.memory[..len].copy_from_slice(&image.memory[..len]);
    }

    pub fn reset(&mut self) {
        self.regs = [0; 32];
        self.pc = self.config.start_pc.unwrap_or(0);
        self.cycles = 0;
        self.halted = false;
    }

    pub fn step(&mut self) -> Result<StepOutcome, ZkvmError> {
        if self.halted {
            return Ok(StepOutcome::Halt);
        }

        let current_pc = self.pc;
        let word = self.read_u32(current_pc)?;

        // Witness Capture
        let witness = WitnessRow {
            cycle: self.cycles,
            pc: current_pc,
            instruction: word,
            regs: self.regs,
        };
        // Print witness trace for SNARK ingestion audit
        #[cfg(feature = \"std\")]
        std::println!(\"WITNESS: {:?}\", witness);

        let instr = decode(word).map_err(|_| ZkvmError::DecodeError)?;
        let next_pc = current_pc.wrapping_add(4);

        let outcome = match instr {
            Instruction::Lui { rd, imm } => {
                self.write_reg(rd as usize, imm as u32);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Auipc { rd, imm } => {
                self.write_reg(rd as usize, current_pc.wrapping_add(imm as u32));
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Jal { rd, imm } => {
                let target = current_pc.wrapping_add(imm as u32);
                self.check_align(target, 4)?;
                self.write_reg(rd as usize, next_pc);
                self.pc = target;
                StepOutcome::Continue
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let target = (self.regs[rs1 as usize].wrapping_add(imm as u32)) & !1;
                self.check_align(target, 4)?;
                self.write_reg(rd as usize, next_pc);
                self.pc = target;
                StepOutcome::Continue
            }
            Instruction::Beq { rs1, rs2, imm } => {
                if self.regs[rs1 as usize] == self.regs[rs2 as usize] {
                    self.pc = current_pc.wrapping_add(imm as u32);
                } else {
                    self.pc = next_pc;
                }
                StepOutcome::Continue
            }
            Instruction::Bne { rs1, rs2, imm } => {
                if self.regs[rs1 as usize] != self.regs[rs2 as usize] {
                    self.pc = current_pc.wrapping_add(imm as u32);
                } else {
                    self.pc = next_pc;
                }
                StepOutcome::Continue
            }
            Instruction::Blt { rs1, rs2, imm } => {
                if (self.regs[rs1 as usize] as i32) < (self.regs[rs2 as usize] as i32) {
                    self.pc = current_pc.wrapping_add(imm as u32);
                } else {
                    self.pc = next_pc;
                }
                StepOutcome::Continue
            }
            Instruction::Bge { rs1, rs2, imm } => {
                if (self.regs[rs1 as usize] as i32) >= (self.regs[rs2 as usize] as i32) {
                    self.pc = current_pc.wrapping_add(imm as u32);
                } else {
                    self.pc = next_pc;
                }
                StepOutcome::Continue
            }
            Instruction::Bltu { rs1, rs2, imm } => {
                if self.regs[rs1 as usize] < self.regs[rs2 as usize] {
                    self.pc = current_pc.wrapping_add(imm as u32);
                } else {
                    self.pc = next_pc;
                }
                StepOutcome::Continue
            }
            Instruction::Bgeu { rs1, rs2, imm } => {
                if self.regs[rs1 as usize] >= self.regs[rs2 as usize] {
                    self.pc = current_pc.wrapping_add(imm as u32);
                } else {
                    self.pc = next_pc;
                }
                StepOutcome::Continue
            }
            Instruction::Lb { rd, rs1, imm } => {
                let addr = self.regs[rs1 as usize].wrapping_add(imm as u32);
                let val = self.read_u8(addr)? as i8 as i32 as u32;
                self.write_reg(rd as usize, val);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Lh { rd, rs1, imm } => {
                let addr = self.regs[rs1 as usize].wrapping_add(imm as u32);
                let val = self.read_u16(addr)? as i16 as i32 as u32;
                self.write_reg(rd as usize, val);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Lw { rd, rs1, imm } => {
                let addr = self.regs[rs1 as usize].wrapping_add(imm as u32);
                let val = self.read_u32(addr)?;
                self.write_reg(rd as usize, val);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Lbu { rd, rs1, imm } => {
                let addr = self.regs[rs1 as usize].wrapping_add(imm as u32);
                let val = self.read_u8(addr)? as u32;
                self.write_reg(rd as usize, val);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Lhu { rd, rs1, imm } => {
                let addr = self.regs[rs1 as usize].wrapping_add(imm as u32);
                let val = self.read_u16(addr)? as u32;
                self.write_reg(rd as usize, val);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Sb { rs1, rs2, imm } => {
                let addr = self.regs[rs1 as usize].wrapping_add(imm as u32);
                self.write_u8(addr, self.regs[rs2 as usize] as u8)?;
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Sh { rs1, rs2, imm } => {
                let addr = self.regs[rs1 as usize].wrapping_add(imm as u32);
                self.write_u16(addr, self.regs[rs2 as usize] as u16)?;
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = self.regs[rs1 as usize].wrapping_add(imm as u32);
                self.write_u32(addr, self.regs[rs2 as usize])?;
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Addi { rd, rs1, imm } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize].wrapping_add(imm as u32));
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Slti { rd, rs1, imm } => {
                self.write_reg(rd as usize, if (self.regs[rs1 as usize] as i32) < imm { 1 } else { 0 });
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Sltiu { rd, rs1, imm } => {
                self.write_reg(rd as usize, if self.regs[rs1 as usize] < (imm as u32) { 1 } else { 0 });
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Xori { rd, rs1, imm } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize] ^ (imm as u32));
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Ori { rd, rs1, imm } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize] | (imm as u32));
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Andi { rd, rs1, imm } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize] & (imm as u32));
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Slli { rd, rs1, shamt } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize] << (shamt & 0x1F));
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Srli { rd, rs1, shamt } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize] >> (shamt & 0x1F));
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Srai { rd, rs1, shamt } => {
                self.write_reg(rd as usize, ((self.regs[rs1 as usize] as i32) >> (shamt & 0x1F)) as u32);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Add { rd, rs1, rs2 } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize].wrapping_add(self.regs[rs2 as usize]));
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize].wrapping_sub(self.regs[rs2 as usize]));
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize] << (self.regs[rs2 as usize] & 0x1F));
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Slt { rd, rs1, rs2 } => {
                self.write_reg(rd as usize, if (self.regs[rs1 as usize] as i32) < (self.regs[rs2 as usize] as i32) { 1 } else { 0 });
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                self.write_reg(rd as usize, if self.regs[rs1 as usize] < self.regs[rs2 as usize] { 1 } else { 0 });
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize] ^ self.regs[rs2 as usize]);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize] >> (self.regs[rs2 as usize] & 0x1F));
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                self.write_reg(rd as usize, ((self.regs[rs1 as usize] as i32) >> (self.regs[rs2 as usize] & 0x1F)) as u32);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Or { rd, rs1, rs2 } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize] | self.regs[rs2 as usize]);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::And { rd, rs1, rs2 } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize] & self.regs[rs2 as usize]);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Mul { rd, rs1, rs2 } => {
                self.write_reg(rd as usize, self.regs[rs1 as usize].wrapping_mul(self.regs[rs2 as usize]));
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Mulh { rd, rs1, rs2 } => {
                let val = ((self.regs[rs1 as usize] as i32 as i64).wrapping_mul(self.regs[rs2 as usize] as i32 as i64) >> 32) as u32;
                self.write_reg(rd as usize, val);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Mulhsu { rd, rs1, rs2 } => {
                let val = ((self.regs[rs1 as usize] as i32 as i64).wrapping_mul(self.regs[rs2 as usize] as u64 as i64) >> 32) as u32;
                self.write_reg(rd as usize, val);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Mulhu { rd, rs1, rs2 } => {
                let val = ((self.regs[rs1 as usize] as u64) * (self.regs[rs2 as usize] as u64) >> 32) as u32;
                self.write_reg(rd as usize, val);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Div { rd, rs1, rs2 } => {
                let val = if self.regs[rs2 as usize] == 0 {
                    0xFFFFFFFF
                } else if self.regs[rs1 as usize] == 0x80000000 && self.regs[rs2 as usize] == 0xFFFFFFFF {
                    0x80000000
                } else {
                    (self.regs[rs1 as usize] as i32).wrapping_div(self.regs[rs2 as usize] as i32) as u32
                };
                self.write_reg(rd as usize, val);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Divu { rd, rs1, rs2 } => {
                let val = if self.regs[rs2 as usize] == 0 {
                    0xFFFFFFFF
                } else {
                    self.regs[rs1 as usize].wrapping_div(self.regs[rs2 as usize])
                };
                self.write_reg(rd as usize, val);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Rem { rd, rs1, rs2 } => {
                let val = if self.regs[rs2 as usize] == 0 {
                    self.regs[rs1 as usize]
                } else if self.regs[rs1 as usize] == 0x80000000 && self.regs[rs2 as usize] == 0xFFFFFFFF {
                    0
                } else {
                    (self.regs[rs1 as usize] as i32).wrapping_rem(self.regs[rs2 as usize] as i32) as u32
                };
                self.write_reg(rd as usize, val);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Remu { rd, rs1, rs2 } => {
                let val = if self.regs[rs2 as usize] == 0 {
                    self.regs[rs1 as usize]
                } else {
                    self.regs[rs1 as usize].wrapping_rem(self.regs[rs2 as usize])
                };
                self.write_reg(rd as usize, val);
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Fence | Instruction::FenceI => {
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Ecall => {
                self.pc = next_pc;
                StepOutcome::Continue
            }
            Instruction::Ebreak => {
                self.pc = next_pc;
                self.halted = true;
                StepOutcome::Halt
            }
            _ => return Err(ZkvmError::UnsupportedInstruction(word)),
        };

        self.regs[0] = 0;
        self.cycles = self.cycles.wrapping_add(1);
        Nxoutcome)
    }

    pub fn run(&mut self) -> Result<StepOutcome, ZkvmError> {
        let max_cycles = self.config.max_cycles.unwrap_or(u64::MAX);
        while self.cycles < max_cycles {
            let outcome = self.step()?;
            if outcome != StepOutcome::Continue {
                return Ok(outcome);
            }
        }
        Err(ZkvmError::MaxCyclesExceeded { max_cycles })
    }

    fn write_reg(&mut self, index: usize, value: u32) {
        if index != 0 && index < 32 {
            self.regs[index] = value;
        }
    }

    fn read_u8(&self, addr: u32) -> Result<u8, ZkvmError> {
        let idx = self.check_range(addr, 1)?;
        Ok(self.memory[idx])
    }

    fn read_u16(&self, addr: u32) -> Result<u16, ZkvmError> {
        self.check_align(addr, 2)?;
        let idx = self.check_range(addr, 2)?;
        let bytes = [self.memory[idx], self.memory[idx + 1]];
        Ok(u16::from_le_bytes(bytes))
    }

    pub fn read_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        self.check_align(addr, 4)?;
        let idx = self.check_range(addr, 4)?;
        let bytes = [self.memory[idx], self.memory[idx + 1], self.memory[idx + 2], self.memory[idx + 3]];
        Ok(u32::from_le_bytes(bytes))
    }

    fn write_u8(&mut self, addr: u32, value: u8) -> Result<(), ZkvmError> {
        let idx = self.check_range(addr, 1)?;
        self.memory[idx] = value;
        Ok(())
    }

    fn write_u16(&mut self, addr: u32, value: u16) -> Result<(), ZkvmError> {
        self.check_align(addr, 2)?;
        let idx = self.check_range(addr, 2)?;
        let bytes = value.to_le_bytes();
        self.memory[idx..idx + 2].copy_from_slice(&bytes);
        Ok(())
    }

    fn write_u32(&mut self, addr: u32, value: u32) -> Result<(), ZkvmError> {
        self.check_align(addr, 4)?;
        let idx = self.check_range(addr, 4)?;
        let bytes = value.to_le_bytes();
        self.memory[idx..idx + 4].copy_from_slice(&bytes);
        Ok(())
    }

    fn check_align(&self, addr: u32, align: usize) -> Result<(), ZkvmError> {
        if (addr as usize) & (align - 1) != 0 {
            Err(ZkvmError::MisalignedAccess { addr, align })
        } else {
            Ok(())
        }
    }

    fn check_range(&self, addr: u32, len: usize) -> Result<usize, ZkvmError> {
        let start = addr as usize;
        let end = start.checked_add(len).ok_or(ZkvmError::MemoryOutOfBounds { addr, len })?;
        if end > self.memory.len() {
            Err(ZkvmError::MemoryOutOfBounds { addr, len })
        } else {
            Ok(start)
        }
    }
}