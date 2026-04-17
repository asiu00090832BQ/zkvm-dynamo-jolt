use core::fmt;
use rv32im_decoder::{
    decode, div_i32, div_u32, mul_u32_low, mulh_i32_i32, mulhsu_i32_u32, mulhu_u32_u32, rem_i32,
    rem_u32, DecodeError, Instruction,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    DecodeError(DecodeError),
    MemoryOutOfBounds { addr: u32, size: usize },
    MisalignedAccess { addr: u32, size: usize },
    EnvironmentCall,
    Breakpoint,
    StepLimitExceeded { max_steps: usize },
    InvalidElf,
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DecodeError(error) => write!(f, "{error}"),
            Self::MemoryOutOfBounds { addr, size } => {
                write!(f, "memory access out of bounds at 0x{addr:08x} for {size} bytes")
            }
            Self::MisalignedAccess { addr, size } => {
                write!(f, "misaligned access at 0x{addr:08x} for {size} bytes")
            }
            Self::EnvironmentCall => write!(f, "environment call"),
            Self::Breakpoint => write!(f, "breakpoint"),
            Self::StepLimitExceeded { max_steps } => {
                write!(f, "maximum step count exceeded: {max_steps}")
            }
            Self::InvalidElf => write!(f, "invalid ELF executable"),
        }
    }
}

impl std::error::Error for ZkvmError {}

impl From<DecodeError> for ZkvmError {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError(value)
    }
}

#[derive(Debug, Clone)]
pub struct Zkvm {
    regs: [u32; 32],
    pc: u32,
    memory: Vec<u8>,
}

