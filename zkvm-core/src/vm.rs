extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::fmt;

use crate::decoder::{
    decode, decompose_i32_16, decompose_u32_16, mul_i32_u32_wide_16, mul_i32_wide_16,
    mul_u32_wide_16, recompose_i32_16, recompose_u32_16, Instruction, ZkvmError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub reset_pc: u32,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            memory_size: 65_536,
            reset_pc: 0,
        }
    }
}

pub struct Zkvm {
    pc: u32,
    registers: [u32; 32],
    memory: Vec<u8>,
    halted: bool,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            pc: config.reset_pc,
            registers: [0; 32],
            memory: vec![0; config.memory_size],
            halted: false,
        }
    }

    pub fn pc(&self) -> u32 {
        self.pc
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

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    pub fn reset(&mut self, config: ZkvmConfig) {
        self.pc = config.reset_pc;
        self.registers = [0; 32];
        self.memory.resize(config.memory_size, 0);
        self.memory.fill(0);
        self.halted = false;
    }

    pub fn load_program(&mut self, address: u32, program: &[u8]) -> Result<(), ZkvmError> {
        let start = self.checked_range(address, program.len())?;
        self.memory[start..start + program.len()].copy_from_slice(program);
        Ok(())
    }

    pub fn fetch(&self) -> Result<Instruction, ZkvmError> {
        let word = self.fetch_word()?;
        decode(word)
    }

    pub fn step(&mut self) -> Result<(), ZkvmError> {
        if self.halted {
            return Err(ZkvmError::Halted);
        }

        let current_pc = self.pc;
        let instruction = self.fetch()?;
        let mut next_pc = current_pc.wrapping_add(4);

        match instruction {
            Instruction::Lui { rd, imm } => {
                self.write_register(rd, imm as u32);
            }
            Instruction::Auipc { rd, imm } => {
                self.write_register(rd, current_pc.wrapping_add(imm as u32));
            }
            Instruction::Jal { rd, imm } => {
                self.write_register(rd, next_pc);
                next_pc = current_pc.wrapping_add(imm as u32);
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.read_register(rs1).wrapping_add(imm as u32) & !1;
                self.write_register(rd, next_pc);
                next_pc = target;
            }

            Instruction::Beq { rs1, rs2, imm } => {
                if self.read_register(rs1) == self.read_register(rs2) {
                    next_pc = current_pc.wrapping_add(imm as u32);
                }
            }
            Instruction::Bne { rs1, rs2, imm } => {
                if self.read_register(rs1) != self.read_register(rs2) {
                    next_pc = current_pc.wrapping_add(imm as u32);
                }
            }
            Instruction::Blt { rs1, rs2, imm } => {
                if (self.read_register(rs1) as i32) < (self.read_register(rs2) as i32) {
                    next_pc = current_pc.wrapping_add(imm as u32);
                }
            }
            Instruction::Bge { rs1, rs2, imm } => {
                if (self.read_register(rs1) as i32) >= (self.read_register(rs2) as i32) {
                    next_pc = current_pc.wrapping_add(imm as u32);
                }
            }
            Instruction::Bltu { rs1, rs2, imm } => {
                if self.read_register(rs1) < self.read_register(rs2) {
                    next_pc = current_pc.wrapping_add(imm as u32);
                }
            }
            Instruction::Bgeu { rs1, rs2, imm } => {
                if self.read_register(rs1) >= self.read_register(rs2) {
                    next_pc = current_pc.wrapping_add(imm as u32);
                }
            }

            Instruction::Lb { rd, rs1, imm } => {
                let addr = self.read_register(rs1).wrapping_add(imm as u32);
                let value = self.read_u8(addr)? as i8 as i32 as u32;
                self.write_register(rd, value);
            }
            Instruction::Lh { rd, rs1, imm } => {
                let addr = self.read_register(rs1).wrapping_add(imm as u32);
                let value = self.read_u16(addr)? as i16 as i32 as u32;
                self.write_register(rd, value);
            }
            Instruction::Lw { rd, rs1, imm } => {
                let addr = self.read_register(rs1).wrapping_add(imm as u32);
                let value = self.read_u32(addr)?;
                self.write_register(rd, value);
            }
            Instruction::Lbu { rd, rs1, imm } => {
                let addr = self.read_register(rs1).wrapping_add(imm as u32);
                let value = u32::from(self.read_u8(addr)?);
                self.write_register(rd, value);
            }
            Instruction::Lhu { rd, rs1, imm } => {
                let addr = self.read_register(rs1).wrapping_add(imm as u32);
                let value = u32::from(self.read_u16(addr)?);
                self.write_register(rd, value);
            }

            Instruction::Sb { rs1, rs2, imm } => {
                let addr = self.read_register(rs1).wrapping_add(imm as u32);
                self.write_u8(addr, self.read_register(rs2) as u8)?;
            }
            Instruction::Sh { rs1, rs2, imm } => {
                let addr = self.read_register(rs1).wrapping_add(imm as u32);
                self.write_u16(addr, self.read_register(rs2) as u16)?;
            }
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = self.read_register(rs1).wrapping_add(imm as u32);
                self.write_u32(addr, self.read_register(rs2))?;
            }

            Instruction::Addi { rd, rs1, imm } => {
                self.write_register(rd, self.read_register(rs1).wrapping_add(imm as u32));
            }
            Instruction::Slti { rd, rs1, imm } => {
                self.write_register(rd, ((self.read_register(rs1) as i32) < imm) as u32);
            }
            Instruction::Sltiu { rd, rs1, imm } => {
                self.write_register(rd, (self.read_register(rs1) < imm as u32) as u32);
            }
            Instruction::Xori { rd, rs1, imm } => {
                self.write_register(rd, self.read_register(rs1) ^ imm as u32);
            }
            Instruction::Ori { rd, rs1, imm } => {
                self.write_register(rd, self.read_register(rs1) | imm as u32);
            }
            Instruction::Andi { rd, rs1, imm } => {
                self.write_register(rd, self.read_register(rs1) & imm as u32);
            }
            Instruction::Slli { rd, rs1, shamt } => {
                self.write_register(rd, self.read_register(rs1) << u32::from(shamt));
            }
            Instruction::Srli { rd, rs1, shamt } => {
                self.write_register(rd, self.read_register(rs1) >> u32::from(shamt));
            }
            Instruction::Srai { rd, rs1, shamt } => {
                self.write_register(
                    rd,
                    ((self.read_register(rs1) as i32) >> u32::from(shamt)) as u32,
                );
            }

            Instruction::Add { rd, rs1, rs2 } => {
                self.write_register(
                    rd,
                    self.read_register(rs1).wrapping_add(self.read_register(rs2)),
                );
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                self.write_register(
                    rd,
                    self.read_register(rs1).wrapping_sub(self.read_register(rs2)),
                );
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                self.write_register(rd, self.read_register(rs1) << (self.read_register(rs2) & 0x1f));
            }
            Instruction::Slt { rd, rs1, rs2 } => {
                self.write_register(
                    rd,
                    ((self.read_register(rs1) as i32) < (self.read_register(rs2) as i32)) as u32,
                );
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                self.write_register(rd, (self.read_register(rs1) < self.read_register(rs2)) as u32);
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                self.write_register(rd, self.read_register(rs1) ^ self.read_register(rs2));
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                self.write_register(rd, self.read_register(rs1) >> (self.read_register(rs2) & 0x1f));
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                self.write_register(
                    rd,
                    ((self.read_register(rs1) as i32) >> (self.read_register(rs2) & 0x1f)) as u32,
                );
            }
            Instruction::Or { rd, rs1, rs2 } => {
                self.write_register(rd, self.read_register(rs1) | self.read_register(rs2));
            }
            Instruction::And { rd, rs1, rs2 } => {
                self.write_register(rd, self.read_register(rs1) & self.read_register(rs2));
            }

            Instruction::Mul { rd, rs1, rs2 } => {
                let lhs = recompose_i32_16(decompose_i32_16(self.read_register(rs1) as i32));
                let rhs = recompose_i32_16(decompose_i32_16(self.read_register(rs2) as i32));
                let product = mul_i32_wide_16(lhs, rhs);
                self.write_register(rd, product as u32);
            }
            Instruction::Mulh { rd, rs1, rs2 } => {
                let lhs = recompose_i32_16(decompose_i32_16(self.read_register(rs1) as i32));
                let rhs = recompose_i32_16(decompose_i32_16(self.read_register(rs2) as i32));
                let product = mul_i32_wide_16(lhs, rhs);
                self.write_register(rd, (product >> 32) as u32);
            }
            Instruction::Mulhsu { rd, rs1, rs2 } => {
                let lhs = recompose_i32_16(decompose_i32_16(self.read_register(rs1) as i32));
                let rhs = recompose_u32_16(decompose_u32_16(self.read_register(rs2)));
                let product = mul_i32_u32_wide_16(lhs, rhs);
                self.write_register(rd, (product >> 32) as u32);
            }
            Instruction::Mulhu { rd, rs1, rs2 } => {
                let lhs = recompose_u32_16(decompose_u32_16(self.read_register(rs1)));
                let rhs = recompose_u32_16(decompose_u32_16(self.read_register(rs2)));
                let product = mul_u32_wide_16(lhs, rhs);
                self.write_register(rd, (product >> 32) as u32);
            }
            Instruction::Div { rd, rs1, rs2 } => {
                let lhs = recompose_i32_16(decompose_i32_16(self.read_register(rs1) as i32));
                let rhs = recompose_i32_16(decompose_i32_16(self.read_register(rs2) as i32));
                let value = if rhs == 0 {
                    u32::MAX
                } else if lhs == i32::MIN && rhs == -1 {
                    lhs as u32
                } else {
                    (lhs / rhs) as u32
                };
                self.write_register(rd, value);
            }
            Instruction::Divu { rd, rs1, rs2 } => {
                let lhs = recompose_u32_16(decompose_u32_16(self.read_register(rs1)));
                let rhs = recompose_u32_16(decompose_u32_16(self.read_register(rs2)));
                let value = if rhs == 0 { u32::MAX } else { lhs / rhs };
                self.write_register(rd, value);
            }
            Instruction::Rem { rd, rs1, rs2 } => {
                let lhs = recompose_i32_16(decompose_i32_16(self.read_register(rs1) as i32));
                let rhs = recompose_i32_16(decompose_i32_16(self.read_register(rs2) as i32));
                let value = if rhs == 0 {
                    lhs as u32
                } else if lhs == i32::MIN && rhs == -1 {
                    0
                } else {
                    (lhs % rhs) as u32
                };
                self.write_register(rd, value);
            }
            Instruction::Remu { rd, rs1, rs2 } => {
                let lhs = recompose_u32_16(decompose_u32_16(self.read_register(rs1)));
                let rhs = recompose_u32_16(decompose_u32_16(self.read_register(rs2)));
                let value = if rhs == 0 { lhs } else { lhs % rhs };
                self.write_register(rd, value);
            }

            Instruction::Fence => {}
            Instruction::FenceI => {}
            Instruction::Ecall => {
                self.halted = true;
            }
            Instruction::Ebreak => {
                self.halted = true;
            }
        }

        self.registers[0] = 0;
        self.pc = next_pc;
        Ok(())
    }

    pub fn run(&mut self, max_steps: usize) -> Result<usize, ZkvmError> {
        let mut steps = 0usize;
        while !self.halted && steps < max_steps {
            self.step()?;
            steps += 1;
        }
        Ok(steps)
    }

    fn read_register(&self, index: u8) -> u32 {
        self.registers[index as usize]
    }

    fn write_register(&mut self, index: u8, value: u32) {
        if index != 0 {
            self.registers[index as usize] = value;
        }
    }

    fn fetch_word(&self) -> Result<u32, ZkvmError> {
        if self.pc & 0x3 != 0 {
            return Err(ZkvmError::MisalignedInstruction { pc: self.pc });
        }

        let start = self
            .checked_range(self.pc, 4)
            .map_err(|_| ZkvmError::PcOutOfBounds { pc: self.pc })?;
        Ok(u32::from_le_bytes([
            self.memory[start],
            self.memory[start + 1],
            self.memory[start + 2],
            self.memory[start + 3],
        ]))
    }

    fn checked_range(&self, addr: u32, size: usize) -> Result<usize, ZkvmError> {
        let start = addr as usize;
        let end = start
            .checked_add(size)
            .ok_or(ZkvmError::MemoryOutOfBounds { addr, size })?;
        if end > self.memory.len() {
            return Err(ZkvmError::MemoryOutOfBounds { addr, size });
        }
        Ok(start)
    }

    fn read_u8(&self, addr: u32) -> Result<u8, ZkvmError> {
        let start = self.checked_range(addr, 1)?;
        Ok(self.memory[start])
    }

    fn read_u16(&self, addr: u32) -> Result<u16, ZkvmError> {
        if addr & 0x1 != 0 {
            return Err(ZkvmError::MisalignedLoad { addr, width: 2 });
        }
        let start = self.checked_range(addr, 2)?;
        Ok(u16::from_le_bytes([self.memory[start], self.memory[start + 1]]))
    }

    fn read_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        if addr & 0x3 != 0 {
            return Err(ZkvmError::MisalignedLoad { addr, width: 4 });
        }
        let start = self.checked_range(addr, 4)?;
        Ok(u32::from_le_bytes([
            self.memory[start],
            self.memory[start + 1],
            self.memory[start + 2],
            self.memory[start + 3],
        ]))
    }

    fn write_u8(&mut self, addr: u32, value: u8) -> Result<(), ZkvmError> {
        let start = self.checked_range(addr, 1)?;
        self.memory[start] = value;
        Ok(())
    }

    fn write_u16(&mut self, addr: u32, value: u16) -> Result<(), ZkvmError> {
        if addr & 0x1 != 0 {
            return Err(ZkvmError::MisalignedStore { addr, width: 2 });
        }
        let start = self.checked_range(addr, 2)?;
        let bytes = value.to_le_bytes();
        self.memory[start..start + 2].copy_from_slice(&bytes);
        Ok(())
    }

    fn write_u32(&mut self, addr: u32, value: u32) -> Result<(), ZkvmError> {
        if addr & 0x3 != 0 {
            return Err(ZkvmError::MisalignedStore { addr, width: 4 });
        }
        let start = self.checked_range(addr, 4)?;
        let bytes = value.to_le_bytes();
        self.memory[start..start + 4].copy_from_slice(&bytes);
        Ok(())
    }
}

impl fmt::Display for Zkvm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Zkvm {{ pc: 0x{:08x}, halted: {} }}", self.pc, self.halted)
    }
}
