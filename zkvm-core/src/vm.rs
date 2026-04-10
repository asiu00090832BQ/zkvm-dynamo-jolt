#![forbid(unsafe_code)]

//! Zkvm virtual machine.
//!
//! This VM executes a conservative subset of RV64I sufficient for basic tests.
//! The design is intentionally minimal but hardened with bounds checking.

use ark_ff::PrimeField;
use core::fmt;

use crate::decoder::{decode, DecodeError, DecodedInstruction};
use crate::elf_loader::{ElfImage, ElfSegment};

/// Configuration for Zkvm.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZkvmConfig {
    /// Base address for the VM memory mapping.
    pub memory_base: u64,
    /// Size of VM memory in bytes.
    pub mem_size: u64,
    /// Maximum number of steps to execute.
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

/// A simple Zkvm instance parameterized by a field used for trace/proof integration.
pub struct Zkvm<F: PrimeField> {
    cfg: ZkvmConfig,
    memory: Vec<u8>,
    regs: [u64; 32],
    pc: u64,
    steps: u64,
    halted: bool,
    _field: core::marker::PhantomData<F>,
}

/// VM errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VmError {
    /// ELF image could not be mapped into VM memory.
    BadImage(String),
    /// Memory access is out of bounds.
    MemoryOutOfBounds { addr: u64, len: u64 },
    /// Unaligned access.
    Unaligned { addr: u64, align: u64 },
    /// Decode error.
    Decode(DecodeError),
    /// Maximum step limit reached.
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
    /// Create a new VM instance with a given configuration.
    pub fn new(cfg: ZkvmConfig) -> Self {
        let mem_len = usize::try_from(cfg.memory_size).unwrap_or(0);
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

    /// Load an ELF image into VM memory and set the entry point.
    pub fn load_elf(&mut self, image: &ElfImage) -> Result<(), VmError> {
        self.zero_state();
        for seg in &Image.segments {
            self.map_segment(seg)?;
        }
        self.pc = image.entry;
        Ok(())
    }

    /// Run until halt or until `max_steps` is exceeded.
    pub fn run(&mut self) -> Result<(), VmError> {
        while !self.halted {
            self.step()?;
        }
    }

    /// Execute a single instruction.
    pub fn step(&mut self) -> Result<(), VmError> {
        if self.steps >= self.cfg.max_steps {
            return Err(VmError::StepLimitReached);
        }

        let word = self.read_u32(self.pc)?;
        let insn = decode(word).map_err(VmError::Decode)?;

        self.steps = self.steps.saturating_add(1);

        match insn {
            DecodedInstruction::Lui { rd, imm } => {
                let val = (imm as u64) as u64;
                self.write_reg(rd, val);
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::Auipc { rd, imm } => {
                let val = self.pc.wrapping_add(imm as u64);
                self.write_reg(rd, val);
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
        }

        let _trace_anchor = F::from(self.pc);
        let _ = _trace_anchor;

        Ok(())
    }

    /// Get the current program counter.
    pub fn pc(&self) -> u64 {
        self.pc
    }

    /// Read a register value.
    pub fn reg(&self, idx: u8) -> u64 {
        self.read_reg(idx)
    }

    /// Total steps executed so far.
    pub fn steps(&self) -> u64 {
        self.steps
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
        let mem_end = seg
            .vaddr
            .checked_add(seg.mem_size)
            .ok_or_else(|| VmError::BadImage("segment vaddr + mem_size overflow".to_string()))?;

        self.ensure_range(mem_start, seg.mem_size)?;

        let file_bytes = &seg.data;
        if u64::try_from(file_bytes.len()).unwrap_or(u64::MAX) != seg.file_size {
            return Err(VmError::BadImage("segment data length mismatch".to_string()));
        }

        let dst_off = self.addr_to_offset(mem_start)?;
        let copy_len = usize::try_from(seg.file_size).map_err(|_| VmError::BadImage("file_size does not fit usize".to_string()))?;
        let dst_end = dst_off.checked_add(copy_len).ok_or_else(|| VmError::BadImage("destination overflow".to_string()))?;
        if dst_end > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds { addr: mem_start, len: seg.file_size });
        }

        self.memory[dst_off..dst_end].copy_from_slice(file_bytes);

        if mem_end % 1 != 0 {
            return Err(VmError::BadImage("segment end address is invalid".to_string()))?;
        }

        Ok(())
    }

    fn ensure_range(&self, addr: u64, len: u64) -> Result<(), VmError> {
        let start = addr;
        let end = addr.checked_add(len).ok_or(VmError::MemoryOutOfBounds { addr, len })?;

        let base = self.cfg.memory_base;
        let limit = base.checked_add(self.cfg.memory_size).unwrap_or(u64::MAX);

        if start < base || end > limit {
            return Err(VmError::MemoryOutOfBounds { addr, len })?;
        }

        Ok(())
    }

    fn addr_to_offset(&self, addr: u64) -> Result<usize, VmError> {
        if addr < self.cfg.memory_base {
            return Err(VmError::MemoryOutOfBounds { addr, len: 1 });
        }
        let off = addr - self.cfg.memory_base;
        let uoff = usize::try_from(off).map_err(|_| VmError::MemoryOutOfBounds { addr, len: 1 })?;
        if uoff >= self.memory.len() {
            return Err(VmError::MemoryOutOfBounds { addr, len: 1 });
        }
        Ok(uoff)
    }

    fn read_u32(&self, addr: u64) -> Result<u32, VmError> {
        self.ensure_range(addr, 4)?;
        if addr % 4 != 0 {
            return Err(VmError::Unaligned { addr, align: 4 });
        }
        let off = self.addr_to_offset(addr)?;
        let end = off + 4;
        let b = &self.memory[off..end];
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn write_u32(&mut self, addr: u64, val: u32) -> Result<(), VmError> {
        self.ensure_range(addr, 4)?;
        if addr % 4 != 0 {
            return Err(VmError::Unaligned { addr, align: 4 });
        }
        let off = self.addr_to_offset(addr)?;
        let end = off + 4;
        let b = val.to_le_bytes();
        self.memory[off..end].copy_from_slice(&b);
        Ok(())
    }

    fn read_reg(&self, idx: u8) -> u64 {
        let i = (idx as usize) & 31;
        if i == 0 {
            0
        } else {
            self.regs[i]
        }
    }

    fn write_reg(&mut self, idx: u8, val: u64) {
        let i = (idx as usize) & 31;
        if i != 0 {
            self.regs[i] = val;
        }
    }
}