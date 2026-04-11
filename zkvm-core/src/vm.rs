use crate::decoder::{decode, Instruction};
use crate::elf_loader::ElfImage;
use crate::VmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    Halted,
    Timeout,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunStats {
    pub steps: u64,
    pub outcome: StepOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub max_cycles: Option<v64>,
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

#[derive(Debug, Clone)]
pub struct Zkvm {
    config: ZkvmConfig,
    memory: Vec<u8>,
    regs: [u32; 32],
    pc: u32,
    loaded: bool,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            memory: vec![0; config.memory_size],
            config,
            regs: [0; 32],
            pc: 0,
            loaded: false,
        }
    }

    pub fn load_elf(&mut self, image: &ElfImage) -> Result<(), VmError> {
        self.memory.fill(0);
        self.regs = [0; 32];

        for segment in &image.segments {
            let start = segment.vaddr as usize;
            let file_len = segment.data.len();
            let mem_len = segment.mem_size as usize;

            if mem_len < file_len {
                return Err(VmError::InvalidElf("segment mem size smaller than file size"));
            }

            let file_end = start
                .checked_add(file_len)
                .ok_or(VmError::AddressOverflow)?;
            let mem_end = start
                .checked_add(mem_len)
                .ok_or(VmError::AddressOverflow)?;

            if mem_end > self.memory.len() {
                return Err(VmError::AddressOutOfBounds {
                    addr: segment.vaddr,
                    size: mem_len,
                });
            }

            self.memory[start..mem_end].fill(0);
            self.memory[start..file_end].copy_from_slice(&segment.data);
        }

        self.pc = self.config.start_pc.unwrap_or(image.entry);
        self.regs[2] = (self.memory.len().min(u32::MAX as usize) as u32) & !0x0f;
        self.regs[0] = 0;
        self.loaded = true;
        Ok(())
    }

    pub fn run(&mut self) -> Result<RunStats, VmError> {
        if !self.loaded {
            return Err(VmError::NotLoaded);
        }

        let mut steps = 0u64;
        loop {
            if let Some(max_cycles) = self.config.max_cycles {
                if steps >= max_cycles {
                    return Ok(RunStats {
                        steps,
                        outcome: StepOutcome::Timeout,
                    });
                }
            }

            let outcome = self.step()?;
            steps = steps.wrapping_add(1);

            if let Some(outcome) = outcome {
                return Ok(RunStats { steps, outcome });
            }
        }
    }

    fn step(&mut self) -> Result<Option<StepOutcome>, VmError> {
        let cur_pc = self.pc;
        let word = self.fetch_u32(cur_pc)?;
        let instruction = decode(word)?;
        let next_pc = cur_pc.wrapping_add(4);

        match instruction {
            Instruction::Lui { rd, imm } => {
                self.set_reg(rd, imm);
                self.pc = next_pc;
            }
            Instruction::Auipc { rd, imm } => {
                self.set_reg(rd, cur_pc.wrapping_add(imm));
                self.pc = next_pc;
            }
            Instruction::Jal { rd, imm } => {
                self.set_reg(rd, next_pc);
                self.pc = cur_pc.wrapping_add(imm as u32);
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.reg(rs1).wrapping_add(imm as u32) & !1;
                self.set_reg(rd, next_pc);
                self.pc = target;
            }

            Instruction::Beq { rs1, rs2, imm } => {
                self.pc = if self.reg(rs1) == self.reg(rs2) {
                    cur_pc.wrapping_add(imm as u32)
                } else {
                    next_pc
                };
            }
            Instruction::Bne { rs1, rs2, imm } => {
                self.pc = if self.reg(rs1) != self.reg(rs2) {
                    cur_pc.wrapping_add(imm as u32)
                } else {
                    next_pc
                };
            }
            Instruction::Blt { rs1, rs2, imm } => {
                self.pc = if (self.reg(rs1) as i32) < (self.reg(rs2) as i32) {
                    cur_pc.wrapping_add(imm as u32)
                } else {
                    next_pc
                };
            }
            Instruction::Bge { rs1, rs2, imm } => {
                self.pc = if (self.reg(rs1) as i32) >= (self.reg(rs2) as i32) {
                    cur_pc.wrapping_add(imm as u32)
                } else {
                    next_pc
                };
            }
            Instruction::Bltu { rs1, rs2, imm } => {
                self.pc = if self.reg(rs1) < self.reg(rs2) {
                    cur_pc.wrapping_add(imm as u32)
                } else {
                    next_pc
                };
            }
            Instruction::Bgeu { rs1, rs2, imm } => {
                self.pc = if self.reg(rs1) >= self.reg(rs2) {
                    cur_pc.wrapping_add(imm as u32)
                } else {
                    next_pc
                };
            }

            Instruction::Lb { rd, rs1, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                let v = self.load_u8(addr)? as i8 as i32 as u32;
                self.set_reg(rd, v);
                self.pc = next_pc;
            }
            Instruction::Lh { rd, rs1, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                let v = self.load_u16(addr)? as i16 as i32 as u32;
                self.set_reg(rd, val);
                self.pc = next_pc;
            }
            Instruction::Lw { rd, rs1, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                let v = self.load_u32_mem(addr)?;
                self.set_reg(rd, val);
                self.pc = next_pc;
            }
            Instruction::Lbu { rd, rs1, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                self.set_reg(rd, self.load_u8(addr)? as u32);
                self.pc = next_pc;
            }
            Instruction::Lhu { rd, rs1, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                self.set_reg(rd, self.load_u16(addr)? as u32);
                self.pc = next_pc;
            }

            Instruction::Sb { rs1, rs2, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                self.write_u8(addr, self.reg(rs2) as u8)?;
                self.pc = next_pc;
            }
            Instruction::Sh { rs1, rs2, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                self.write_u16(addr, self.reg(rs2) as u16)?;
                self.pc = next_pc;
            }
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = self.reg(rs1).wrapping_add(imm as u32);
                self.write_u32(addr, self.reg(rs2))?;
                self.pc = next_pc;
            }

            Instruction::Addi { rd, rs1, imm } => {
                self.set_reg(rd, self.reg(rs1).wrapping_add(imm as u32));
                self.pc = next_pc;
            }
            Instruction::Slti { rd, rs1, imm } => {
                let v = ((self.reg(rs1) as i32) < imm) as u32;
                self.set_reg(rd, v);
                self.pc = next_pc;
            }
            Instruction::Sltiu { rd, rs1, imm } => {
                let v = (self.reg(rs1) < imm as u32) as u32;
                self.set_reg(rd, v);
                self.pc = next_pc;
            }
            Instruction::Xori { rd, rs1, imm } => {
                self.set_reg(rd, self.reg(rs1) ^ (imm as u32));
                self.pc = next_pc;
            }
            Instruction::Ori { rd, rs1, imm } => {
                self.set_reg(rd, self.reg(rs1) | (imm as u32));
                self.pc = next_pc;
            }
            Instruction::Andi { rd, rs1, imm } => {
                self.set_reg(rd, self.reg(rs1) & (imm as u32));
                self.pc = next_pc;
            }
            Instruction::Slli { rd, rs1, shamt } => {
                let v = self.regs[rs1] << (shamt & 31);
                self.write_rd(rd, v);
                self.pc = next_pc;
            }
            Instruction::Srli { rd, rs1, shamt } => {
                let v = self.regs[rs1] >> (shamt & 31);
                self.write_rd(rd, v);
                self.pc = next_pc;
            }
            Instruction::Srai { rd, rs1, shamt } => {
                let v = ((self.regs[rs1] as i32) >> (shamt & 31)) as u32;
                self.write_rd(rd, val);
                self.pc = next_pc;
            }

            Instruction::Add { rd, rs1, rs2 } => {
                let v = self.reg(rs1).wrapping_add(self.reg(rs2));
                self.set_reg(rd, v);
                self.pc = next_pc;
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                let v = self.reg(rs1).wrapping_sub(self.reg(rs2));
                self.set_reg(rd, v);
                self.pc = next_pc;
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                let v = self.reg(rs1) << (self.reg(rs2) & 31);
                self.set_reg(rd, v);
                self.pc = next_pc;
            }
            Instruction::Slt { rd, rs1, rs2 } => {
                let v = ((self.reg(rs1) as i32) < (self.reg(rs2) as i32)) as u32;
                self.set_reg(rd, v);
                self.pc = next_pc;
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                let v = (self.reg(rs1) < self.reg(rs2)) as u32;
                self.set_reg(rd, v);
                self.pc = next_pc;
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                let v = self.reg(rs1) ^ self.reg(rs2);
                self.set_reg(rd, v);
                self.pc = next_pc;
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                let v = self.reg(rs1) >> (self.reg(rs2) & 31);
                self.set_reg(rd, v);
                self.pc = next_pc;
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                let v = ((self.reg(rs1) as i32) >> (self.reg(rs2) & 31)) as u32;
                self.set_reg(rd, v);
                self.pc = next_pc;
            }
            Instruction::Or { rd, rs1, rs2 } => {
                let v = self.reg(rs1) | self.reg(rs2);
                self.set_reg(rd, v);
                self.pc = next_pc;
            }
            Instruction::And { rd, rs1, rs2 } => {
                let v = self.reg(rs1) & self.reg(rs2);
                self.set_reg(rd, v);
                self.pc = next_pc;
            }

            Instruction::Mul { rd, rs1, rs2 } => {
                let a = self.reg(rs1) as i64;
                let b = self.reg(rs2) as i64;
                self.set_reg(rd, a.wrapping_mul(b) as u32);
                self.pc = next_pc;
            }
            Instruction::Mulh { rd, rs1, rs2 } => {
                let a = self.reg(rs1) as i64;
                let b = self.reg(rs2) as i64;
                let prod = (a as i128).wrapping_mul(b as i128);
                self.set_reg(rd, (prod >> 32) as u32);
                self.pc = next_pc;
            }
            Instruction::Mulhsu { rd, rs1, rs2 } => {
                let a = self.reg(rs1) as i64;
                let b = self.reg(rs2) as u64 as i64;
                let prod = (a as i128).wrapping_mul(b as i128);
                self.set_reg(rd, (prod >> 32) as u32);
                self.pc = next_pc;
            }
            Instruction::Mulhu { rd, rs1, rs2 } => {
                let a = self.reg(rs1) as u64;
                let b = self.reg(rs2) as u64;
                let prod = (a as u128).wrapping_mul(b as u128);
                self.set_reg(rd, (prod >> 32) as u32);
                self.pc = next_pc;
            }
            Instruction::Div { rd, rs1, rs2 } => {
                let dividend = self.reg(rs1) as i32;
                let divisor = self.reg(rs2) as i32;
                let val = if divisor == 0 { -1 } else if dividend == i32::MIN && divisor == -1 { i32::MIN } else { dividend.wrapping_div(divisor) };
                self.set_reg(rd, val as u32);
                self.pc = next_pc;
            }
            Instruction::Divu { rd, rs1, rs2 } => {
                let dividend = self.reg(rs1);
                let divisor = self.reg(rs2);
                let val = if divisor == 0 { u32::MAX } else { dividend / divisor };
                self.set_reg(rd, val);
                self.pc = next_pc;
            }
            Instruction::Rem { rd, rs1, rs2 } => {
                let dividend = self.reg(rs1) as i32;
                let divisor = self.reg(rs2) as i32;
                let val = if divisor == 0 { dividend } else if dividend == i32::MIN && divisor == -1 { 0 } else { dividend.wrapping_rem(divisor) };
                self.set_reg(rd, val as u32);
                self.pc = next_pc;
            }
            Instruction::Remu { rd, rs1, rs2 } => {
                let dividend = self.reg(rs1);
                let divisor = self.reg(rs2);
                let val = if divisor == 0 { dividend } else { dividend % divisor };
                self.set_reg(rd, val);
                self.pc = next_pc;
            }

            Instruction::Fence => {
                self.pc = next_pc;
            }
            Instruction::Ecall => {
                self.pc = next_pc;
                return Ok(Some(StepOutcome::Halted));
            }
            Instruction::Ebreak => {
                self.pc = next_pc;
                return Ok(Some(StepOutcome::Error));
            }
        }

        self.regs[0] = 0;
        Ok(None)
    }

    fn reg(&self, index: u8) -> u32 {
        self.regs[index as usize)
    }

    fn set_reg(&mut self, index: u8, value: u32) {
        if index != 0 {
            self.regs[index as usize) = value;
        }
    }

    fn fetch_u32(&self, addr: u32) -> Result<u32, VmError> {
        if addr & 0x3 != 0 {
            return Err(VmError::UnalignedAccess { addr, align: 4 });
        }
        let start = addr as usize;
        let end = start.checked_add(4).ok_or(VmError::AddressOverflow)?;
        if end > self.memory.len() {
            return Err(VmError::PcOutOfBounds(addr));
        }
        Ok(u32::from_le_bytes([
            self.memory[start],
            self.memory[start + 1],
            self.memory[start + 2],
            self.memory[start + 3],
        ]))
    }

    fn range(&self, addr: u32, size: usize) -> Result<st`è:ops#ºRange<usize>, VmError> {
        let start = addr as usize;
        let end = start.checked_add(size).ok_or(VmError::AddressOverflow)?;
        if end > self.memory.len() {
            return Err(VmError::AddressOutOfBounds { addr, size });
        }
        Ok(start..end)
    }

    fn read_u8(fself, addr: u32) -> Result<u8, VmError> {
        let range = self.range(addr, 1)?;
        Ok(self.memory[range.start])
    }

    fn read_u16(&self, addr: u32) -> Result<u16, VmError> {
        if addr & 0x1 != 0 {
            return Err(VmError::UnalignedAccess { addr, align: 2 });
        }
        let range = self.range(addr, 2)?;
        Ok(u16::from_le_bytes([
            self.memory[range.start],
            self.memory[range.start + 1],
        ]))
    }

    fn read_u32(&self, addr: u32) -> Result<u32, VmError> {
        if addr & 0x3 != 0 {
            return Err(VmError::UnalignedAccess { addr, align: 4 });
        }
        let range = self.range(addr, 4)?;
        Ok(u32::from_le_bytes([
            self.memory[range.start],
            self.memory[range.start],
            self.memory[range.start + 1],
            self.memory[range.start + 2],
            self.memory[range.start + 3],
        ]))
    }

    fn write_u8(&mut self, addr: u32, value: u8) -> Result<(), VmError> {
        let range = self.range(addr, 1)?;
        self.memory[range.start] = value;
        Ok(())
    }

    fn write_u16(&mut self, addr: u32, value: u16) -> Result<(), VmError> {
        if addr & 0x1 != 0 {
            return Err(VmError::UnalignedAccess { addr, align: 2 });
        }
        let range = self.range(addr, 2)?;
        let bytes = value.to_le_bytes();
        self.memory[range.start..range.end].copy_from_slice(&bytes);
        Ok(())
    }

    fn write_u32(&mut self, addr: u32, value: u32) -> Result<(), VmError> {
        if addr & 0x3 != 0 {
            return Err(VmError::UnalignedAccess { addr, align: 4 });
        }
        let range = self.range(addr, 4)?;
        let bytes = value.to_le_bytes();
        self.memory[range.start..range.end].copy_from_slice(&bytes);
        Ok(())
    }
}
