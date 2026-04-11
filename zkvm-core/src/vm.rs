use crate::decoder::{decode, Instruction};
use crate::elf_loader::LoadedElf;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZkvmConfig { pub memory_size: usize, pub max_cycles: Option<u64>, pub start_pc: Option<u32>, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError { DecodeError, InvalidElf, MemoryOutOfBounds { addr: u32, len: usize }, InvalidInstruction(u32), StepLimitReached, Trap, }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome { Continue, Ecall, Ebreak, Halted, StepLimitReached, }
pub struct Zkvm { pub regs: [u32; 32], pub pc: u32, pub memory: Vec<u8>, pub config: ZkvmConfig, }
impl Zkvm { pub fn new(config: ZkvmConfig) -> Self { Self { regs: [0u32; 32], pc: 0, memory: vec![0u8; config.memory_size], config, } } }
