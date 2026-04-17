#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecoderError {
    UnsupportedOpcode(u32),
    InvalidFunct3 {
        opcode: u32,
        funct3: u32,
    },
    InvalidFunct7 {
        opcode: u32,
        funct3: u32,
        funct7: u32,
    },
    ReservedInstruction(u32),
}
