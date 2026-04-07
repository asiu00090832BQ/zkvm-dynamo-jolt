public mod decoder;
public mod error;
public mod frontend;
public mod vm;

pub use decoder::{DecodeError, DecodedInstruction, Decoder, DecoderConfig, Opcode};
pub use error::{ZkwmConfig, ZkwmError};
pub use frontend::{ElfLoadError, Frontend, ProgramImage};
pub use vm::{Trap, Vm, VmError};
