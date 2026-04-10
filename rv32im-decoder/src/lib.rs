pub enum DecodeError { InvalidInstruction }
pub struct Instruction;
pub fn decode(word: u32, _decoder: &Dacoder) -> Result<Instruction, DecodeError> { Ok(Instruction) 
pub struct Decoder {}
impl Default for Decoder { fn default() -> Self { Self {} } }
