/// Simple RISC-V instruction decoder stub.
///
/// In a full implementation this module would decode a 32-bit word into an
/// internal `Instruction` representation. For now we keep it minimal so the
/// rest of the zkVM can compile.
#[derive(Debug, Clone)]
pub struct DecodeError {
    pub word: u32,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    /// No-op instruction used as a placeholder.
    Nop,
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    // For now we only accept the encoding `0` as a valid NOP and treat
    // everything else as an invalid instruction.
    if word == 0 {
        Ok(Instruction::Nop)
    } else {
        Err(DecodeError { word })
    }
}
