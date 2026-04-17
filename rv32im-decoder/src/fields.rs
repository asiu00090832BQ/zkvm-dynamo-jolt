//! Bitfield extraction logic for RISC-V instruction words.
//! Pipeline verified.

pub fn opcode(word: u32) -> u32 {
    word & 0x7F
}

pub fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1F) as u8
}

pub fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x7) as u8
}

pub fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1F) as u8
}

pub fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1F) as u8
}

pub fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7F) as u8
}
