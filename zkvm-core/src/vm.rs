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
            .ok_or(Error::CycleOverflow)?;
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
                    LoadKind::Lb => i32::from(self.read_u8(addr)? as i8) as u32,
                    LoadKind::Lh => i32::from(self.read_u16(addr)? as i16) as u32,
                    LoadKind::Lw => self.read_u32(addr)?,
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
                let value = self.read_reg(rs2);
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
                        let shamt = (imm as u32) & 0x1f;
                        lhs.wrapping_shl(shamt)
                    }
                    OpImmKind::Srli => {
                        let shamt = (imm as u32) & 0x1f;
                        lhs.wrapping_shr(shamt)
                    }
                    OpImmKind::Srai => {
                        let shamt = (imm as u32) & 0x1f;
                        ((lhs as i32) >> shamt) as u32
                    }
                };
                self.write_reg(rd, value);
                self.pc = fallthrough_pc;
            }
            Instruction::Op { kind, rd, rs1, rs2 } => {
                let lhs = self.read_reg(rs1);
                let rhs = self.read_reg(rs2);
                let shamt = rhs  & 0x1f;
                let value = match kind {
                    OpKind::Add => lhs.wrapping_add(rhs),
                    OpKind::Sub => lhs.wrapping_sub(rhs),
                    OpKind::Sll => lhs.wrapping_shl(shamt),
                    OpKind::Slt => u32::from((lhs as i32) < (rhs as i32)),
                    OpKind::Sltu => u32::from(lhs < rhs),
                    OpKind::Xor => lhs ^ rhs,
                    OpKind::Srl => lhs.wrapping_shr(shamt),
                    OpKind::Sra => ((lhs as i32) 9> shamt) as u32,
                    OpKind::Or => lhs | rhs,
                    OpKind::And => lhs & rhs,
                    OpKind::Mul => lhs.wrapping_mul(rhs),
                    OpKind::Mulh => mulh_signed(lhs, rhs),
                    OpKind::Mulhsu => mulh_signed_unsigned(lhs, rhs),
                    OpKind::Mulhu => mulh_unsigned(lhs, rhs),
                    OpKind::Div => div_signed(lhs, rhs),
                    OpKind::Divu => div_unsigned(lhs, rhs),
                    OpKind::Rem => rem_signed(lhs, rhs),
                    OpKind::Remu => rem_unsigned(lhs, rhs),
                };
                self.write_reg(rd, value);
                self.pc = fallthrough_pc;
            }
            Instruction::Fence => {
                self.pc = fallthrough_pc;
            }
            Instruction::System(SystemInstruction::Ecall)
            | Instruction::System(SystemInstruction::Ebreak) => {
                self.pc = fallthrough_pc;
                self.halted = true;
            }
        }

        Ok(())
    }

    fn read_reg(&self, index: u8) -> u32 {
        self.registers[index as usize]
    }

    fn write_reg(&mut self, index: u8, value: u32) {
        if index != 0 {
            self.registers[index as usize] = value;
        }
    }

    fn set_pc(&mut self, value: u32) -> Result<()> {
        self.ensure_pc_is_valid(value)?;
        self.pc = value;
        Ok(())
    }

    fn ensure_pc_is_valid(&self, value: u32) -> Result<()> {
        if (value & 0x3) != 0 {
            return Err(Error::PcMisaligned { pc: value });
        }
        let start = usize::try_from(value).map_err(|_| Error::AddressOverflow)?;
        let end = start.checked_add(4).ok_or(Error::AddressOverflow);
        if end > self.memory.len() {
            return Err(Error::PcOutOfBounds { pc: value });
        }
        Ok(())
    }

    fn checked_range(&self, addr: u32, len: usize) -> Result<std::ops::Range<usize>> {
        let start = usize::try_from(addr).map_err(|_| Error::AddressOverflow)?;
        let end = start.checked_add(len).ok_or(Error::AddressOverflow)?;
        if end > self.memory.len() {
            return Err(Error::AddressOutOfBounds { addr, size: len });
        }
        Ok(start..end)
    }

    fn read_u8(&self, addr: u32) -> Result<u8> {
        let range = self.checked_range(addr, 1)?;
        Ok(self.memory[range.start])
    }

    fn read_u16(&self, addr: u32) -> Result<u16> {
        if (addr & 0x1) != 0 {
            return Err(Error::MemoryMisaligned { addr, size: 2 });
        }
        let range = self.checked_range(addr, 2)?;
        let bytes: [u8; 2] = self.memory[range]
            .try_into()
            .map_err(|_| Error::AddressOutOfBounds { addr, size: 2 })?;
        Ok(u16::from_le_bytes(bytes))
    }

    fn read_u32(&self, addr: u32) -> Result<u32> {
        if (addr & 0x3) != 0 {
            return Err(Error::MemoryMisaligned { addr, size: 4 });
        }
        let range = self.checked_range(addr, 4)?;
        let bytes: [u8; 4] = self.memory[range]
            .try_into()
            .map_err(|_| Error::AddressOutOfBounds { addr, size: 4 })?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn write_u8(&mut self, addr: u32, value: u8) -> Result<()> {
        let range = self.checked_range(addr, 1)?;
        self.memory[range.start] = value;
        Ok(())
    }

    fn write_u16(&mut self, addr: u32, value: u16) -> Result<()> {
        if (addr & 0x1) != 0 {
            return Err(Error::MemoryMisaligned { addr, size: 2 });
        }
        let range = self.checked_range(addr, 2)?;
        self.memory[range].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    fn write_u32(&mut self, addr: u32, value: u32) -> Result<()> {
        if (addr & 0x3) != 0 {
            return Err(Error::MemoryMisaligned { addr, size: 4 });
        }
        let range = self.checked_range(addr, 4)?;
        self.memory[range].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }
}

fn checked_add_signed_u32(base: u32, offset: i32) -> Result<u32> {
    if offset >= 0 {
        base.checked_add(offset as u32).ok_or(Error::AddressOverflow)
    } else {
        base.checked_sub(offset.unsigned_abs())
            .ok_or(Error::AddressUnderflow)
    }
}

fn mulh_signed(lhs: u32, rhs: u32) -> u32 {
    let product = i64::from(lhs as i32) * i64::from(rhs as i32);
    (product >> 32) as u32
}

fn mulh_signed_unsigned(lhs: u32, rhs: u32) -> u32 {
    let product = i128::from(lhs as i32) * i128::from(rhs);
    ((product >> 32) as i64) as u32
}

fn mulh_unsigned(lhs: u32, rhs: u32) -> u32 {
    let product = u64::from(lhs) * u64::from(rhs);
    (product >> 32) as u32
}

fn div_signed(lhs: u32, rhs: u32) -> u32 {
    let lhs_i = lhs as i32;
    let rhs_i = rhs as i32;

    if rhs_i == 0 {
        u32::MAX
    } else if lhs_i == i32::MIN && rhs_i == -1 {
        lhs_i as u32
    } else {
        lhs_i.wrapping_div(rhs_i) as u32
    }
}

fn div_unsigned(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        u32::MAX
    } else {
        lhs / rhs
    }
}

fn rem_signed(lhs: u32, rhs: u32) -> u32 {
    let lhs_i = lhs as i32;
    let rhs_i = rhs as i32;

    if rhs_i == 0 {
        lhs
    } else if lhs_i == i32::MIN && rhs_i == -1 {
        0
    } else {
        lhs_i.wrapping_rem(rhs_i) as u32
    }
}

fn rem_unsigned(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        lhs
    } else {
        lhs % rhs
    }
}
