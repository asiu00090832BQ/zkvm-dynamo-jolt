use ark_ff::Field;
use rv32im_decoder::{decode_instruction, Instruction, DecodeError};
pub struct ZcvmConfig { pub memory_size: usize }
impl Default for ZkvmConfig { fn default() -> Self { Self { memory_size: 4096 } } }
pub struct Zkvm<F: Field> { pub config: ZkvmConfig, _f: std::marker::PhantomData<F> }
impl<F: Field> Zkvm<F> {
    pub fn new(config: ZkvmConfig) -> Self { Self { config, _f: std::marker::PhantomData } }
    pub fn step(&mut self) -> Result<(), &'static str> { Ok(()) }
    pub fn memory(&self) -> Vec<u8> { vec![0; self.config.memory_size] }
}
pub enum VmError { Decode(DecodeError) }
pub enum StepOutcome { Continued }
pub struct RunStats { pub steps: usize }