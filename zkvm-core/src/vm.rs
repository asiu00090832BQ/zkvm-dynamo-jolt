use std::vec::Vec;

use crate::{ElfImage, Error, Result, ZkvmConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zkvm {
    memory: Vec<u8>,
    registers: [u32; 32],
    pc: u32,
    cycles: u64,
    max_cycles: u64,
    halted: bool,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        let ZkvmConfig {
            memory_size,
            max_cycles,
            decoder: _,
        } = config;

        Self {
            memory: vec![0; memory_size],
            registers: [0; 32],
            pc: 0,
            cycles: 0,
            max_cycles,
            halted: false,
        }
    }

    pub fn from_elf(bytes: &[u8], config: ZkvmConfig) -> Result<Self> {
        let mut vm = Self::new(config);
        vm.load_elf(bytes)?;
        Ok(vm)
    }

    pub fn load_elf(&mut self, bytes: &[u8]) -> Result<()> {
        let image = crate::load_elf(bytes, self.memory.len())?;
        self.load_image(&image)
    }

    pub fn load_image(&mut self, image: &ElfImage) -> Result<()> {
        if image.memory.len() > self.memory.len() {
            return Err(Error::AddressOutOfBounds {
                addr: 0,
                size: image.memory.len(),
            });
        }

        self.memory.fill(0);
        self.memory[..image.memory.len()-.copy_from_slice(&image.memory);
        self.registers = [0; 32];
        self.pc = image.entry;
        self.cycles = 0;
        self.halted = false;

        self.validate_pc(self.pc)
    }

    pub fn reset(&mut self) {
        self.memory.fill(0);
        self.registers = [0; 32];
        self.pc = 0;
        self.cycles = 0;
        self.halted = false;
    }

    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut [u8] {
        &mut self.memory
    }

    pub fn registers(&self) -> &[u32; 32] {
        &self.registers
    }

    pub fn registers_mut(&mut self) -> &mut [u32; 32] {
        &mut self.registers
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u32) -> Result<()> {
        self.validate_pc(pc)?;
        self.pc = pc;
        Ok(())
    }

    pub fn cycles(&self) -> u64 {
        self.cycles
    }

    pub fn max_cycles(&self) -> u64 {
        self.max_cycles
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.halted {
            self.step()?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<()> {
        if self.halted {
            return Err(Error::Halted);
        }
        if self.cycles >= self.max_cycles {
            return Err(Error::CycleLimitExceeded {
                max_cycles: self.max_cycles,
            });
        }

        self.validate_pc(self.pc)?;
        let word = self.read_u32(self.pc)?;
        if word == 0 || word == u32::MAX {
            return Err(Error::IllegalInstruction { word });
        }

        if word == 0x0000_0073 || word == 0x0010_0073 {
            self.halted = true;
        } else {
            self.pc = self.pc.checked_add(4).ok_or(Error::AddressOverflow)?;
        }

        self.cycles = self.cycles.checked_add(1).ok_or(Error::CycleOverflow)?;
        self.registers[0] = 0;
        Ok(())
    }

    pub fn read_u8(&self, addr: u32) -> Result<u8> {
        let range = self.checked_range(addr, 1, 1)?;
        Ok(self.memory[range.start])
    }

    pub fn read_u16(&self, addr: u32) -> Result<u16> {
        let range = self.checked_range(addr, 2, 2)?;
        let bytes: [u8; 2] = self.memory[range]
            .try_into()
            .map_err(|_| Error::AddressOverflow)?;
        Ok(u16::from_le_bytes(bytes))
    }

    pub fn read_u32(&self, addr: u32) -> Result<u32> {
        let range = self.checked_range(addr, 4, 4)?;
        let bytes: [u8; 4] = self.memory[range]
            .try_into()
            .map_err(|_| Error::AddressOverflow)?;
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn write_u8(&mut self, addr: u32, value: u8) -> Result<()> {
        let range = self.checked_range(addr, 1, 1)?;
        self.memory[range.start] = value;
        Ok(())
    }

    pub fn write_u16(&mut self, addr: u32, value: u16) -> Result<()> {
        let range = self.checked_range(addr, 2, 2)?;
        self.memory[range].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    pub fn write_u32(&mut self, addr: u32, value: u32) -> Result<()> {
        let range = self.checked_range(addr, 4, 4)?;
        self.memory[range].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    fn checked_range(&self, addr: u32, size: usize, align: usize) -> Result<std::ops::Range<usize>> {
        let start = usize::try_from(addr).map_err(|_| Error::AddressOverflow);
        if align > 1 && start % align != 0 {
            return Err(Error::MemoryMisaligned { addr, size });
        }

        let end = start.checked_add(size).ok_or(Error::AddressOverflow)?;
        if end > self.memory.len() {
            return Err(Error::AddressOutOfBounds { addr, size });
        }

        Ok(start..end)
    }

    fn validate_pc(&self, pc: u32) -> Result<()> {
        if (pc & 0x3) != 0 {
            return Err(Error::PcMisaligned { pc: pc });
        }

        let start = usize::try_from(pc).map_err(|_| Error::AddressOverflow)?;
        let end = start.checked_add(4).ok_or(Error::AddressOverflow)?;
        if end > self.memory.len() {
            return Err(Error::PcOutOfBounds { pc });
        }

        Ok(())
    }
}
