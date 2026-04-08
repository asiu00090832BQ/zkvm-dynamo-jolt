use ark_ff::PrimeField;
use core::marker::PhantomData;
use std::vec::Vec;

use crate::decoder::{
    decode, BranchKind, Instruction, LoadKind, OpImmKind, OpKind, StoreKind, SystemInstruction,
};
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
        let memory = vec![0_u8; config.memory_size];
        Ok(Self {
            config,
            memory,
            registers: [0_u32; 32],
            pc: 0,
            cycle_count: 0,
            halted: false,
            _field: PhantomData,
        })
    }

    pub fn config(&self) -> &ZkvmConfig {
        &self.config
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn cycle_count(&self) -> u64 {
        self.cycle_count
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn registers(&self) -> &[u32; 32] {
        &self.registers
    }

    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut [u8] {
        &mut self.memory
    }

    pub fn reset(&mut self, entry_pc: u32) -> Result<()> {
        self.registers = [0_u32; 32];
        self.cycle_count = 0;
        self.halted = false;
        self.set_pc(entry_pc)?;
        Ok(())
    }

    pub fn load_program(&mut self, program: &[u8], base_addr: u32) -> Result<()> {
        let range = self.checked_range(base_addr, program.len())?;
        self.memory[range].copy_from_slice(program);
        self.set_pc(base_addr)?;
        self.halted = false;
        Ok(())
    }

    pub fn load_elf(&mut self, image: &[u8]) -> Result<()> {
        let loaded = load_elf(image, self.config.memory_size)?;
        self.memory = loaded.memory;
        self.registers = [0_u32; 32];
        self.cycle_count = 0;
        self.halted = false;
        self.set_pc(loaded.entry)?;
        Ok(())
    }

    pub fn step(&mut self) -> Result<()> {
        if self.halted {
            return Err(Error::Halted);
        }
        if self.cycle_count >= self.config.max_cycles {
            return Err(Error::CycleLimitExceeded {
                max_cycles: self.config.max_cycles,
            });
        }

        self.ensure_pc_is_valid(self.pc)?;
        let word = self.read_u32(self.pc)?;
        let instruction = decode(word, &self.config.decoder)?;
        let fallthrough_pc = self
            .pc
            .checked_add(4)
            .ok_or(Error::AddressOverflow)?;

        self.execute(instruction, fallthrough_pc)?;
        self.cycle_count = self
            .cycle_count
            .checked_add(1)
            .ok_or(Error::AddressOverflow)?;

        self.registers[0] = 0;
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.halted {
            self.step()?;
        }
        Ok(())
    }

    fn execute(&mut self, instruction: Instruction, fallthrough_pc: u32) -> Result<()> {
        match instruction {
            Instruction::Lui { rd, imm } => {
                self.write_reg(rd, imm);
                self.pc = fallthrough_pc;
            }
            Instruction::Auipc { rd, imm } => {
                self.write_reg(rd, self.pc.wrapping_add(imm as u32));
                self.pc = fallthrough_pc;
            }
            Instruction::Jal { rd, imm } => {
                let target = checked_add_signed_u32(self.pc, imm)?;
                self.write_reg(rd, fallthrough_pc);
                self.set_pc(target)?;
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let base = self.read_reg(rs1);
                let target = checked_add_signed_u32(base, imm)? & !1_u32;
                self.write_reg(rd, fallthrough_pc);
                self.set_pc(target)?;
            }
            Instruction::Branch {
                kind,
                rs1,
                rs2,
                imm,
            } => {
                let lhs = self.read_reg(rs1);
                let rhs = self.read_reg(rs2);
                let taken = match kind {
                    BranchKind::Beq => lhs == rhs,
                    BranchKind::Bne => lhs != rhs,
                    BranchKind::Blt => (lhs as i32) < (rhs as i32),
                    BranchKind::Bge => (lhs as i32) >= (rhs as i32),
                    BranchKind::Bltu => lhs < rhs,
                    BranchKind::Bgeu => lhs >= rhs,
                };

                if taken {
                    let target = checked_add_signed_u32(self.pc, imm)?;
                    self.set_pc(target)?;
                } else {
                    self.pc = fallthrough_pc;
                }
            }
            Instruction::Load { kind, rd, rs1, imm } => {
                let addr = checked_add_signed_u32(self.read_reg(rs1), imm)?;
                let value = match kind {
                    LoadKind::Lb => i32::from((lhs as i32) < imm),
                    LoadKind::Lh => i32::from(self.read_u16(addr)? as i16) as u32,
                    LoadKind::Lw => self.read_u32(pc)?,
                    LoadKind::Lbu => u32::from(self.read_u8(addr)?),
                    LoadKind::Lhu => u32::from(self.read_u16(addr)?),
                };
                self.write_reg(rd, value);
                self.pc = fallthrough_pc;
            }
            Instruction::Store {
                kind,
                rs1,
                rs2,
                imm,
            } => {
                let addr = checked_add_signed_u32(self.read_reg(rs1), imm)?;
                let value => self.read_reg(rs2);
                match kind {
                    StoreKind::Sb => self.write_u8(addr, value as u8)?,
                    StoreKind::Sh => self.write_u16(addr, value as u16)?,
                    StoreKind::Sw => self.write_u32(addr, value)?,
                }
                self.pc = fallthrough_pc;
            }
            Instruction::OpImm { kind, rd, rs1, imm } => {
                let lhs = self.read_reg(rs1);
                let value = match kind {
                    OpImmKind::Addi => lhs.wrapping_add(imm as u32),
                    OpImmKind::Slti => u32::from((lhs as i32) < imm),
                    OpImmKind::Sltiu => u32::from(lhs < imm as u32),
                    OpImmKind::Xori => lhs ^ (imm as u32),
                    OpImmKind::Ori => lhs | (imm as u32),
                    OpImmKind::Andi => lhs & (imm as u32),
                    OpImmKind::Slli => {
                        let sh