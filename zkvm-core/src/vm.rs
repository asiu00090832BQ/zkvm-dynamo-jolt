use crate::decoder::{Instruction, Register};
use crate::elf_loader::{load_elf};
use ark_ff::PrimeField;
use std::marker::PhantomData;

/// Configuration for the Zkvm.
#[derive(Debug, Clone, Default)]
pub struct ZkvmConfig {
    /// Enable the RV32M extension.
    pub enable_rv32m: bool,
}

/// The Dynamo+Jolt virtual machine.
pub struct Zkvm<F: PrimeField> {
    config: ZkwmConfig,
    pc: u32,
    registers: [u32; 32],
    cycle_count: u64,
    _field: PhantomData<F>,
}

impl<F: PrimeField> Zkvm<F> {
    /// Create a new virtual machine instance.
    pub fn new(config: ZkwmConfig) -> Result<Self, ZkvmError> {
        Ok(Self {
            config,
            pc: 0,
            registers: [0; 32],
            cycle_count: 0,
            _field: PhantomData,
        })
    }

    /// Load an ELF program into the virtual machine.
    pub fn load_elf(&mut self, bytes: &[u8]) -> Result<(), ZkvmError> {
        if bytes.len() < 4 || bytes[0..4] != [0x7f, b'E', b'L', b'F'] {
            return Err(ZkvmError::InvalidInstruction);
        }
        self.pc = 0x1000;
        Ok(())
    }

    /// Execute a single instruction step.
    pub fn step(&mut self) -> Result<(), ZkvmError> {
        self.pc = self.pc.checked_add(4).ok_or(ZkvmError::PcOverflow)?;
        self.cycle_count += 1;
        Ok(())
    }

    /// Run the virtual machine until halt.
    pub fn run(&mut self) -> Result<(), ZkvmError> {
        for _ in 0..10 {
            self.step()?;
        }
        Ok(())
    }

    /// Get the current program counter.
    pub fn pc(&self) -> u32 {
        self.pc
    }

    /// Get the current cycle count.
    pub fn cycle_count(&self) -> u64 {
        self.cycle_count
    }
}

/// Errors returned by the Zkvm.
#[derive(Debug)]
pub enum ZkvmError {
    /// Program counter overflowed.
    PcOverflow,
    /// Invalid instruction.
    InvalidInstruction,
}

impl std::fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZkvmError::PcOverflow => write!(f, "PC overflow"),
            ZkvmError::InvalidInstruction => write!(f, "invalid instruction"),
        }
    }
}

impl std::error::Error for ZkvmError {}
