//! zkVM execution engine: a minimal RISC‑V fetch/decode/execute loop.

use crate::decoder::{self, DecodeError, Instruction, SystemInstruction};
use crate::elf_loader::{self, ElfError};

/// Result type alias for VM operations.
pub type VmResult<T> = Result<T, VmError>;

/// Errors that can occur during VM creation or execution.
#[derive(Debug)]
pub enum VmError {
    /// Error produced by the ELF loader.
    Elf(ElfError),
    /// Failure decoding an instruction.
    Decode(DecodeError),
    /// Program counter points outside the guest memory.
    InvalidPc(u64),
    /// Guest attempted an out‑of‑bounds memory access.
    MemoryOutOfBounds { addr: u64, len: usize },
    /// VM halted due to an ECALL or EBREAK instruction.
    Halted,
}

impl From<ElfError> for VmError {
    fn from(err: ElfError) -> Self {
        VmError::Elf(err)
    }
}

impl From<DecodeError> for VmError {
    fn from(err: DecodeError) -> Self {
        VmError::Decode(err)
    }
}

/// A very small RISC‑V virtual machine.
///
/// This VM is intentionally minimal: it only decodes system instructions
/// (ECALL/EBREAK) and otherwise treats every instruction as a no‑op.
#[derive(Debug)]
pub struct Vm {
    /// Integer registers x0..x31. x0 is always read as zero.
    regs: [u64; 32],
    /// Program counter (PC).
    pc: u64,
    /// Contiguous memory backing the guest address space.
    memory: Vec<u8>,
    /// Virtual address that corresponds to the start of `memory`.
    load_base: u64,
}

impl Vm {
    /// Construct a VM instance from an ELF image.
    ///
    /// This uses the [`elf_loader`] module to parse and map the ELF file
    /// into a linear memory buffer, and sets the PC to the ELF entry point.
    pub fn from_elf(image: &[u8]) -> VmResult<Self> {
        let program = elf_loader::load_elf(image)?;

        Ok(Self {
            regs: [0; 32],
            pc: program.entry,
            memory: program.memory,
            load_base: program.load_base,
        })
    }

    /// Accessor for the program counter.
    pub fn pc(&self) -> u64 {
        self.pc
    }

    /// Accessor for the general-purpose registers.
    pub fn registers(&self) -> &[u64; 32] {
        &self.regs
    }

    /// Translate a guest virtual address into an index into `self.memory`.
    ///
    /// Ensures that the requested range `[addr, addr + len)` lies entirely
    /// within the mapped memory, returning a host index on success.
    fn translate_address(&self, addr: u64, len: usize) -> VmResult<usize> {
        if addr < self.load_base {
            return Err(VmError::InvalidPc(addr));
        }

        let start = (addr - self.load_base) as usize;
        let end = start
            .checked_add(len)
            .ok_or(VmError::MemoryOutOfBounds { addr, len })?;

        if end > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds { addr, len });
        }

        Ok(start)
    }

    /// Fetch a 32‑bit little‑endian instruction word from memory.
    fn fetch_instruction(&self) -> VmResult<[u8; 4]> {
        let pc = self.pc;
        let offset = self.translate_address(pc, 4)?;
        let bytes = &self.memory[offset..offset + 4];

        Ok([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    /// Execute a single instruction and advance the PC.
    ///
    /// On a normal instruction this returns `Ok(())`.
    /// If an ECALL or EBREAK is encountered, this returns `Err(VmError::Halted)`.
    pub fn step(&mut self) -> VmResult<()> {
        let raw = self.fetch_instruction()?;
        let insn = decoder::decode(&raw)?;

        // For this minimal VM we assume fixed 32‑bit instructions.
        self.pc = self.pc.wrapping_add(4);

        self.execute(insn)
    }

    /// Execute a decoded instruction.
    fn execute(&mut self, insn: Instruction) -> VmResult<()> {
        match insn {
            Instruction::System(sys) => {
                // In this VM, ECALL and EBREAK are treated as halting conditions.
                if matches!(sys, SystemInstruction::Ecall | SystemInstruction::Ebreak) {
                    return Err(VmError::Halted);
                }
                self.execute_system(sys)
            }
            // All other instructions are treated as no‑ops.
            _ => Ok(()),
        }
    }

    /// Execute a system instruction.
    ///
    /// The caller is responsible for deciding how ECALL/EBREAK affect control
    /// flow. In this implementation, [`execute`] treats them as halting
    /// conditions and does not call this function for those variants.
    fn execute_system(&mut self, sys: SystemInstruction) -> VmResult<()> {
        match sys {
            SystemInstruction::Ecall => Ok(()),
            SystemInstruction::Ebreak => Ok(()),
        }
    }

    /// Run the VM for at most `max_steps: usize) -> VmResult<()> {
        for _ in 0..max_steps {
            match self.step() {
                Ok(()) => {}
                Err(VmError::Halted) => return Ok(()),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Load a 64‑bit little‑endian word from guest memory.
    ///
    /// This is a convenience helper for embedding hosts; instruction
    /// execution logic for regular loads/stores is not implemented here.
    pub fn load_u64(&self, addr: u64) -> VmResult<u64> {
        let offset = self.translate_address(addr, 8)?;
        let bytes = &self.memory[offset..offset + 8];
        Ok(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    /// Store a 64‑bit little‑endian word into guest memory.
    pub fn store_u64(&mut self, addr: u64, value: u64) -> VmResult<()> {
        let offset = self.translate_address(addr, 8)?;
        let bytes = value.to_le_bytes();
        self.memory[offset..offset + 8].copy_from_slice(&bytes);
        Ok(())
    }
}
