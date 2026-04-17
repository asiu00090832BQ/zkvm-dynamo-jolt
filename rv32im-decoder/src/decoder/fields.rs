pub fn opcode(word: u32) -> u8 {
    (word & 0x7f) as u8
}

pub fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1f) as u8
}

pub fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x07) as u8
}

pub fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}

pub fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

pub fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7f) as u8
}

pub fn shamt(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

pub fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits;
    ((value << shift) as i32) >> shift
}

pub fn imm_i(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

pub fn imm_u(word: u32) -> i32 {
    (word & 0xffff_f000) as i32
}

pub fn imm_s(word: u32) -> i32 {
    let low = (word >> 7) & 0x1f;
    let high = (word >> 25) & 0x7f;
    sign_extend((high << 5) | low, 12)
}

pub fn imm_b(word: u32) -> i32 {
    let bit11 = ((word >> 7) & 0x1) << 11;
    let bits4_1 = ((word >> 8) & 0x0f) << 1;
    let bits10_5 = ((word >> 25) & 0x3f) << 5;
    let bit12 = ((word >> 31) & 0x1) << 12;
    sign_extend(bit12 | bit11 | bits10_5 | bits4_1, 13)
}

pub fn imm_j(word: u32) -> i32 {
    let bits19_12 = ((word >> 12) & 0xff) << 12;
    let bit11 = ((word >> 20) & 0x1) << 11;
    let bits10_1 = ((word >> 21) & 0x03ff) << 1;
    let bit20 = ((word >> 31) & 0x1) << 20;
    sign_extend(bit20 | bits19_12 | bit11 | bits10_1, 21)
}

pub fn csr(word: u32) -> u16 {
    ((word >> 20) & 0x0fff) as u16
}

pub fn zimm(word: u32) -> u8 {
    rs1(word)
}
