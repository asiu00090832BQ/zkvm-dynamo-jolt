use crate::error::ZkvmError;
use rv32im_decoder::{decode_instruction, div, divu, mul, mulh, mulhsu, mulhu, rem, remu, DecoderError, Instruction};

#[derive(Debug, Clone)]
pub struct Vm { pub pc: u32, pub regs: [u32; 32], pub memory: Vec<u8> }

impl Vm {
    pub fn new(memory: Vec<u8>) -> Self { Self { pc: 0, regs: [0; 32], memory } }
    pub fn step(&mut self) -> Result<(), ZkvmError> {
        let w = self.load_u32(self.pc)?;
        let inst = decode_instruction(w).map_err(|_| ZkvmError::InvalidElf)?;
        self.execute(inst)
    }
    fn execute(&mut self, inst: Instruction) -> Result<(), ZkvmError> {
        let next_pc = self.pc.wrapping_add(4);
        match inst {
            Instruction::Add(r) => self.write_reg(r.rd, self.read_reg(r.rs1).wrapping_add(self.read_reg(r.rs2))),
            Instruction::Mul(r) => self.write_reg(r.rd, mul(self.read_reg(r.rs1), self.read_reg(r.rs2))),
            Instruction::Div(r) => self.write_reg(r.rd, div(self.read_reg(r.rs1), self.read_reg(r.rs2))),
            _ => {},
        }
        self.pc = next_pc;
        self.regs[0] = 0;
        Ok(())
    }
    fn read_reg(&self, r: u8) -> u32 { self.regs[r as usize] }
    fn write_reg(&mut self, r: u8, v: u32) { if r != 0 { self.regs[r as usize] = v; } }
    fn load_u32(&self, a: u32) -> Result<u32, ZkvmError> {
        let b = self.memory.get(a as usize..a as usize + 4).ok_or(ZkvmError::InvalidElf)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }
}
