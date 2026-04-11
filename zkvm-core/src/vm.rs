use crate::decoder::{decode, DecodeError, Instruction};
use crate::elf_loader::{self, ElfError, ElfImage};

#[derive(Debug, Clone)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub max_cycles: Option<u64>,
    pub start_pc: Option<u32>,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            memory_size: 1024 * 1024,
            max_cycles: None,
            start_pc: None,
        }
    }
}

#[derive(Debug)]
pub enum VmError {
    Decode(DecodeError),
    Elf(ElfError),
    MemoryOutOfBounds { addr: u32, len: usize },
    InvalidInstruction(u32),
    StepLimitReached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    Continue,
    Ecall,
    Breakpoint,
    Halted,
    StepLimitReached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunStats {
    pub steps: u64,
    pub outcome: StepOutcome,
}

pub struct Zkvm {
    regs: [u32; 32],
    pc: u32,
    memory: Vec<u8>,
    config: ZkvmConfig,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            regs: [0u32; 32],
            pc: config.start_pc.unwrap_or(0),
            memory: vec![0u8; config.memory_size],
            config,
        }
    }

    pub fn load_elf(&mut self, image: &ElfImage) -> Result<(), VmError> {
        elf_loader::load_segments_into_memory(&mut self.memory, image).map_err(VmError::Elf)?;
        self.pc = image.entry;
        Ok(())
    }

    pub fn run(&mut self) -> Result<RunStats, VmError> {
        let mut steps: u64 = 0;
        let max_cycles = self.config.max_cycles.unwrap_or(u64::MAX);
        loop {
            if steps >= max_cycles {
                return Ok(RunStats {
                    steps,
                    outcome: StepOutcome::StepLimitReached,
                });
            }
            let pc = self.pc;
            let inst_word = self.fetch_u32(pc)?;
            let inst = match decode(inst_word) {
                Ok(i) => i,
                Err(e) => return Err(VmError::Decode(e)),
            };
            match self.execute(inst) {
                Ok(StepOutcome::Continue) => {
                    steps += 1;
                }
                Ok(outcome) => {
                    steps += 1;
                    return Ok(RunStats { steps, outcome });
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn translate_address(&self, address: u32, len: usize) -> Result<usize, VmError> {
        let start = address as usize;
        let end = start.checked_add(len).ok_or(VmError::MemoryOutOfBounds {
            addr: address,
            len,
        })?;
        if end > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds { addr: address, len });
        }
        Ok(start)
    }

    fn fetch_u32(&self, address: u32) -> Result<u32, VmError> {
        let idx = self.translate_address(address, 4)?;
        let bytes = &self.memory[idx..idx + 4];
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn load_u8(&self, address: u32) -> Result<u8, VmError> {
        let idx = self.translate_address(address, 1)?;
        Ok(self.memory[idx])
    }

    fn load_u16(&self, address: u32) -> Result<u16, VmError> {
        let idx = self.translate_address(address, 2)?;
        let s = &self.memory[idx..idx + 2];
        Ok(u16::from_le_bytes([s[0], s[1]]))
    }

    fn load_u32_mem(&self, address: u32) -> Result<u32, VmError> {
        self.fetch_u32(address)
    }

    fn store_u8(&mut self, address: u32, value: u8) -> Result<(), VmError> {
        let idx = self.translate_address(address, 1)?;
        self.memory[idx] = value;
        Ok(())
    }

    fn store_u16(&mut self, address: u32, value: u16) -> Result<(), VmError> {
        let idx = self.translate_address(address, 2)?;
        let bytes = value.to_le_bytes();
        self.memory[idx] = bytes[0];
        self.memory[idx + 1] = bytes[1];
        Ok(())
    }

    fn store_u32(&mut self, address: u32, value: u32) -> Result<(), VmError> {
        let idx = self.translate_address(address, 4)?;
        let bytes = value.to_le_bytes();
        self.memory[idx] = bytes[0];
        self.memory[idx + 1] = bytes[1];
        self.memory[idx + 2] = bytes[2];
        self.memory[idx + 3] = bytes[3];
        Ok(())
    }

    fn branch_target(pc: u32, imm: i32) -> u32 {
        pc.wrapping_add(imm as u32)
    }

    fn next_pc(&self) -> u32 {
        self.pc.wrapping_add(4)
    }

    fn write_rd(&mut self, rd: usize, value: u32) {
        if rd != 0 {
            self.regs[rd] = value;
        }
    }

    fn execute(&mut self, inst: Instruction) -> Result<StepOutcome, VmError> {
        match inst {
            Instruction::LUI { rd, imm } => {
                self.write_rd(rd, imm as u32);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::AUIPC { rd, imm } => {
                let val = self.pc.wrapping_add(imm as u32);
                self.write_rd(rd, val);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::JAL { rd, imm } => {
                let ret = self.next_pc();
                let target = Self::branch_target(self.pc, imm);
                self.write_rd(rd, ret);
                self.pc = target;
                Ok(StepOutcome::Continue)
            }
            Instruction::JALR { rd, rs1, imm } => {
                let base = self.regs[rs1];
                let target = base.wrapping_add(imm as u32) & !1;
                let ret = self.next_pc();
                self.write_rd(rd, ret);
                self.pc = target;
                Ok(StepOutcome::Continue)
            }
            Instruction::BEQ { rs1, rs2, imm } => {
                let take = self.regs[rs1] == self.regs[rs2];
                if take {
                    self.pc = Self::branch_target(self.pc, imm);
                } else {
                    self.pc = self.next_pc();
                }
                Ok(StepOutcome::Continue)
            }
            Instruction::BNE { rs1, rs2, imm } => {
                let take = self.regs[rs1] != self.regs[rs2];
                if take {
                    self.pc = Self::branch_target(self.pc, imm);
                } else {
                    self.pc = self.next_pc();
                }
                Ok(StepOutcome::Continue)
            }
            Instruction::BLT { rs1, rs2, imm } => {
                let a = self.regs[rs1] as i32;
                let b = self.regs[rs2] as i32;
                let take = a < b;
                if take {
                    self.pc = Self::branch_target(self.pc, imm);
                } else {
                    self.pc = self.next_pc();
                }
                Ok(StepOutcome::Continue)
            }
            Instruction::BGE { rs1, rs2, imm } => {
                let a = self.regs[rs1] as i32;
                let b = self.regs[rs2] as i32;
                let take = a >= b;
                if take {
                    self.pc = Self::branch_target(self.pc, imm);
                } else {
                    self.pc = self.next_pc();
                }
                Ok(StepOutcome::Continue)
            }
            Instruction::BLTU { rs1, rs2, imm } => {
                let take = self.regs[rs1] < self.regs[rs2];
                if take {
                    self.pc = Self::branch_target(self.pc, imm);
                } else {
                    self.pc = self.next_pc();
                }
                Ok(StepOutcome::Continue)
            }
            Instruction::BGEU { rs1, rs2, imm } => {
                let take = self.regs[rs1] >= self.regs[rs2];
                if take {
                    self.pc = Self::branch_target(self.pc, imm);
                } else {
                    self.pc = self.next_pc();
                }
                Ok(StepOutcome::Continue)
            }
            Instruction::LB { rd, rs1, imm } => {
                let addr = self.regs[rs1].wrapping_add(imm as u32);
                let v = self.load_u8(addr)? as i8 as i32 as u32;
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::LH { rd, rs1, imm } => {
                let addr = self.regs[rs1].wrapping_add(imm as u32);
                let v = self.load_u16(addr)? as i16 as i32 as u32;
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::LW { rd, rs1, imm } => {
                let addr = self.regs[rs1].wrapping_add(imm as u32);
                let v = self.load_u32_mem(addr)?;
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::LBU { rd, rs1, imm } => {
                let addr = self.regs[rs1].wrapping_add(imm as u32);
                let v = self.load_u8(addr)? as u32;
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::LHU { rd, rs1, imm } => {
                let addr = self.regs[rs1].wrapping_add(imm as u32);
                let v = self.load_u16(addr)? as u32;
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SB { rs1, rs2, imm } => {
                let addr = self.regs[rs1].wrapping_add(imm as u32);
                let v = (self.regs[rs2] & 0xff) as u8;
                self.store_u8(addr, v)?;
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SH { rs1, rs2, imm } => {
                let addr = self.regs[rs1].wrapping_add(imm as u32);
                let v = (self.regs[rs2] & 0xffff) as u16;
                self.store_u16(addr, v)?;
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SW { rs1, rs2, imm } => {
                let addr = self.regs[rs1].wrapping_add(imm as u32);
                let v = self.regs[rs2];
                self.store_u32(addr, v)?;
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::ADDI { rd, rs1, imm } => {
                let v = self.regs[rs1].wrapping_add(imm as u32);
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SLTI { rd, rs1, imm } => {
                let a = self.regs[rs1] as i32;
                let b = imm;
                let v = if a < b { 1 } else { 0 };
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SLTIU { rd, rs1, imm } => {
                let a = self.regs[rs1];
                let b = imm as u32;
                let v = if a < b { 1 } else { 0 };
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::XORI { rd, rs1, imm } => {
                let v = self.regs[rs1] ^ (imm as u32);
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::ORI { rd, rs1, imm } => {
                let v = self.regs[rs1] | (imm as u32);
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::ANDI { rd, rs1, imm } => {
                let v = self.regs[rs1] & (imm as u32);
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SLLI { rd, rs1, shamt } => {
                let v = self.regs[rs1] << (shamt & 31);
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SRLI { rd, rs1, shamt } => {
                let v = self.regs[rs1] >> (shamt & 31);
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SRAI { rd, rs1, shamt } => {
                let v = ((self.regs[rs1] as i32) >> (shamt & 31)) as u32;
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::ADD { rd, rs1, rs2 } => {
                let v = self.regs[rs1].wrapping_add(self.regs[rs2]);
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SUB { rd, rs1, rs2 } => {
                let v = self.regs[rs1].wrapping_sub(self.regs[rs2]);
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SLL { rd, rs1, rs2 } => {
                let shamt = self.regs[rs2] & 31;
                let v = self.regs[rs1] << shamt;
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SLT { rd, rs1, rs2 } => {
                let a = self.regs[rs1] as i32;
                let b = self.regs[rs2] as i32;
                let v = if a < b { 1 } else { 0 };
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SLTU { rd, rs1, rs2 } => {
                let a = self.regs[rs1];
                let b = self.regs[rs2];
                let v = if a < b { 1 } else { 0 };
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::XOR { rd, rs1, rs2 } => {
                let v = self.regs[rs1] ^ self.regs[rs2];
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SRL { rd, rs1, rs2 } => {
                let shamt = self.regs[rs2] & 31;
                let v = self.regs[rs1] >> shamt;
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::SRA { rd, rs1, rs2 } => {
                let shamt = (self.regs[rs2] & 31) as u32;
                let v = ((self.regs[rs1] as i32) >> shamt) as u32;
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::OR { rd, rs1, rs2 } => {
                let v = self.regs[rs1] | self.regs[rs2];
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::AND { rd, rs1, rs2 } => {
                let v = self.regs[rs1] & self.regs[rs2];
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::MUL { rd, rs1, rs2 } => {
                let a = self.regs[rs1] as u64;
                let b = self.regs[rs2] as u64;
                let v = a.wrapping_mul(b) as u32;
                self.write_rd(rd, v);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::MULH { rd, rs1, rs2 } => {
                let a = self.regs[rs1] as i64;
                let b = self.regs[rs2] as i64;
                let prod = (a as i128).wrapping_mul(b as i128);
                let hi = (prod >> 32) as u32;
                self.write_rd(rd, hi);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::MULHSU { rd, rs1, rs2 } => {
                let a = self.regs[rs1] as i64;
                let b = self.regs[rs2] as u64 as i64;
                let prod = (a as i128).wrapping_mul(b as i128);
                let hi = (prod >> 32) as u32;
                self.write_rd(rd, hi);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::MULHU { rd, rs1, rs2 } => {
                let a = self.regs[rs1] as u64;
                let b = self.regs[rs2] as u64;
                let prod = (a as u128).wrapping_mul(b as u128);
                let hi = (prod >> 32) as u32;
                self.write_rd(rd, hi);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::DIV { rd, rs1, rs2 } => {
                let dividend = self.regs[rs1] as i32;
                let divisor = self.regs[rs2] as i32;
                let val = if divisor == 0 {
                    -1
                } else if dividend == i32::MIN && divisor == -1 {
                    i32::MIN
                } else {
                    dividend.wrapping_div(divisor)
                };
                self.write_rd(rd, val as u32);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::DIVU { rd, rs1, rs2 } => {
                let dividend = self.regs[rs1];
                let divisor = self.regs[rs2];
                let val = if divisor == 0 { u32::MAX } else { dividend / divisor };
                self.write_rd(rd, val);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::REM { rd, rs1, rs2 } => {
                let dividend = self.regs[rs1] as i32;
                let divisor = self.regs[rs2] as i32;
                let val = if divisor == 0 {
                    dividend
                } else if dividend == i32::MIN && divisor == -1 {
                    0
                } else {
                    dividend.wrapping_rem(divisor)
                };
                self.write_rd(rd, val as u32);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::REMU { rd, rs1, rs2 } => {
                let dividend = self.regs[rs1];
                let divisor = self.regs[rs2];
                let val = if divisor == 0 { dividend } else { dividend % divisor };
                self.write_rd(rd, val);
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::FENCE => {
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::FENCEI => {
                self.pc = self.next_pc();
                Ok(StepOutcome::Continue)
            }
            Instruction::ECALL => {
                self.pc = self.next_pc();
                Ok(StepOutcome::Ecall)
            }
            Instruction::EBREAK => {
                self.pc = self.next_pc();
                Ok(StepOutcome::Breakpoint)
            }
            Instruction::INVALID(word) => Err(VmError::InvalidInstruction(word)),
        }
    }
}
