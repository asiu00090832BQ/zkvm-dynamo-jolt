use ark_ff::PrimeField;
use crate::decoder::{decode, Instruction};
use crate::elf_loader::LoadedProgram;

pub struct Memory { pub base: u32, pub bytes: Vec<u8> }
#[derive(Debug)]
pub enum Trap { IllegalInstruction(u32), LoadAccessFault(u32) }

impl Memory {
    pub fn read_u32(&self, addr: u32) -> Result<u32, Trap> {
        let off = addr.checked_sub(self.base).ok_or(Trap::LoadAccessFault(addr))? as usize;
        if off + 4 > self.bytes.len() { return Err(Trap::LoadAccessFault(addr)); }
        Ok(u32::from_le_bytes([self.bytes[off], self.bytes[off+1], self.bytes[off+2], self.bytes[off+3]]))
    }
}

pub struct Vm<F: PrimeField> { pub regs: [u32; 32], pub pc: u32, pub memory: Memory, _f: core::marker::PhantomData<F> }

impl<F: PrimeField> Vm<F> {
    pub fn new(p: LoadedProgram) -> Self { 
        Self { regs: [0; 32], pc: p.entry, memory: Memory { base: p.base, bytes: p.memory }, _f: Default::default() } 
    }

    pub fn step(&mut self) -> Result<(), Trap> {
        let word = self.memory.read_u32(self.pc)?;
        let inst = decode(word, &Default::default()).map_err(|_| Trap::IllegalInstruction(word))?;

        self.pc += 4;
        self.regs[0] = 0;

        Ok(())
    }
}