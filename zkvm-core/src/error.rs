use std::{error::Error, fmt};
pub type Result<T, E> = std::result::Result<T, E>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElfLoadError { EmptyInput, ParseError(String), UnsupportedClass(u8), SegmentUnaligned { index: usize, vaddr: u64, required: u64 }, EntryNotInExecutableSegment { entry: u32 } }
impl fmt::Display for ElfLoadError { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:?}", self) } }
impl Error for ElfLoadError {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError { InstructionNot32Bit { raw: u32 }, UnsupportedOpcode { raw: u32, opcode: u8 }, UnsupportedFunct3 { raw: u32, opcode: u8, funct3: u8 }, UnsupportedFunct7 { raw: u32, opcode: u8, funct3: u8, funct7: u8 }, UnsupportedSystem { raw: u32, funct3: u8, imm12: u16 }, UnsupportedExtension { raw: u32, opcode: u8, detail: &'static str }, InvalidRegister { field: &'static str, reg: u8 }, ReservedEncoding { raw: u32, detail: &'static str } }
impl fmt::Display for DecodeError { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:?}", self) } }
impl Error for DecodeError {}