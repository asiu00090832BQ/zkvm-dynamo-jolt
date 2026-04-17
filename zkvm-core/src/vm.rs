use std::error::Error;
use std::fmt;

use crate::decoder::{DecodeError, Decoder, Instruction, Register, Rv32ImDecoder};
use rv32im_decoder::decoder::rv32m;

#[derive(Debug, Clone)]
pub struct ZkvmConfig {
    pub max_steps: usize,
    pub strict_alignment: bool,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            max_steps: 1_000_000,
            strict_alignment: true,
        }
    }
}

#[derive(Debug)]
pub enum ZkvmError {
    Decode(DecodeError),
    MemoryOutOfBounds { addr: u32, size: usize },
    MisalignedAccess { addr: u32, size: usize },
    InvalidProgramCounter(u32),
    Breakpoint(u32),
    MaxStepsExceeded(usize),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(error) => write!(f, "{error}"),
            Self::MemoryOutOfBounds { addr, size } => {
                write!(f, "memory out of bounds: addr=0x{addr:08x}, size={size}")
            }
            Self::MisalignedAccess { addr, size } => {
                write!(f, "misaligned access: addr=0x{addr:08x}, size={size}")
            }
            Self::InvalidProgramCounter(pc) => write!(f, "invalid program counter: 0x{pc:08x}"),
            Self::Breakpoint(pc) => write!(f, "breakpoint at pc=0x{pc:08x}"),
            Self::MaxStepsExceeded(steps) => write!(f, "max steps exceeded: {steps}"),
        }
    }
}

impl Error for ZkvmError {}

impl From<DecodeError> for ZkvmError {
    fn from(value: DecodeError) -> Self {
        Self::Decode(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    Continue,
    Halt,
}

#[derive(Debug, Clone)]
pub struct Zkvm {
    pub regs: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
    pub config: ZkvmConfig,
    pub halted: bool,
    decoder: Rv32ImDecoder,
}

impl Zkvm {
    pub fn new(memory_size: usize, config: ZkvmConfig) -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
            memory: vec![0; memory_size],
            config,
            halted: false,
            decoder: Rv32ImDecoder::new(),
        }
    }

