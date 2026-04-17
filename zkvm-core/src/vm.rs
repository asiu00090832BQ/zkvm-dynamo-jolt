// zkvm-core/src/decoder.rs has been removed; decoding now comes from rv32im-decoder.

use core::fmt;

use rv32im_decoder::{
    decode, BranchKind, DecodeError, Instruction, Limb16, LoadKind, OpImmKind, OpKind, StoreKind,
    SystemKind,
};

#[derive(Debug)]
pub enum ZkvmError {
    Decode(DecodeError),
    MisalignedFetch(u32),
    MemoryOutOfBounds { addr: u32, size: usize },
    MisalignedLoad { addr: u32, size: usize },
    MisalignedStore { addr: u32, size: usize },
    StepLimitExceeded(usize),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(err) => write!(f, "{err}"),
            Self::MisalignedFetch(pc) => write!(f, "misaligned instruction fetch at 0x{pc:08x}"),
            Self::MemoryOutOfBounds { addr, size } => {
                write!(f, "memory access out of bounds at 0x{addr:08x} for {size} bytes")
            }
            Self::MisalignedLoad { addr, size } => {
                write!(f, "misaligned load at 0x{addr:08x} for {size} bytes")
            }
            Self::MisalignedStore { addr, size } => {
                write!(f, "misaligned store at 0x{addr:08x} for {size} bytes")
            }
            Self::StepLimitExceeded(limit) => write!(f, "step limit exceeded: {limit}"),
        }
    }
}

impl std::error::Error for ZkvmError {}

impl From<DecodeError> for ZkvmError {
    fn from(value: DecodeError) -> Self {
        Self::Decode(value)
    }
}

#[derive(Clone, Debug)]
pub struct Zkvm {
    pub regs: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
    pub halted: bool,
    pub exit_code: Option<u32>,
}