impl Zkvm {
    pub fn new(memory_size: usize) -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
            memory: vec![0; memory_size],
        }
    }

    pub fn with_memory(memory: Vec<u8>) -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
            memory,
        }
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, value: u32) {
        self.pc = value;
    }

    pub fn registers(&self) -> &[u32; 32] {
        &self.regs
    }

    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut [u8] {
        &mut self.memory
    }

    pub fn step(&mut self) -> Result<(), ZkvmError> {
        let pc = self.pc;
        let word = self.read_u32(pc)?;
        let instruction = decode(word)?;
        self.execute_instruction(pc, instruction)
    }

    fn execute_instruction(&mut self, pc: u32, instruction: Instruction) -> Result<(), ZkvmError> {
        let next_pc = pc.wrapping_add(4);
        let mut updated_pc = next_pc;

        match instruction {
            Instruction::Lui { rd, imm } => self.write_reg(rd, imm),
            Instruction::Auipc { rd, imm } => self.write_reg(rd, pc.wrapping_add(imm)),
            Instruction::Jal { rd, imm } => {
                self.write_reg(rd, next_pc);
                updated_pc = pc.wrapping_add(imm as u32);
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.read_reg(rs1).wrapping_add(imm as u32) & !1;
                self.write_reg(rd, next_pc);
                updated_pc = target;
            }
            Instruction::Beq { rs1, rs2, imm } => {
                if self.read_reg(rs1) == self.read_reg(rs2) { updated_pc = pc.wrapping_add(imm as u32); }
            }
            Instruction::Bne { rs1, rs2, imm } => {
                if self.read_reg(rs1) != self.read_reg(rs2) { updated_pc = pc.wrapping_add(imm as u32); }
            }
            Instruction::Blt { rs1, rs2, imm } => {
                if (self.read_reg(rs1) as i32) < (self.read_reg(rs2) as i32) { updated_pc = pc.wrapping_add(imm as u32); }
            }
            Instruction::Bge { rs1, rs2, imm } => {
                if (self.read_reg(rs1) as i32) >= (self.read_reg(rs2) as i32) { updated_pc = pc.wrapping_add(imm as u32); }
            }
            Instruction::Bltu { rs1, rs2, imm } => {
                if self.read_reg(rs1) < self.read_reg(rs2) { updated_pc = pc.wrapping_add(imm as u32); }
            }
            Instruction::Bgeu { rs1, rs2, imm } => {
                if self.read_reg(rs1) >= self.read_reg(rs2) { updated_pc = pc.wrapping_add(imm as u32); }
            }
            Instruction::Lb { rd, rs1, imm } => {
                let val = self.read_u8(self.read_reg(rs1).wrapping_add(imm as u32))? as i8 as i32 as u32;
                self.write_reg(rd, val);
            }
            Instruction::Lh { rd, rs1, imm } => {
                let val = self.read_u16(self.read_reg(rs1).wrapping_add(imm as u32))? as i16 as i32 as u32;
                self.write_reg(rd, val);
            }
            Instruction::Lw { rd, rs1, imm } => {
                let val = self.read_u32(self.read_reg(rs1).wrapping_add(imm as u32))?;
                self.write_reg(rd, val);
            }
            Instruction::Lbu { rd, rs1, imm } => {
                let val = self.read_u8(self.read_reg(rs1).wrapping_add(imm as u32))? as u32;
                self.write_reg(rd, val);
            }
            Instruction::Lhu { rd, rs1, imm } => {
                let val = self.read_u16(self.read_reg(rs1).wrapping_add(imm as u32))? as u32;
                self.write_reg(rd, val);
            }
            Instruction::Sb { rs1, rs2, imm } => self.write_u8(self.read_reg(rs1).wrapping_add(imm as u32), self.read_reg(rs2) as u8)?,
            Instruction::Sh { rs1, rs2, imm } => self.write_u16(self.read_reg(rs1).wrapping_add(imm as u32), self.read_reg(rs2) as u16)?,
            Instruction::Sw { rs1, rs2, imm } => self.write_u32(self.read_reg(rs1).wrapping_add(imm as u32), self.read_reg(rs2))?,
            Instruction::Addi { rd, rs1, imm } => self.write_reg(rd, self.read_reg(rs1).wrapping_add(imm as u32)),
            Instruction::Slti { rd, rs1, imm } => self.write_reg(rd, ((self.read_reg(rs1) as i32) < imm) as u32),
            Instruction::Sltiu { rd, rs1, imm } => self.write_reg(rd, (self.read_reg(rs1) < imm as u32) as u32),
            Instruction::Xori { rd, rs1, imm } => self.write_reg(rd, self.read_reg(rs1) ^ imm as u32),
            Instruction::Ori { rd, rs1, imm } => self.write_reg(rd, self.read_reg(rs1) | imm as u32),
            Instruction::Andi { rd, rs1, imm } => self.write_reg(rd, self.read_reg(rs1) & imm as u32),
            Instruction::Slli { rd, rs1, shamt } => self.write_reg(rd, self.read_reg(rs1) << shamt),
            Instruction::Srli { rd, rs1, shamt } => self.write_reg(rd, self.read_reg(rs1) >> shamt),
            Instruction::Srai { rd, rs1, shamt } => self.write_reg(rd, ((self.read_reg(rs1) as i32) >> shamt) as u32),
            Instruction::Add { rd, rs1, rs2 } => self.write_reg(rd, self.read_reg(rs1).wrapping_add(self.read_reg(rs2))),
            Instruction::Sub { rd, rs1, rs2 } => self.write_reg(rd, self.read_reg(rs1).wrapping_sub(self.read_reg(rs2))),
            Instruction::Sll { rd, rs1, rs2 } => self.write_reg(rd, self.read_reg(rs1) << (self.read_reg(rs2) & 0x1f)),
            Instruction::Slt { rd, rs1, rs2 } => self.write_reg(rd, ((self.read_reg(rs1) as i32) < (self.read_reg(rs2) as i32)) as u32),
            Instruction::Sltu { rd, rs1, rs2 } => self.write_reg(rd, (self.read_reg(rs1) < self.read_reg(rs2)) as u32),
            Instruction::Xor { rd, rs1, rs2 } => self.write_reg(rd, self.read_reg(rs1) ^ self.read_reg(rs2)),
            Instruction::Srl { rd, rs1, rs2 } => self.write_reg(rd, self.read_reg(rs1) >> (self.read_reg(rs2) & 0x1f)),
            Instruction::Sra { rd, rs1, rs2 } => self.write_reg(rd, ((self.read_reg(rs1) as i32) >> (self.read_reg(rs2) & 0x1f)) as u32),
            Instruction::Or { rd, rs1, rs2 } => self.write_reg(rd, self.read_reg(rs1) | self.read_reg(rs2)),
            Instruction::And { rd, rs1, rs2 } => self.write_reg(rd, self.read_reg(rs1) & self.read_reg(rs2)),
            Instruction::Mul { rd, rs1, rs2 } => self.write_reg(rd, mul_u32_low(self.read_reg(rs1), self.read_reg(rs2))),
            Instruction::Mulh { rd, rs1, rs2 } => self.write_reg(rd, mulh_i32_i32(self.read_reg(rs1), self.read_reg(rs2))),
            Instruction::Mulhsu { rd, rs1, rs2 } => self.write_reg(rd, mulhsu_i32_u32(self.read_reg(rs1), self.read_reg(rs2))),
            Instruction::Mulhu { rd, rs1, rs2 } => self.write_reg(rd, mulhu_u32_u32(self.read_reg(rs1), self.read_reg(rs2))),
            Instruction::Div { rd, rs1, rs2 } => self.write_reg(rd, div_i32(self.read_reg(rs1), self.read_reg(rs2))),
            Instruction::Divu { rd, rs1, rs2 } => self.write_reg(rd, div_u32(self.read_reg(rs1), self.read_reg(rs2))),
            Instruction::Rem { rd, rs1, rs2 } => self.write_reg(rd, rem_i32(self.read_reg(rs1), self.read_reg(rs2))),
            Instruction::Remu { rd, rs1, rs2 } => self.write_reg(rd, rem_u32(self.read_reg(rs1), self.read_reg(rs2))),
            Instruction::Fence | Instruction::FenceI => {}
            Instruction::Ecall => return Err(ZkvmError::EnvironmentCall),
            Instruction::Ebreak => return Err(ZkvmError::Breakpoint),
        }

        self.pc = updated_pc;
        self.regs[0] = 0;
        Ok(())
    }

    fn read_reg(&self, index: u8) -> u32 { self.regs[index as usize] }
    fn write_reg(&mut self, index: u8, value: u32) { if index != 0 { self.regs[index as usize] = value; } }

    fn read_u8(&self, addr: u32) -> Result<u8, ZkvmError> {
        let start = addr as usize;
        if start >= self.memory.len() { return Err(ZkvmError::MemoryOutOfBounds { addr, size: 1 }); }
        Ok(self.memory[start])
    }

    fn read_u16(&self, addr: u32) -> Result<u16, ZkvmError> {
        if addr % 2 != 0 { return Err(ZkvmError::MisalignedAccess { addr, size: 2 }); }
        let start = addr as usize;
        if start + 1 >= self.memory.len() { return Err(ZkvmError::MemoryOutOfBounds { addr, size: 2 }); }
        Ok(u16::from_le_bytes([self.memory[start], self.memory[start + 1]]))
    }

    fn read_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        if addr % 4 != 0 { return Err(ZkvmError::MisalignedAccess { addr, size: 4 }); }
        let start = addr as usize;
        if start + 3 >= self.memory.len() { return Err(ZkvmError::MemoryOutOfBounds { addr, size: 4 }); }
        Ok(u32::from_le_bytes([self.memory[start], self.memory[start + 1], self.memory[start + 2], self.memory[start + 3]]))
    }

    fn write_u8(&mut self, addr: u32, val: u8) -> Result<(), ZkvmError> {
        let start = addr as usize;
        if start >= self.memory.len() { return Err(ZkvmError::MemoryOutOfBounds { addr, size: 1 }); }
        self.memory[start] = val;
        Ok(())
    }

    fn write_u16(&mut self, addr: u32, val: u16) -> Result<(), ZkvmError> {
        if addr % 2 != 0 { return Err(ZkvmError::MisalignedAccess { addr, size: 2 }); }
        let start = addr as usize;
        if start + 1 >= self.memory.len() { return Err(ZkvmError::MemoryOutOfBounds { addr, size: 2 }); }
        self.memory[start..start + 2].copy_from_slice(&val.to_le_bytes());
        Ok(())
    }

    fn write_u32(&mut self, addr: u32, val: u32) -> Result<(), ZkvmError> {
        if addr % 4 != 0 { return Err(ZkvmError::MisalignedAccess { addr, size: 4 }); }
        let start = addr as usize;
        if start + 3 >= self.memory.len() { return Err(ZkvmError::MemoryOutOfBounds { addr, size: 4 }); }
        self.memory[start..start + 4].copy_from_slice(&val.to_le_bytes());
        Ok(())
    }
}
