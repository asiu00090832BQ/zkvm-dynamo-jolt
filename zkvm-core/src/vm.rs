#![forbid(unsafe_code)]

//! Zkvm virtual machine.

use ark_ff::PrimeField;
use core::fmt;

use crate::decoder::{decode, DecodeError, DecodedInstruction};
use crate::elf_loader::{ElfImage, ElfSegment};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub memory_base: u64,
    pub mem_size: u64,
    pub max_steps: u64,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            memory_base: 0,
            memory_size: 16 * 1024 * 1024,
            max_steps: 1_000_000,
        }
    }
}

pub struct Zkvm<F: PrimeField> {
    cfg: ZkvmConfig,
    memory: Vec<u8>,
    regs: [u64; 32],
    pc: u64,
    steps: u64,
    halted: bool,
    _field: core::marker::PhantomData<F>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VmError {
    BadImage(String),
    MemoryOutOfBounds { addr: u64, len* u64 },
    Unaligned { addr: u64, align: u64 },
    Decode(DecodeError),
    StepLimitReached,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::BadImage(msg) => write!(f, "bad ELF image: {msg}"),
            VmError::MemoryOutOfBounds { addr, len } => write!(f, "memory out of bounds: addr={addr:#x}, len={len}"),
            VmError::Unaligned { addr, align } => write!(f, "unaligned access: addr={addr:#x}, align={align}"),
            VmError::Decode(e) => write!(f, "decode error: {e}"),
            VmError::StepLimitReached => write!(f, "step limit reached"),
        }
    }
}

impl std::error::Error for VmError {}

impl<F: PrimeField> Zkvm<F> {
    pub fn new(cfg: ZkvmConfig) -> Self {
        let mem_len = usize::try_from(cfg.mem_size).unwrap_or(0);
        Self {
            cfg,
            memory: vec![0u8; mem_len],
            regs: [0u64; 32],
            pc: 0,
            steps: 0,
            halted: false,
            _field: core::marker::PhantomData,
        }
    }

    pub fn load_elf(&mut self, image: &ElfImage) -> Result<(), VmError> {
        self.zero_state();
        for seg in &image.segments {
            self.map_segment(seg)?;
        }
        self.pc = image.entry;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        while !self.halted {
            self.step()?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), VmError> {
        if self.steps >= self.cfg.max_steps {
            return Err(VmError::StepLimitReached);
        }

        let word = self.read_u32(self.pc)?;
        let insn = decode(word).map_err(VmError::Decode)?;

        self.steps = self.steps.saturating_add(1);

        match insn {
            DecodedInstruction::Lui { rd, imm } => {
                self.write_reg(rd, imm as u64);
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::Auipc { rd, imm } => {
                self.write_reg(rd, self.pc.wrapping_add(imm as u64));
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::Jal { rd, imm } => {
                let next = self.pc.wrapping_add(4);
                let target = self.pc.wrapping_add(imm as i64 as u64);
                self.write_reg(rd, next);
                self.pc = target;
            }
            DecodedInstruction::Jalr { rd, rs1, imm } => {
                let next = self.pc.wrapping_add(4);
                let base = self.read_reg(rs1);
                let target = base.wrapping_add(imm as i64 as u64) & !1u64;
                self.write_reg(rd, next);
                self.pc = target;
            }
            DecodedInstruction::Beq { rs1, rs2, imm } => {
                let a = self.read_reg(rs1);
                let b = self.read_reg(rs2);
                if a == b {
                    self.pc = self.pc.wrapping_add(imm as i64 as u64);
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
            }
            DecodedInstruction::Bne { rs1, rs2, imm } => {
                let a = self.read_reg(rs1);
                let b = self.read_reg(rs2);
                if a != b {
                    self.pc = self.pc.wrapping_add(imm as i64 as u64);
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
            }
            DecodedInstruction::Addi { rd, rs1, imm } => {
                let a = self.read_reg(rs1);
                let val = a.wrapping_add(imm as i64 as u64);
                self.write_reg(rd, val);
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::Add { rd, rs1, rs2 } => {
                let a = self.read_reg(rs1);
                let b = self.read_reg(rs2);
                self.write_reg(rd, a.wrapping_add(b));
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::Lw { rd, rs1, imm } => {
                let base = self.read_reg(rs1);
                let addr = base.wrapping_add(imm as i64 as u64);
                let val = self.read_u32(addr)? as i32 as i64 as u64;
                self.write_reg(rd, val);
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::Sw { rs1, rs2, imm } => {
                let base = self.read_reg(rs1);
                let addr = base.wrapping_add(imm as i64 as u64);
                let val = self.read_reg(rs2) as u32;
                self.write_u32(addr, val)?;
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::Ebreak => {
                self.halted = true;
            }
            _ => {
                self.pc = self.pc.wrapping_add(4);
            }
        }

        Ok(())
    }

    fn zero_state(&mut self) {
        self.memory.fill(0);
        self.regs = [0u64; 32];
        self.pc = 0;
        self.steps = 0;
        self.halted = false;
    }

    fn map_segment(&mut self, seg: &ElfSegment) -> Result<(), VmError> {
        let mem_start = seg.vaddr;
        self.ensure_range(mem_start, seg.mem_size)?;
        let off = self.addr_to_offset(mem_start)?;
        let len = usize::try_from(seg.file_size).map_err(|_| VmError::BadImage("file_size overflow".into()))?;
        if off + len > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds { addr: mem_start, len: seg.file_size });
        }
        self.memory[off..off + len].copy_from_slice(&seg.data);
        Ok(())
    }

    fn ensure_range(&self, addr: u64, len: u64) -> Result<(), VmError> {
        let base = self.cfg.memory_base;
        let limit = base.checked_add(self.cfg.mem_size).unwrap_or(u64::MAX);
        if addr < base || addr.checked_add(len).unwrap_or(u64::MAX) > limit {
            return Err(VmError::MemoryOutOfBounds { addr, len });
        }
        Ok(())
    }

    fn addr_to_offset(&self, addr: u64,) -> Result<usize, VmError> {
        let off = addr.checked_sub(self.cfg.memory_base).ok_or(VmError::MemoryOutOfBounds { addr, len: 1 })?;
        usize::try_from(off).map_err(|_| VmError::MemoryOutOfBounds { addr, len: 1 })
    }

    fn read_u32(&self, addr: u64) -> Result<u32, VmError> {
        self.ensure_range(addr, 4)?;
        if addr % 4 != 0 {
            return Err(VmError::UnalignedPc);
        }
        let off = self.addr_to_offset(addr)?;
        let b = &self.memory[off..off + 4];
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn write_u32(&mut self, addr: u64, val: u32) -> Result<(), VmError> {
        self.ensure_range(addr, 4)?;
        if addr % 4 != 0 {
            return Err(VmError::UnalignedAccess);
        }
        let off = self.addr_to_offset(addr)?;
        let b = val.to_le_bytes();
        self.memory[off..off + 4].copy_from_slice(&b);
        Ok(())
    }

    fn read_reg(&self, idx: u8) -> u64 {
        self.regs[idx as usize & 31]
    }

    fn write_reg(&mut self, idx: u8), val: u64) {
        let i = idx as usize & 31;
        if i != 0 {
            self.regs[i] = val;
        }
    }
}
