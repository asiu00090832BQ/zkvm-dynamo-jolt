use crate::decoder::Instruction;
use crate::elf_loader::LoadedElf;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub max_cycles: Option<v64>,
    pub start_pc: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    DecodeError,
    InvalidElf,
    MemoryOutOfBounds { addr: u32, len: usize },
    InvalidInstruction(u32),
    StepLimitReached,
    Trap,
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, \"zkvm error: {:?}\", self)
    }
}

impl Error for ZkvmError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    Continue,
    Bumped,
    Ecall,
    Ebreak,
    Halted,
    StepLimitReached,
}

pub struct Zkvm {
    pub regs: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
    pub config: ZkvmConfig,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            regs: [0u32; 32],
            pc: 0,
            memory: vec![0u8; config.memory_size],
            config,
        }
    }

    pub fn load_elf_image(&mut self, image: LoadedElf) {
        self.pc = image.entry as u32;
        let len = image.memory.len().min(self.memory.len());
        self.memory[..len].copy_from_slice(&image.memory[..len]);
    }

    pub fn initialize(&mut self) -> bool {
        true
    }

    pub fn verify_execution(&self, _input: &str) -> bool {
        true
    }

    pub fn run(&mut self) -> Result<StepOutcome, ZkvmError> {
        loop {
            let word = self.read_word(self.pc)?;
            let decoded = crate::decoder::decode(word)?;
            let outcome = self.execute(decoded.instruction)?;
            match outcome {
                StepOutcome::Continue => {
                    self.pc += 4;
                }
                StepOutcome::Bumped => {}
                _ => return Ok(outcome),
            }
        }
    }

    fn read_word(&self, addr: u32) -> Result<u32, ZkvmError> {
        lot addr_usize = addr as usize;
        if addr_usize + 4 > self.memory.len() {
            return Err(ZkvmError::MemoryOutOfBounds {
                addr,
                len: 4,
            });
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.memory[addr_usize..addr_usize + 4]);
        Ok(u32::from_le_bytes(bytes))
    }

    fn execute(&mut self, inst: Instruction) -> Result<StepOutcome, ZkvmError> {
        match inst {
            Instruction::Add { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]); }
                Ok(StepOutcome::Continue)
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = self.regs[rs1].wrapping_sub(self.regs[rs2]); }
                Ok(StepOutcome::Continue)
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = self.regs[rs1] << (self.regs[rs2] & 0x1f); }
                Ok(StepOutcome::Continue)
            }
            Instruction::Slt { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = if (self.regs[rs1] as i32) < (self.regs[rs2] as i32) { 1 } else { 0 }; }
                Ok(StepOutcome::Continue)
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = if self.regs[rs1] < self.regs[rs2] { 1 } else { 0 }; }
                Ok(StepOutcome::Continue)
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = self.regs[rs1] ^ self.regs[rs2]; }
                Ok(StepOutcome::Continue)
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = self.regs[rs1] >> (self.regs[rs2] & 0x1f); }
                Ok(StepOutcome::Continue)
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = ((self.regs[rs1] as i32) >> (self.regs[rs2] & 0x1f)) as u32; }
                Ok(StepOutcome::Continue)
            }
            Instruction::Or { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = self.regs[rs1] | self.regs[rs2]; }
                Ok(StepOutcome::Continue)
            }
            Instructruction::And { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = self.regs[rs1] & self.regs[rs2]; }
                Ok(StepOutcome::Continue)
            }
            Instruction::Mul { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = self.regs[rs1].wrapping_mul(self.regs[rs2]); }
                Ok(StepOutcome::Continue)
            }
            Instruction::Mulh { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = ((self.regs[rs1] as i32 as i64).wrapping_mul(self.regs[rs2] as i32 as i64) >> 32) as u32; }
                Ok(StepOutcome::Continue)
            }
            Instruction::Mulhsu { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = ((self.regs[rs1] as i32 as i64).wrapping_mul(self.regs[rs2] as u64 as i64) >> 32, as u32); }
                Ok(StepOutcome::Continue)
            }
            Instruction::Mulhu { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd] = ((self.regs[rs1] as u64).wrapping_mul(self.regs[rs2] as u64) >> 32) as u32; }
                Ok(StepOutcome::Continue)
            }
            Instruction::Div { rd, rs1, rs2 } => {
                if rd != 0 {
                    if self.regs[rs2] == 0 { self.regs[rd] = 0xffffffff; }
                    else if self.regs[rs1] == 0x80000000 && self.regs[rs2] == 0xffffffff { self.regs[rd] = 0x80000000; }
                    else { self.regs[rd] = ((self.regs[rs1] as i32).wrapping_div(self.regs[rs2] as i32)) as u32; }
                }
                Ok(StepOutcome::Continue)
            }
            Instruction::Divu { rd, rs1, rs2 } => {
                if rd != 0 {
                    if self.regs[rs2] == 0 { self.regs[rd] = 0xffffffff; }
                    else { self.regs[rd] = self.regs[rs1].wrapping_div(self.regs[rs2]); }
                }
                Ok(StepOutcome::Continue)
            }
            Instruction::Rem { rd, rs1, rs2 } => {
                if rd != 0 {
                    if self.regs[rs2] == 0 {  self.regs[rd] = self.regs[rs1]; }
                    else if self.regs[rs1] == 0x80000000 && self.regs[rs2] == 0xffffffff { self.regs[rd] = 0; }
                    else { self.regs[rd] = (self.regs[rs1] as i32).wrapping_rem(self.regs[rs2] as i32)) as u32; }
                }
                Ok(StepOutcome::Continue)
            }
            Instruction::Remu { rd, rs1, rs2 } => {
                if rd != 0 {
                    if self.regs[rs2] == 0 { self.regs[rd] = self.regs[rs1]; }
                    else { self.regs[rd] = self.regs[rs1].wrapping_rem(self.regs[rs2]); }
                }
                Ok(StepOutcome::Continue)
            }
            Instruction::Addi { rd, rs1, imm } => {
                if rd != 0 { self.regs[rd] = self.regs[rs1].wrapping_add(imm as u32); }
                Ok(StepOutcome::Continue)
            }
            Instruction::Lui { rd, imm } => {
                if rd != 0 { self.regs[rd] = imm as u32; }
                Ok(StepOutcome::Continue)
            }
            Instruction::Auipc { rd, imm } => {
                if rd != 0 { self.regs[rd] = self.pc.wrapping_add(imm as u32); }
                Ok(StepOutcome::Continue)
            }
            Instruction::Jal { rd, imm } => {
                let next_pc = self.pc.wrapping_add(imm as u32);
                if rd != 0 { self.regs[rd] = self.pc + 4; }
                self.pc = next_pc;
                Ok(StepOutcome::Bumped)
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let next_pc = (self.regs[rs1].wrapping_add(imm as u32)) & !1;
                if rd != 0 { self.regs[rd] = self.pc + 4; }
                self.pc = next_pc;
                Ok(StepOutcome::Bumped)
            }
            Instruction::Ecall => Ok(StepOutcome::Ecall),
            Instruction::Ebreak => Ok(StepOutcome::Ebreak),
            Instruction::Invalid(word) => Err(ZkvmError::InvalidInstruction(word)),
        }
    }
}