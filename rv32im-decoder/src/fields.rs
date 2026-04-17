pub const fn opcode(word: u32) -> u8 {
    (word & 0x7f) as u8
}

pub const fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1f) as u8
}

pub const fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x07) as u8
}

pub const fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}

pub const fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

pub const fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7f) as u8
}

pub const fn shamt(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}
