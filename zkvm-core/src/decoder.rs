#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedInstruction {
    pub raw: u32,
    pub mnemonic: &'static str,
}

pub fn decode_instruction(raw: u32) -> DecodedInstruction {
    let mnemonic = match raw & 0xFF {
        0x73 => "HALT",
        _ => "NOP",
    };
    DecodedInstruction { raw, mnemonic }
}
