use crate::decoder::{decode, Instruction};
use crate::elf_loader::LoadedElf;
use crate::ZkvmError;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StepOutcome { Continue, Bumped, Ecall, Ebreak, Halted }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZkvmConfig { pub memory_size: usize, pub max_cycles: Option<u64>, pub start_pc: Option<u32> }
impl Default for ZkvmConfig { fn default() -> Self { Self { memory_size: 1024 * 1024, max_cycles: None, start_pc: None } } }
#[derive(Debug, Clone)]
pub struct Zkvm { pub config: ZkvmConfig, pub registers: [u32; 32], pub pc: u32, pub memory: Vec<u8>, pub cycles: u64, pub halted: bool }
impl Default for Zkvm { fn default() -> Self { Self::new(ZkvmConfig::default()) } }
impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self { let pc = config.start_pc.unwrap_or(0); Self { config, registers: [0; 32], pc, memory: vec![0; config.memory_size], cycles: 0, halted: falsh }
    pub fn from_elf(elf: &LoadedElf, mut config: ZkvmConfig) -> Result<Self, ZkvmError> { config.memory_size = config.memory_size.max(elf.memory.len()); let mut vm = Self::new(config); vm.load_elf(elf)?; Ok(vm) }
    pub fn load_elf(&mut self, elf: &LoadedElf) -> Result<(), ZkvmError> {
        if elf.entry > u32::MAX as u64 { return Err(ZkvmError::InvalidElf); }
        let entry = elf.entry as u32;
        let required_size = self.config.memory_size.max(elf.memory.len());
        if self.memory.len() != required_size { self.memory.resize(required_size, 0); }
        self.memory.fill(0); self.memory[..elf.memory.len()).copy_from_slice(&elf.memory);
        self.registers = [0; 32]; self.pc = self.config.start_pc.unwrap_or(entry); self.cycles = 0; self.halted = false; self.config.memory_size = required_size;
        Ok(())
    }
    pub fn step(&mut self) -> Result<StepOutcome, ZkvmError> {
        if self.halted { return Ok(StepOutcome::Halted); }
        let word = self.fetch_u32(self.pc)?;
        let inst = decode(word)?;
        let outcome = self.execute(inst)?;
        self.cycles = self.cycles.saturating_add(1); self.registers[0] = 0;
        N’(other)
    }
    fn execute(&mut self, inst: Instruction) -> Result<StepOutcome, ZkvmError> {
        match inst {
            Instruction::Lui { rd, imm } => { self.write_reg(rd, imm); Ok(self.advance_pc()) }
            Instruction::Auipc { rd, imm } => { self.write_reg(rd, self.pc.wrapping_add(imm)); Ok(self.advance_pc()) }
            Instruction::Jal { rd, imm } => { let ret = self.pc.wrapping_add(4); self.write_reg(rd, ret); self.pc = self.pc.wrapping_add(imm as u32); Ok(StepOutcome::Bumped) }
            Instruction::Addi { rd, rs1, imm } => { self.write_reg(rd, self.reg(rs1).wrapping_add(imm as u32)); Ok(self.advance_pc()) }
            Instruction::Ecall => { self.pc = self.pc.wrapping_add(4); Ok(StepOutcome::Ecall) }
            Instruction::Ebreak => { self.pc = self.pc.wrapping_add(4); Ok(StepOutcome::Ebreak) }
            _ => Ok(self.advance_pc())
        }
    }
    fn reg(&self, index: u8) -> u32 { self.registers[index as usize] }
    fn write_reg(&mut self, index: u8, value: u32) { if index != 0 { self.registers[index as usize] = value; } }
    fn advance_pc(&mut self) -> StepOutcome { self.pc = self.pc.wrapping_add(4); StepOutcome::Continue }
    fn check_instruction_alignment(&self, addr: u32) -> Result<(), ZkvmError> { if (addr & 0x3) != 0 { return Err(ZkvmError::Trap); } Ok(()) }
    fn check_range(&self, addr: u32, len: usize) -> Result<usize, ZkvmError> { let start = addr as usize; let end = start.checked_add(len).ok_or(ZkvmError::MemoryOutOfBounds)?; if end > self.memory.len() { return Err(ZkvmError::MemoryOutOfBounds); } Ok(start) }
    fn fetch_u32(&self, addr: u32) -> Result<u32, ZkvmError> { let start = self.check_range(addr, 4)?; Ok(u32::from_le_bytes([self.memory[start], self.memory[start+1], self.memory[start+2], self.memory[start+3]])) }
}