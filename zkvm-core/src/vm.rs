use crate::decoder::{Decoder, DecoderConfig, Instruction};
use crate::elf_loader::ElfImage;
pub struct ZkvmConfig { pub mem_size: usize, pub enable_m_extension: bool }
impl Default for ZkvmConfig { fn default() -> Self { Self { mem_size: 1024 * 1024, enable_m_extension: true } } }
pub struct Zkvm { pub config: ZkvmConfig, pub decoder: Decoder, pub pc: u32, pub regs: [u32; 32], pub memory: Vec<u8>, pub base_vaddr: u32, pub halted: bool }
impl Zkvm { pub fn new(config: ZkvmConfig) -> Self { Self { config, decoder: Decoder::new(DecoderConfig::default()), pc: 0, regs: [0; 32], memory: vec![0; config.mem_size], base_vaddr: 0, halted: false } } pub fn load_elf(&mut self, image: &ElfImage) { self.pc = image.entry as u32; } pub fn run(&mut self) -> Result<(), String> { Ok(()) } }