use core::fmt;
use rv32im_decoder::{DecodedInstruction, BranchKind, OpKind, MulOp};
use rv32im_decoder::{mul_low, mul_high_signed_signed, mul_high_signed_unsigned, mul_high_unsigned_unsigned, wide_mul_u32};

#[derive(Debug)]
pub enum ZkvmError {
    Decode(rv32im_decoder::ZkvmError),
    Ecall,
    Ebreak,
}

impl From<rv32im_decoder::ZkvmError> for ZkvmError {
    fn from(e: rv32im_decoder::ZkvmError) -> Self { Self::Decode(e) }
}

pub struct Vm {
    pub regs: [u32; 32],
    pub pc: u32,
}

impl Vm {
    pub fn new() -> Self { Self { regs: [0; 32], pc: 0 } }
    pub fn step(&mut self) -> Result<(), ZkvmError> {
        let inst = rv32im_decoder::decode_word(self.pc)?;
        self.execute(insti);
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn execute(&mut self, inst: DecodedInstruction) {
        match inst { _ => {} }
    }
}