impl Zkvm {
    pub fn new(memory_size: usize) -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
            memory: vec![0; memory_size],
            halted: false,
            exit_code: None,
        }
    }

    pub fn load_program(&mut self, base: u32, program: &[u8]) -> Result<(), ZkvmError> {
        let start = base as usize;
        let end = start
            .checked_add(program.len())
            .ok_or(ZkvmError::MemoryOutOfBounds {
                addr: base,
                size: program.len(),
            })?;
        if end > self.memory.len() {
            return Err(ZkvmError::MemoryOutOfBounds {
                addr: base,
                size: program.len(),
            });
        }
        self.memory[start..end].copy_from_slice(program);
        self.pc = base;
        Ok(())
    }

    pub fn run(&mut self, max_steps: usize) -> Result<(), ZkvmError> {
        for step in 0..max_steps {
            if self.halted {
                return Ok(());
            }
            self.step()?;
            if self.halted {
                return Ok(());
            }
            if step + 1 == max_steps {
                return Err(ZkvmError::StepLimitExceeded(max_steps));
            }
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), ZkvmError> {
        let word = self.fetch_u32(self.pc)?;
        let instruction = decode(word)?;
        self.execute(instruction)
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), ZkvmError> {
        let next_pc = self.pc.wrapping_add(4);

        match instruction {
            Instruction::Lui { rd, imm } => {
                self.write_reg(rd, imm as u32);
                self.pc = next_pc;
            }
            Instruction::Auipc { rd, imm } => {
                self.write_reg(rd, self.pc.wrapping_add(imm as u32));
                self.pc = next_pc;
            }
            Instruction::Jal { rd, imm } => {
                self.write_reg(rd, next_pc);
                self.pc = self.pc.wrapping_add(imm as u32);
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.read_reg(rs1).wrapping_add(imm as u32) & !1;
                self.write_reg(rd, next_pc);
                self.pc = target;
            }
            Instruction::Branch { kind, rs1, rs2, imm } => {
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
                self.pc = if taken {
                    self.pc.wrapping_add(imm as u32)
                } else {
                    next_pc
                };
            }
            Instruction::Load { kind, rd, rs1, imm } => {
                let addr = self.read_reg(rs1).wrapping_add(imm as u32);
                let value = match kind {
                    LoadKind::Lb => self.load_u8(addr)? as i8 as i32 as u32,
                    LoadKind::Lh => self.load_u16(addr)? as i16 as i32 as u32,
                    LoadKind::Lw => self.load_u32(addr)?,
                    LoadKind::Lbu => self.load_u8(addr)? as u32,
                    LoadKind::Lhu => self.load_u16(addr)? as u32,
                };
                self.write_reg(rd, value);
                self.pc = next_pc;
            }
            Instruction::Store { kind, rs1, rs2, imm } => {
                let addr = self.read_reg(rs1).wrapping_add(imm as u32);
                let value = self.read_reg(rs2);
                match kind {
                    StoreKind::Sb => self.store_u8(addr, value as u8)?,
                    StoreKind::Sh => self.store_u16(addr, value as u16)?,
                    StoreKind::Sw => self.store_u32(addr, value)?,
                }
                self.pc = next_pc;
            }
            Instruction::OpImm { kind, rd, rs1, imm } => {
                let lhs = self.read_reg(rs1);
                let value = match kind {
                    OpImmKind::Addi => lhs.wrapping_add(imm as u32),
                    OpImmKind::Slti => ((lhs as i32) < imm) as u32,
                    OpImmKind::Sltiu => (lhs < imm as u32) as u32,
                    OpImmKind::Xori => lhs ^ imm as u32,
                    OpImmKind::Ori => lhs | imm as u32,
                    OpImmKind::Andi => lhs & imm as u32,
                    OpImmKind::Slli => lhs << (imm as u32 & 0x1f),
                    OpImmKind::Srli => lhs >> (imm as u32 & 0x1f),
                    OpImmKind::Srai => ((lhs as i32) >> (imm as u32 & 0x1f)) as u32,
                };
                self.write_reg(rd, value);
                self.pc = next_pc;
            }
            Instruction::Op { kind, rd, rs1, rs2 } => {
                let lhs = self.read_reg(rs1);
                let rhs = self.read_reg(rs2);
                let value = match kind {
                    OpKind::Add => lhs.wrapping_add(rhs),
                    OpKind::Sub => lhs.wrapping_sub(rhs),
                    OpKind::Sll => lhs << (rhs & 0x1f),
                    OpKind::Slt => ((lhs as i32) < (rhs as i32)) as u32,
                    OpKind::Sltu => (lhs < rhs) as u32,
                    OpKind::Xor => lhs ^ rhs,
                    OpKind::Srl => lhs >> (rhs & 0x1f),
                    OpKind::Sra => ((lhs as i32) >> (rhs & 0x1f)) as u32,
                    OpKind::Or => lhs | rhs,
                    OpKind::And => lhs & rhs,
                    OpKind::Mul => self.mul_wide(lhs, rhs) as u32,
                    OpKind::Mulh => (((lhs as i32 as i64) * (rhs as i32 as i64)) >> 32) as u32,
                    OpKind::Mulhsu => (((lhs as i32 as i64) * (rhs as u64 as i64)) >> 32) as u32,
                    OpKind::Mulhu => (self.mul_wide(lhs, rhs) >> 32) as u32,
                    OpKind::Div => div_signed(lhs, rhs),
                    OpKind::Divu => div_unsigned(lhs, rhs),
                    OpKind::Rem => rem_signed(lhs, rhs),
                    OpKind::Remu => rem_unsigned(lhs, rhs),
                };
                self.write_reg(rd, value);
                self.pc = next_pc;
            }
            Instruction::Fence => {
                self.pc = next_pc;
            }
            Instruction::System { kind } => {
                self.halted = true;
                self.exit_code = Some(match kind {
                    SystemKind::Ecall => self.regs[10],
                    SystemKind::Ebreak => 0,
                });
                self.pc = next_pc;
            }
        }

        self.regs[0] = 0;
        Ok(())
    }

    fn mul_wide(&self, lhs: u32, rhs: u32) -> u64 {
        let a = Limb16::from_u32(lhs);
        let b = Limb16::from_u32(rhs);
        a.widening_mul(b)
    }

    fn read_reg(&self, index: u8) -> u32 {
        self.regs[index as usize]
    }

    fn write_reg(&mut self, index: u8, value: u32) {
        if index != 0 {
            self.regs[index as usize] = value;
        }
    }

    fn fetch_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        if addr & 0b11 != 0 {
            return Err(ZkvmError::MisalignedFetch(addr));
        }
        self.load_u32(addr)
    }

    fn load_u8(&self, addr: u32) -> Result<u8, ZkvmError> {
        let index = addr as usize;
        self.memory
            .get(index)
            .copied()
            .ok_or(ZkvmError::MemoryOutOfBounds { addr, size: 1 })
    }

    fn load_u16(&self, addr: u32) -> Result<u16, ZkvmError> {
        if addr & 0b1 != 0 {
            return Err(ZkvmError::MisalignedLoad { addr, size: 2 });
        }
        let bytes = self.read_bytes(addr, 2)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn load_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        if addr & 0b11 != 0 {
            return Err(ZkvmError::MisalignedLoad { addr, size: 4 });
        }
        let bytes = self.read_bytes(addr, 4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn store_u8(&mut self, addr: u32, value: u8) -> Result<(), ZkvmError> {
        let index = addr as usize;
        let slot = self
            .memory
            .get_mut(index)
            .ok_or(ZkvmError::MemoryOutOfBounds { addr, size: 1 })?;
        *slot = value;
        Ok(())
    }

    fn store_u16(&mut self, addr: u32, value: u16) -> Result<(), ZkvmError> {
        if addr & 0b1 != 0 {
            return Err(ZkvmError::MisalignedStore { addr, size: 2 });
        }
        self.write_bytes(addr, &value.to_le_bytes())
    }

    fn store_u32(&mut self, addr: u32, value: u32) -> Result<(), ZkvmError> {
        if addr & 0b11 != 0 {
            return Err(ZkvmError::MisalignedStore { addr, size: 4 });
        }
        self.write_bytes(addr, &value.to_le_bytes())
    }

    fn read_bytes(&self, addr: u32, size: usize) -> Result<&[u8], ZkvmError> {
        let start = addr as usize;
        let end = start
            .checked_add(size)
            .ok_or(ZkvmError::MemoryOutOfBounds { addr, size })?;
        self.memory
            .get(start..end)
            .ok_or(ZkvmError::MemoryOutOfBounds { addr, size })
    }

    fn write_bytes(&mut self, addr: u32, bytes: &[u8]) -> Result<(), ZkvmError> {
        let start = addr as usize;
        let end = start
            .checked_add(bytes.len())
            .ok_or(ZkvmError::MemoryOutOfBounds {
                addr,
                size: bytes.len(),
            })?;
        let dst = self
            .memory
            .get_mut(start..end)
            .ok_or(ZkvmError::MemoryOutOfBounds {
                addr,
                size: bytes.len(),
            })?;
        dst.copy_from_slice(bytes);
        Ok(())
    }
}

fn div_signed(lhs: u32, rhs: u32) -> u32 {
    let lhs = lhs as i32;
    let rhs = rhs as i32;
    if rhs == 0 {
        u32::MAX
    } else if lhs == i32::MIN && rhs == -1 {
        lhs as u32
    } else {
        (lhs / rhs) as u32
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
    let lhs = lhs as i32;
    let rhs = rhs as i32;
    if rhs == 0 {
        lhs as u32
    } else if lhs == i32::MIN && rhs == -1 {
        0
    } else {
        (lhs % rhs) as u32
    }
}

fn rem_unsigned(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
        lhs
    } else {
        lhs % rhs
    }
}