    pub fn with_memory(memory: Vec<u8>, config: ZkvmConfig) -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
            memory,
            config,
            halted: false,
            decoder: Rv32ImDecoder::new(),
        }
    }

    pub fn reset(&mut self, pc: u32) {
        self.regs = [0; 32];
        self.pc = pc;
        self.halted = false;
    }

    pub fn load_program(&mut self, base: u32, program: &[u8]) -> Result<(), ZkvmError> {
        let start = self.check_range(base, program.len(), false)?;
        self.memory[start..start + program.len()].copy_from_slice(program);
        self.pc = base;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), ZkvmError> {
        let mut steps = 0usize;
        while !self.halted {
            if steps >= self.config.max_steps {
                return Err(ZkvmError::MaxStepsExceeded(steps));
            }

            match self.step()? {
                StepOutcome::Continue => steps += 1,
                StepOutcome::Halt => return Ok(()),
            }
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<StepOutcome, ZkvmError> {
        if self.halted {
            return Ok(StepOutcome::Halt);
        }

        if self.config.strict_alignment && (self.pc & 0b11) != 0 {
            return Err(ZkvmError::InvalidProgramCounter(self.pc));
        }

        let current_pc = self.pc;
        let word = self.fetch_u32(current_pc)?;
        let instruction = self.decoder.decode(word)?;
        let mut next_pc = current_pc.wrapping_add(4);

        match instruction {
            Instruction::Lui { rd, imm } => {
                self.write_reg(rd, imm as u32);
            }
            Instruction::Auipc { rd, imm } => {
                self.write_reg(rd, current_pc.wrapping_add(imm as u32));
            }
            Instruction::Jal { rd, imm } => {
                self.write_reg(rd, current_pc.wrapping_add(4));
                next_pc = Self::add_offset(current_pc, imm);
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let target = Self::add_offset(self.read_reg(rs1), imm) & !1;
                self.write_reg(rd, current_pc.wrapping_add(4));
                next_pc = target;
            }
            Instruction::Beq { rs1, rs2, imm } => {
                if self.read_reg(rs1) == self.read_reg(rs2) {
                    next_pc = Self::add_offset(current_pc, imm);
                }
            }
            Instruction::Bne { rs1, rs2, imm } => {
                if self.read_reg(rs1) != self.read_reg(rs2) {
                    next_pc = Self::add_offset(current_pc, imm);
                }
            }
            Instruction::Blt { rs1, rs2, imm } => {
                if (self.read_reg(rs1) as i32) < (self.read_reg(rs2) as i32) {
                    next_pc = Self::add_offset(current_pc, imm);
                }
            }
            Instruction::Bge { rs1, rs2, imm } => {
                if (self.read_reg(rs1) as i32) >= (self.read_reg(rs2) as i32) {
                    next_pc = Self::add_offset(current_pc, imm);
                }
            }
            Instruction::Bltu { rs1, rs2, imm } => {
                if self.read_reg(rs1) < self.read_reg(rs2) {
                    next_pc = Self::add_offset(current_pc, imm);
                }
            }
            Instruction::Bgeu { rs1, rs2, imm } => {
                if self.read_reg(rs1) >= self.read_reg(rs2) {
                    next_pc = Self::add_offset(current_pc, imm);
                }
            }
            Instruction::Lb { rd, rs1, imm } => {
                let addr = Self::add_offset(self.read_reg(rs1), imm);
                self.write_reg(rd, self.load_i8(addr)? as u32);
            }
            Instruction::Lh { rd, rs1, imm } => {
                let addr = Self::add_offset(self.read_reg(rs1), imm);
                self.write_reg(rd, self.load_i16(addr)? as u32);
            }
            Instruction::Lw { rd, rs1, imm } => {
                let addr = Self::add_offset(self.read_reg(rs1), imm);
                self.write_reg(rd, self.load_u32(addr)?);
            }
            Instruction::Lbu { rd, rs1, imm } => {
                let addr = Self::add_offset(self.read_reg(rs1), imm);
                self.write_reg(rd, self.load_u8(addr)? as u32);
            }
            Instruction::Lhu { rd, rs1, imm } => {
                let addr = Self::add_offset(self.read_reg(rs1), imm);
                self.write_reg(rd, self.load_u16(addr)? as u32);
            }
            Instruction::Sb { rs1, rs2, imm } => {
                let addr = Self::add_offset(self.read_reg(rs1), imm);
                self.store_u8(addr, self.read_reg(rs2) as u8)?;
            }
            Instruction::Sh { rs1, rs2, imm } => {
                let addr = Self::add_offset(self.read_reg(rs1), imm);
                self.store_u16(addr, self.read_reg(rs2) as u16)?;
            }
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = Self::add_offset(self.read_reg(rs1), imm);
                self.store_u32(addr, self.read_reg(rs2))?;
            }
            Instruction::Addi { rd, rs1, imm } => {
                self.write_reg(rd, self.read_reg(rs1).wrapping_add(imm as u32));
            }
            Instruction::Slti { rd, rs1, imm } => {
                self.write_reg(rd, Self::bit((self.read_reg(rs1) as i32) < imm));
            }
            Instruction::Sltiu { rd, rs1, imm } => {
                self.write_reg(rd, Self::bit(self.read_reg(rs1) < imm as u32));
            }
            Instruction::Xori { rd, rs1, imm } => {
                self.write_reg(rd, self.read_reg(rs1) ^ imm as u32);
            }
            Instruction::Ori { rd, rs1, imm } => {
                self.write_reg(rd, self.read_reg(rs1) | imm as u32);
            }
            Instruction::Andi { rd, rs1, imm } => {
                self.write_reg(rd, self.read_reg(rs1) & imm as u32);
            }
            Instruction::Slli { rd, rs1, shamt } => {
                self.write_reg(rd, self.read_reg(rs1) << shamt);
            }
            Instruction::Srli { rd, rs1, shamt } => {
                self.write_reg(rd, self.read_reg(rs1) >> shamt);
            }
            Instruction::Srai { rd, rs1, shamt } => {
                self.write_reg(rd, ((self.read_reg(rs1) as i32) >> shamt) as u32);
            }
            Instruction::Add { rd, rs1, rs2 } => {
                self.write_reg(rd, self.read_reg(rs1).wrapping_add(self.read_reg(rs2)));
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                self.write_reg(rd, self.read_reg(rs1).wrapping_sub(self.read_reg(rs2)));
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                let shamt = self.read_reg(rs2) & 0x1f;
                self.write_reg(rd, self.read_reg(rs1) << shamt);
            }
            Instruction::Slt { rd, rs1, rs2 } => {
                self.write_reg(
                    rd,
                    Self::bit((self.read_reg(rs1) as i32) < (self.read_reg(rs2) as i32)),
                );
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                self.write_reg(rd, Self::bit(self.read_reg(rs1) < self.read_reg(rs2)));
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                self.write_reg(rd, self.read_reg(rs1) ^ self.read_reg(rs2));
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                let shamt = self.read_reg(rs2) & 0x1f;
                self.write_reg(rd, self.read_reg(rs1) >> shamt);
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                let shamt = self.read_reg(rs2) & 0x1f;
                self.write_reg(rd, ((self.read_reg(rs1) as i32) >> shamt) as u32);
            }
            Instruction::Or { rd, rs1, rs2 } => {
                self.write_reg(rd, self.read_reg(rs1) | self.read_reg(rs2));
            }
            Instruction::And { rd, rs1, rs2 } => {
                self.write_reg(rd, self.read_reg(rs1) & self.read_reg(rs2));
            }
            Instruction::Fence { .. } => {}
            Instruction::Ecall => {
                self.halted = true;
                return Ok(StepOutcome::Halt);
            }
            Instruction::Ebreak => {
                return Err(ZkvmError::Breakpoint(current_pc));
            }
            Instruction::Mul { rd, rs1, rs2 } => {
                let value = rv32m::mul_low_u32(self.read_reg(rs1), self.read_reg(rs2))?;
                self.write_reg(rd, value);
            }
            Instruction::Mulh { rd, rs1, rs2 } => {
                let value = rv32m::mulh_signed(
                    self.read_reg(rs1) as i32,
                    self.read_reg(rs2) as i32,
                )?;
                self.write_reg(rd, value);
            }
            Instruction::Mulhsu { rd, rs1, rs2 } => {
                let value = rv32m::mulhsu_signed_unsigned(
                    self.read_reg(rs1) as i32,
                    self.read_reg(rs2),
                )?;
                self.write_reg(rd, value);
            }
            Instruction::Mulhu { rd, rs1, rs2 } => {
                let value = rv32m::mulhu_unsigned(self.read_reg(rs1), self.read_reg(rs2))?;
                self.write_reg(rd, value);
            }
            Instruction::Div { rd, rs1, rs2 } => {
                let value =
                    rv32m::div_signed(self.read_reg(rs1) as i32, self.read_reg(rs2) as i32)?;
                self.write_reg(rd, value);
            }
            Instruction::Divu { rd, rs1, rs2 } => {
                let value = rv32m::div_unsigned(self.read_reg(rs1), self.read_reg(rs2))?;
                self.write_reg(rd, value);
            }
            Instruction::Rem { rd, rs1, rs2 } => {
                let value =
                    rv32m::rem_signed(self.read_reg(rs1) as i32, self.read_reg(rs2) as i32)?;
                self.write_reg(rd, value);
            }
            Instruction::Remu { rd, rs1, rs2 } => {
                let value = rv32m::rem_unsigned(self.read_reg(rs1), self.read_reg(rs2))?;
                self.write_reg(rd, value);
            }
        }

        self.regs[0] = 0;
        self.pc = next_pc;
        Ok(StepOutcome::Continue)
    }

    fn read_reg(&self, register: Register) -> u32 {
        self.regs[register.index()]
    }

    fn write_reg(&mut self, register: Register, value: u32) {
        if register != Register::X0 {
            self.regs[register.index()] = value;
        }
    }

    fn bit(value: bool) -> u32 {
        if value { 1 } else { 0 }
    }

    fn add_offset(base: u32, offset: i32) -> u32 {
        base.wrapping_add(offset as u32)
    }

    fn check_range(
        &self,
        addr: u32,
        size: usize,
        require_alignment: bool,
    ) -> Result<usize, ZkvmError> {
        let start = addr as usize;
        let end = start
            .checked_add(size)
            .ok_or(ZkvmError::MemoryOutOfBounds { addr, size })?;

        if end > self.memory.len() {
            return Err(ZkvmError::MemoryOutOfBounds { addr, size });
        }

        if require_alignment && self.config.strict_alignment && size > 1 && start % size != 0 {
            return Err(ZkvmError::MisalignedAccess { addr, size });
        }

        Ok(start)
    }

    fn fetch_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        let start = self.check_range(addr, 4, true)?;
        Ok(u32::from_le_bytes([
            self.memory[start],
            self.memory[start + 1],
            self.memory[start + 2],
            self.memory[start + 3],
        ]))
    }

    fn load_u8(&self, addr: u32) -> Result<u8, ZkvmError> {
        let start = self.check_range(addr, 1, false)?;
        Ok(self.memory[start])
    }

    fn load_i8(&self, addr: u32) -> Result<i32, ZkvmError> {
        Ok((self.load_u8(addr)? as i8) as i32)
    }

    fn load_u16(&self, addr: u32) -> Result<u16, ZkvmError> {
        let start = self.check_range(addr, 2, true)?;
        Ok(u16::from_le_bytes([self.memory[start], self.memory[start + 1]]))
    }

    fn load_i16(&self, addr: u32) -> Result<i32, ZkvmError> {
        Ok((self.load_u16(addr)? as i16) as i32)
    }

    fn load_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        let start = self.check_range(addr, 4, true)?;
        Ok(u32::from_le_bytes([
            self.memory[start],
            self.memory[start + 1],
            self.memory[start + 2],
            self.memory[start + 3],
        ]))
    }

    fn store_u8(&mut self, addr: u32, value: u8) -> Result<(), ZkvmError> {
        let start = self.check_range(addr, 1, false)?;
        self.memory[start] = value;
        Ok(())
    }

    fn store_u16(&mut self, addr: u32, value: u16) -> Result<(), ZkvmError> {
        let start = self.check_range(addr, 2, true)?;
        let bytes = value.to_le_bytes();
        self.memory[start..start + 2].copy_from_slice(&bytes);
        Ok(())
    }

    fn store_u32(&mut self, addr: u32, value: u32) -> Result<(), ZkvmError> {
        let start = self.check_range(addr, 4, true)?;
        let bytes = value.to_le_bytes();
        self.memory[start..start + 4].copy_from_slice(&bytes);
        Ok(())
    }
}
