use crate::bits::bit_range;
pub const fn opcode(word: u32) -> u8 { bit_range(word, 0, 7) as u8 }
pub const fn rd(word: u32) -> u8 { bit_range(word, 7, 5) as u8 }
pub const fn funct3(word: u32) -> u8 { bit_range(word, 12, 3) as u8 }
pub const fn rs1(word: u32) -> u8 { bit_range(word, 15, 5) as u8 }
pub const fn rs2(word: u32) -> u8 { bit_range(word, 20, 5) as u8 }
pub const fn funct7(word: u32) -> u8 { bit_range(word, 25, 7) as u8 }
pub const fn csr(word: u32) -> u16 { bit_range(word, 20, 12) as u16 }
pub const fn zimm(word: u32) -> u8 { rs1(word) }
