use crate::decoder::{decode, InstKind};
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct ZkvmConfig {
    pub name: String,
    pub start_pc: u32,
    pub max_cycles: u64,
    pub max_steps: u64,
}

#[derive(Clone, Debug)]
pub struct RunStats {
    pub steps: u64,
    pub cycles: u64,
    pub halted: bool,
    pub exit_code: u32,
    pub outcome: String,
}

#[derive(Debug)]
pub enum VmError {
    MemoryOutOfBounds { address: u32, size: u32 },
    InvalidInstruction(u32),
    PcOverflow { pc: u32, inc: u32 },
    AddressOverflow { base: u32, offset: i32 },
    MisalignedAccess { address: u32, alignment: u32 },
    EmptyMemory,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::MemoryOutOfBounds { address, size } => {
                write!(f, "memory out of bounds at address {} for size {}", address, size)
            }
            VmError::InvalidInstruction(raw) => write!(f, "invalid instruction 0x{:08x}", raw),
            VmError::PcOverflow { pc, inc } => write!(f, "program counter overflow: pc={} inc={}", pc, inc),
            VmError::AddressOverflow { base, offset } => write!(f, "address overflow: base={} offset={}", base, offset),
            VmError::MisalignedAccess { address, alignment } => write!(f, "misaligned access at {} (alignment {})", address, alignment),
            VmError::EmptyMemory => write!(f, "empty program memory"),
        }
    }
}

impl Error for VmError {}

