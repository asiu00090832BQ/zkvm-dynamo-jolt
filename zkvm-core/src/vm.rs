use ark_ff::PrimeField;
use core::marker::PhantomData;
use std::vec::Vec;
use rv32im_decoder::{decode, Instruction};
use crate::elf_loader::load_elf;
use crate::{Error, Result, ZkvmConfig};

#[derive(Debug, Clone)]
pub struct Zkvm<F: PrimeField> {
    config: ZkvmConfig,
    memory: Vec<u8>,
    registers: [u32; 32],
    pc: u32,
    cycle_count: u64,
    halted: bool,
    _field: PhantomData<F>,
}

impl<F: PrimeField> Zkvm<F> {
    pub fn new(config: ZkvmConfig) -> Result<Self> {
        let memory = vec![0u8; config.memory_size];
        Ok(Self {
            config,
            memory,
            registers: [0u32; 32],
            pc: 0,
            cycle_count: 0,
            halted: false,
            _field: PhantomData,
        })
    }

    pub fn load_elf(&mut self, image: &[u8]) -> Result<()> {
        let loaded = load_elf(image, self.config.memory_size)?;
        self.memory = loaded.memory;
        self.pc = loaded.entry;
        self.registers = [0u32; 32];
        self.cycle_count = 0;
        self.halted = false;
        Ok(())
    }

    pub fn step(&mut self) -> Result<()> {
        if self.halted { return Err(Error::Halted); }
        let word = self.read_u32(self.pc)?;
        let instruction = decode(word)?;
        self.execute(instruction)?;
        self.cycle_count += 1;
        self.registers[0] = 0;
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.halted { self.step()?; }
        Ok(())
    }

    fn execute(&mut self, instruction: Instruction) -> Result<()> {
        match instruction {
            Instruction::Lui { rd, imm } => { self.write_reg(rd, imm); self.pc += 4; }
            Instruction::Auipc { rd, imm } => { self.write_reg(rd, self.pc.wrapping_add(imm as u32)); self.pc += 4; }
            _ => { self.pc += 4; }
        }
        Ok(())
    }

    fn read_u32(&self, addr: u32) -> Result<u32> {
        let idx = addr as usize;
        Ok(u32::from_le_bytes(self.memory[idx..idx+4].try_into().unwrap()))
    }

    fn write_reg(&mut self, index: u8, val: u32) {
        if index != 0 { self.registers[index as usize] = val; }
    }
    pub fn pc(&self) -> u32 { self.pc }
    pub fn cycle_count(&self) -> u64 { self.cycle_count }
}
