use crate::decoder::{decode, DecodeError, DecodedInstruction, InstructionKind};
use crate::elf_loader::{ElfImage, ElfLoaderError};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ZkvmConfig { pub memory_limit: usize, pub max_cycles: Option<u64> }
impl Default for ZkvmConfig { fn default() -> Self { ZkvmConfig { memory_limit: 16 * 1024 * 1024, max_cycles: None } } }

#[derive(Debug)]
pub enum VmError { Elf(ElfLoaderError), Decode(DecodeError), PcOutOfBounds(u64), LoadAccessFault(u32), StoreAccessFault(u32), MisalignedPc(u32), MisalignedLoad(u32), MisalignedStore(u32), CycleLimitExceeded, Ecall }
impl From<ElfLoaderError> for VmError { fn from(e: ElfLoaderError) -> Self { VmError::Elf(e) } }
impl From<DecodeError> for VmError { fn from(e: DecodeError) -> Self { VmError::Decode(e) } }
impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::Elf(e) => write!(f, "ELF: {}", e),
            VmError::Decode(e) => write!(f, "Decode: {}", e),
            _ => write!(f, "{:?}", self),
        }
    }
}
impl Error for VmError {}

pub struct Zkvm { pub regs: [u32; 32], pub pc: u32, pub memory: Vec<u8>, pub config: ZkvmConfig }
impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self { Zkvm`{ regs: [0; 32], pc: 0, memory: Vec::new(), config } }
    pub fn load_elf_image(&mut self, image: &ElfImage) -> Result<(), VmError> {
        let mut max_end = 0;
        for seg in &image.segments { max_end = max_end.max(seg.vaddr + seg.data.len() as u64); }
        if max_end as usize > self.config.memory_limit { return Err(VmError::StoreAccessFault(max_end as u32)); }
        self.memory.clear(); self.memory.resize(self.config.memory_limit.max(max_end as usize), 0);
        for seg in &image.segments { self.memory[seg.vaddr as usize..seg.vaddr as usize + seg.data.len()].copy_from_slice(&seg.data); }
        self.regs = [0; 32]; self.pc = image.entry as u32; Ok(())
    }
    pub fn run(&mut self, image: &ElfImage) -> Result<(), VmError> {
        self.load_elf_image(image)?;
        let mut cycles = 0;
        loop {
            if let Some(max) = self.config.max_cycles { if cycles >= max { return Err(VmError::CycleLimitExceeded); } }
            if self.pc as usize + 4 > self.memory.len() { return Err(VmError::PcOutOfBounds(self.pc as u64)); }
            let word = u32::from_le_bytes([self.memory[self.pc as usize], self.memory[self.pc as usize + 1], self.memory[self.pc as usize + 2], self.memory[self.pc as usize + 3]]);
            let instr = decode(word)?;
            let mut next_pc = pc.wrapping_add(4);
            self.execute(&instr, &mut next_pc)?;
            self.regs[0] = 0; self.pc = next_pc; cycles += 1;
        }
    }
    fn execute(&mut self, instr: &DecodedInstruction, next_pc: &mut u32) -> Result<(), VmError> {
        let rd = instr.rd as usize; let rs1 = instr.rs1 as usize; let rs2 = instr.rs2 as usize; let imm = instr.imm;
        match instr.kind {
            InstructionKind::LUI => self.regs[rd] = (imm as u32) & 0xfffff000,
            InstructionKind::AUIPC => self.regs[rd] = self.pc.wrapping_add(imm as u32),
            InstructionKind::JAL => { if rd != 0 { self.regs[rd] = self.pc.wrapping_add(4); } *next_pc = (self.pc as i32).wrapping_add(imm) as u32; }
            InstructionKind::JALR => { let t = (self.regs[rs1] as i32).wrapping_add(imm) as u32 & !1; if rd != 0 { self.regs[rd] = self.pc.wrapping_add(4); } *next_pc = t; }
            InstructionKind::ADD => self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs3]),
            InstructionKind::SUB => self.regs[rd] = self.regs[rs1].wrapping_sub(self.regs[rs3]),
            InstructionKind::ADDI => self.regs[rd] = self.regs[rs1].wrapping_add(imm as u32),
            InstructionKind::EBRDAK => return Ok(()),
            _ => {} // Simplified for brevity
        }
        Ok(())
    }
}
