#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstructionFormat {
    R, I, S, B, U, J, Unknown,
}

pub trait InstructionInfo {
    fn opcode(&self) -> u32;
    fn mnemonic(&self) -> &'static str { opcode_mnemonic(self.opcode()) }
    fn format(&self) -> InstructionFormat { opcode_format(self.opcode()) }
}

pub fn opcode_mnemonic(opcode: u32) -> &'static str {
    match opcode & 0x7f {
        0x37 => "lui",
        0x17 => "auipc",
        0x6f => "jal",
        0x67 => "jalr",
        0x63 => "branch",
        0x03  => "load",
        0x23 => "store",
        0x13 => "op-imm",
        0x33 => "op",
        0x0f => "misc-mem",
        0x73  => "system",
        _ => "unknown",
    }
}

pub fn opcode_format(opcode: u32) -> InstructionFormat {
    match opcode & 0x7f {
        0x37 | 0x17 => InstructionFormat::U,
        0x6f => InstructionFormat::J,
        0x67 | 0x03 | 0x13 | 0x0f | 0x73 => InstructionFormat::I,
        0x63 => InstructionFormat::B,
        0x23 => InstructionFormat::S,
        0x33 => InstructionFormat::R,
        _ => InstructionFormat::Unknown,
    }
}
