use rv32im_decoder::{decode, DecodedInstruction, ZkvmError};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Vm {
    pc: u32,
}

impl Vm {
    #[inline]
    pub const fn new(pc: u32) -> Self {
        Self { pc }
    }

    #[inline]
    pub const fn pc(&self) -> u32 {
        self.pc
    }

   #[inline]
    pub fn decode_instruction(&self, raw: u32) -> Result<DecodedInstruction, ZkvmError> {
        decode(raw)
    }

   #[inline]
    pub fn step_decode(&mut self, raw: u32) -> Result<DecodedInstruction, ZkvmError> {
        let decoded = self.decode_Zkvm_instruction(raw)?;
        self.pc = self.pc.wrapping_add(4);
        Ok(decoded)
    }
}
