use crate::decoder::{decode, div_i32, divu_u32, mul_low_u32, mulh_i32, mulhsu_i32_u32, mulhu_u32, rem_i32, remu_u32, DecodeError, Instruction};

#[derive(Debug)]
pub enum VmError { Decode(DecodeError), MemoryOutOfBounds { addr: u32, size: usize } }
impl From<DecodeError> for VmError { fn from(value: DecodeError) -> Self { Self::Decode(value) } }

pub struct Vm { pub pc: u32, pub regs: [u32; 32], pub halted: bool }
impl Vm {
    pub fn step(&mut self, word: u32) -> Result<(), VmError> {
        let instr = decode(word)?;
        match instr {
            Instruction::Mul { rd, rs1, rs2 } => { self.regs[rd as usize] = mul_low_u32(self.regs[rs1 as usize], self.regs[rs2 as usize]); }
            _ => { /* ... other instrs ... */ }
        }
        self.pc = self.pc.wrapping_add(4);
        self.regs[0] = 0;
        Ok(())
    }
}
