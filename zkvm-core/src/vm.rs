use ark_ff::PrimeField;
use core::marker::PhantomData;
use std::vec::Vec;
use crate::decoder::{decode, Instruction, BranchKind, LoadKind, StoreKind, OpImmKind, OpKind, SystemInstruction, DecoderConfig};
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
        Ok(Self { config, memory, registers: [0u32; 32], pc: 0, cycle_count: 0, halted: false, _field: PhantomData })
    }

    pub fn load_elf(&mut self, image: &[u8]) -> Result<()> {
        let loaded = load_elf(image, self.config.memory_size).map_err(Error::ElfLoader)?;
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
        let insn = decode(word, &self.config.decoder).map_err(Error::Decoder)?;
        let next_pc = self.pc.wrapping_add(4);
        match insn {
            Instruction::Lui { rd, imm } => self.write_reg(rd, imm),
            Instruction::Auipc { rd, imm } => self.write_reg(rd, self.pc.wrapping_add(imm as u32)),
             Instruction::Jal { rd, imm } => {
                self.write_reg(rd, next_pc);
                self.pc = self.pc.wrapping_add(imm as u32);
                return Ok(());
            }
            Instruction::Op { .. } => { /* logic */ }
            _ => {}
        }
        self.pc = next_pc;
        self.cycle_count += 1;
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.halted && self.cycle_count < self.config.max_cycles { self.step()?; }
        Ok(())
    }

    fn read_u32(&self, addr, u32) -> Result<u32> {
        let idx = addr as usize;
        if idx + 4 > self.memory.len() { return Err(Error::AddressOutOfBounds { addr, size: 4 }); }
        Ok(u32::from_le_bytes(self.memory[idx..idx+4].try_into().unwrap()))
    }

    fn write_reg(&mut self, rd: u8, val: u32) {
        if rd != 0 { self.registers[rd as usize] = val; }
    }
    pub fn cycle_count(&self) -> u64 { self.cycle_count }
}