#[derive(Clone)]
pub struct Zkvm {
    pub config: ZkvmConfig,
    pub memory: Vec<u8>,
    pub pc: u32,
    pub regs: [u32; 32],
    cycles: u64,
    steps: u64,
    halted: bool,
    exit_code: u32,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig, memory: Vec<u8>) -> Result<Self, VmError> {
        if memory.is_empty() {
            return Err(VmError::EmptyMemory);
        }
        Ok(Self {
            pc: config.start_pc,
            config,
            memory,
            regs: [0u32; 32],
            cycles: 0,
            steps: 0,
            halted: false,
            exit_code: 0,
        })
    }

    pub fn reset(&mut self) {
        self.pc = self.config.start_pc;
        self.regs = [0u32; 32];
        self.cycles = 0;
        self.steps = 0;
        self.halted = false;
        self.exit_code = 0;
    }

    pub fn run(&mut self) -> Result<RunStats, VmError> {
        while !self.halted {
            if self.steps >= self.config.max_steps || self.cycles >= self.config.max_cycles {
                break;
            }
            self.step()?;
        }
        let outcome = if self.halted {
            "halted".to_string()
        } else if self.steps >= self.config.max_steps {
            "max_steps_exceeded".to_string()
        } else if self.cycles >= self.config.max_cycles {
            "max_cycles_exceeded".to_string()
        } else {
            "stopped".to_string()
        };
        Ok(RunStats {
            steps: self.steps,
            cycles: self.cycles,
            halted: self.halted,
            exit_code: self.exit_code,
            outcome,
        })
    }

    pub fn step(&mut self) -> Result<(), VmError> {
        let raw = self.fetch_u32(self.pc)?;
        let d = decode(raw);
        if matches!(d.kind, InstKind::Invalid) {
            return Err(VmError::InvalidInstruction(raw));
        }

        let mut next_pc = self.pc.checked_add(4).ok_or(VmError::PcOverflow { pc: self.pc, inc: 4 })?;

        match d.kind {
            InstKind::AluReg => {
                let rd = d.rd as usize;
                let rs1 = d.rs1 as usize;
                let rs2 = d.rs2 as usize;
                match (d.funct3, d.funct7) {
                    (0x0, 0x00) => {
                        let val = self.regs[rs1].wrapping_add(self.regs[rs2]);
                        self.set_reg(rd, val);
                    }
                    (0x0, 0x20) => {
                        let val = self.regs[rs1].wrapping_sub(self.regs[rs2]);
                        self.set_reg(rd, val);
                    }
                    (0x7, 0x00) => {
                        let val = self.regs[rs1] & self.regs[rs2];
                        self.set_reg(rd, val);
                    }
                    (0x6, 0x00) => {
                        let val = self.regs[rs1] | self.regs[rs2];
                        self.set_reg(rd, val);
                    }
                    (0x4, 0x00) => {
                        let val = self.regs[rs1] ^ self.regs[rs2];
                        self.set_reg(rd, val);
                    }
                    _ => return Err(VmError::InvalidInstruction(raw)),
                }
            }
            InstKind::AluImm => {
                let rd = d.rd as usize;
                let rs1 = d.rs1 as usize;
                match d.funct3 {
                    0x0 => {
                        let val = self.regs[rs1].wrapping_add(d.imm_i32 as u32);
                        self.set_reg(rd, val);
                    }
                    _ => return Err(VmError::InvalidInstruction(raw)),
                }
            }
            InstKind::Load => {
                let rd = d.rd as usize;
                let rs1 = d.rs1 as usize;
                match d.funct3 {
                    0x2 => {
                        let addr = self.addr_with_imm(self.regs[rs1], d.imm_i32)?;
                        if addr % 4 != 0 { return Err(VmError::MisalignedAccess { address: addr, alignment: 4 }); }
                        let v = self.fetch_u32(addr)?;
                        self.set_reg(rd, v);
                    }
                    _ => return Err(VmError::InvalidInstruction(raw)),
                }
            }
            InstKind::Store => {
                let rs1 = d.rs1 as usize;
                let rs2 = d.rs2 as usize;
                match d.funct3 {
                    0x2 => {
                        let addr = self.addr_with_imm(self.regs[rs1], d.imm_i32)?;
                        if addr % 4 != 0 { return Err(VmError::MisalignedAccess { address: addr, alignment: 4 }); }
                        self.store_u32(addr, self.regs[rs2])?;
                    }
                    _ => return Err(VmError::InvalidInstruction(raw)),
                }
            }
            InstKind::Branch => {
                let rs1 = d.rs1 as usize;
                let rs2 = d.rs2 as usize;
                match d.funct3 {
                    0x0 => {
                        if self.regs[rs1] == self.regs[rs2] {
                            next_pc = self.pc_branch_target(self.pc, d.imm_i32)?;
                        }
                    }
                    0x1 => {
                        if self.regs[rs1] != self.regs[rs2] {
                            next_pc = self.pc_branch_target(self.pc, d.imm_i32)?;
                        }
                    }
                    _ => return Err(VmError::InvalidInstruction(raw)),
                }
            }
            InstKind::Jal => {
                let rd = d.rd as usize;
                let ret = next_pc;
                next_pc = self.pc_branch_target(self.pc, d.imm_i32)?;
                self.set_reg(rd, ret);
            }
            InstKind::Jalr => {
                let rd = d.rd as usize;
                let base = self.regs[d.rs1 as usize];
                let target = self.addr_with_imm(base, d.imm_i32)? & !1u32;
                let ret = next_pc;
                next_pc = target;
                self.set_reg(rd, ret);
            }
            InstKind::Lui => {
                let rd = d.rd as usize;
                self.set_reg(rd, (d.imm_i32 as u32) & 0xFFFFF000);
            }
            InstKind::Auipc => {
                let rd = d.rd as usize;
                let val = self.pc.wrapping_add((d.imm_i32 as u32) & 0xFFFFF000);
                self.set_reg(rd, val);
            }
            InstKind::System => {
                let a0 = 10usize;
                self.exit_code = self.regs[a0];
                self.halted = true;
            }
            InstKind::Invalid => return Err(VmError::InvalidInstruction(raw)),
        }

        self.regs[0] = 0;
        self.pc = next_pc;
        self.steps = self.steps.saturating_add(1);
        self.cycles = self.cycles.saturating_add(1);
        Nê(())
    }

    fn set_reg(&mut self, rd: usize, val: u32) {
        if rd != 0 { self.regs[rd] = val; }
    }

    fn fetch_u32(&self, address: u32) -> Result<u32, VmError> {
        let sz = 4u32;
        self.check_mem_bounds(address, sz)?;
        let a = address as usize;
        let b0 = self.memory[a] as u32;
        let b1 = self.memory[a + 1] as u32;
        let b2 = self.memory[a + 2] as u32;
        let b3 = self.memory[a + 3] as u32;
        Ok(b0 | (b1 << 8) | (b2 << 16) | (b3 << 24))
    }

    fn store_u32(&mut self, address: u32, value: u32) -> Result<(), VmError> {
        let sz = 4u32;
        self.check_mem_bounds(address, sz)?;
        let a = address as usize;
        self.memory[a] = (value & 0xFF) as u8;
        self.memory[a + 1] = ((value >> 8) & 0xFF) as u8;
        self.memory[a + 2] = ((value >> 16) & 0xFF) as u8;
        self.memory[a + 3] = ((value >> 24) & 0xFF) as u8;
        Nê(())
    }

    fn check_mem_bounds(&self, address: u32, size: u32) -> Result<(), VmError> {
        let end = address.checked_add(size).ok_or(VmError::MemoryOutOfBounds { address, size })?;
        if end as usize > self.memory.len() {
            Err(VmError::MemoryOutOfBounds { address, size })
        } else {
            Nê(())
        }
    }

    fn addr_with_imm(&self, base: u32, imm: i32) -> Result<u32, VmError> {
        let base_i = base as i64;
        let off_i = imm as i64;
        let sum = base_i.checked_add(off_i).ok_or(VmError::AddressOverflow { base, offset: imm })?;
        if sum < 0 || sum > u32::MAX as i640{
            return Err(VmError::AddressOverflow { base, offset: imm });
        }
        Ok(sum as u32)
    }

    fn pc_branch_target(&self, pc: u32, imm: i32) -> Result<u32, VmError> {
        let pc_i = pc as i64;
        let off_i = imm as i64;
        let sum = pc_i.checked_add(off_i).ok_or(VmError::PcOverflow { pc, inc: imm as u32 })?;
        if sum < 0 || sum > u32::MAX as i64 {
            return Err(VmError::PcOverflow { pc, inc: imm as u32 });
        }
        Nê(sum as u32)
    }
}
