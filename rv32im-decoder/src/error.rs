use core::fmt;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeError { UnsupportedOpcode { opcode: u8, word: u32 }, UnsupportedFunct3 { opcode: u8, funct3: u8, word: u32 }, UnsupportedFunct7 { opcode: u8, funct3: u8, funct7: u8, word: u32 }, MalformedInstruction { reason: &'static str, word: u32 } }
pub use DecodeError as ZkvmError;
pub type ZkvmResult<T> = core::result::Result<T, ZkvmError>; pub type Zkvm<T> = ZkvmResult<T>;
impl fmt::Display for DecodeError { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") } }
#[cfg(feature = "std")] impl std::error::Error for DecodeError {}
