use core::fmt;
use rv32im_decoder::ZkvmError;

pub struct ZkwmConfig { pub reset_pc: u32 }

pub struct Zkvm { pub pc: u32, pub regs: [u32; 32] }
impl Zkvm {
    pub fn new() -> Self { Self { pc: 0, regs: [0; 32] } }
    pub fn step(&mut self) -> Result<(), ZkwmError> {
        let inst = rv32im_decoder::decode_word(self.pc)?;
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }
}
