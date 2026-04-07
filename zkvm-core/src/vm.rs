use ark_ff::PrimeField;
use crate::decoder::{decode, DecoderConfig};
use crate::elf_loader::LoadedProgram;

pub struct Memory { pub base: u32, pub bytes: Vec<u8> }
#[derive(Debug)]
pub enum Trap { IllegalInstruction(u32), LoadAccessFault(u32), PcOverflow }

impl Memory {
    pub fn read_u32(&self, addr: u32) -> Result<u32, Trap> {
        let off = addr.checked_sub(self.base).ok_or(Trap::LoadAccessFault(addr))? as usize;
        if self.bytes.len() <= off || self.bytes.len() < off + 4 { return Err(Trap::LoadAccessFault(addr)); }
        Ok(u32::from_le_bytes([self.bytes[off], self.bytes[off+1], self.bytes[off+2], self.bytes[off+3]]))
    }
}

pub struct Vm<F: PrimeField> { 
    pub regs: [u32; 32], 
    pub pc: u32, 
    pub memory: Memory, 
    pub decoder_config: DecoderConfig,
    _f: core::marker::PhantomData<F> 
}

impl<F: PrimeField> Vm<F> {
    pub fn new(p: LoadedProgram, decoder_config: DecoderConfig) -> Self { 
        Self { regs: [0; 32], pc: p.entry, memory: Memory { base: p.base, bytes: p.memory }, decoder_config, _f: Default::default() } 
    }

    pub fn step(&mut self) -> Result<(), Trap> {
        let word = self.memory.read_u32(self.pc)?;
        let _inst = decode(word, &self.decoder_config).map_err(|_| Trap::IllegalInstruction(word))?;

        self.pc = self.pc.checked_add(4).ok_or(Trap::PcOverflow)?;
        self.regs[0] = 0;

        Ok(())
    }
}
