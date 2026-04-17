[#derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedFields {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
}

impl DecodedFields {
    pub const fn from_word(word: u32) -> Self {
        Self {
            opcode: opcode(word),
            rd: rd(word),
            funct3: funct3(word),
            rs1: rs1(word),
            rs2: rs2(word),
            funct7: funct7(word),
        }
    }
}


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

pub const fn fence_pred(word: u32) -> u8 {
    ((word >> 24) & 0x0f) as u8
}

pub const fn fence_succ(word: u32) -> u8 {
    ((word >> 20) & 0x0f) as u8
}

pub const fn fence_fm(word: u32) -> u8 {
    ((word >> 28) & 0x0f) as u8
}
