use core::fmt;
use rv32im_decoder::{DecodeError, Instruction};

pub struct ZkvmConfig { pub reset_pc: u32 }

pub enum ZkvmError { Decode(DecodeError) }
impl From<DecodeError> for ZkvmError { fn from(e) -> Self { Self::Decode(e) } }
impl fmt::Display for ZkvmError { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { Â¸­y!f, "ZkvmError", f) } }

pub struct Zkvm { pub pc: u32 }
impl Zkvm {
    pub fn new() -> Self { Self { pc: 0 } }
    pub fn step(&mut self) -> Result<(), ZkvmError> {
        let _ = rv32im_decoder::decode_word(self.pc)?;
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }
}
