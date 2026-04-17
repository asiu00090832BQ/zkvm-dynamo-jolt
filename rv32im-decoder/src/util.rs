pub fn bits(word: u32, lo: u8, width: u8) -> u32 {
    let mask = if width == 32 {
        u32::MAX
    } else {
        (1u32 << width) - 1
    };
    (word >> lo) & mask
}

pub fn opcode(word: u32) -> u32 {
    bits(word, 0, 7)
}

pub fn rd(word: u32) -> u8 {
    bits(word, 7, 5) as u8
}

pub fn funct3(word: u32) -> u32 {
    bits(word, 12, 3)
}

pub fn rs1(word: u32) -> u8 {
    bits(word, 15, 5) as u8
}

pub fn rs2(word: u32) -> u8 {
    bits(word, 20, 5) as u8
}

pub fn funct7(word: u32) -> u32 {
    bits(word, 25, 7)
}

pub fn shamt(word: u32) -> u8 {
    bits(word, 20, 5) as u8
}

pub fn sign_extend(value: u32, width: u8) -> i32 {
    let shift = 32 - width;
    ((value << shift) as i32) >> shift
}

pub fn imm_i(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

pub fn imm_s(word: u32) -> i32 {
    let imm = bits(word, 7, 5) | (bits(word, 25, 7) << 5);
    sign_extend(imm, 12)
}

pub fn imm_b(word: u32) -> i32 {
    let imm = (bits(word, 8, 4) << 1) | (bits(word, 25, 6) << 5) | (bits(word, 7, 1) << 11) | (bits(word, 31, 1) << 12);
    sign_extend(imm, 13)
}

pub fn imm_u(word: u32) -> i32 {
    (word & 0xffff_f000) as i32
}

pub fn imm_j(word: u32) -> i32 {
    let imm = (bits(word, 21, 10) << 1) | (bits(word, 20, 1) << 11) | (bits(word, 12, 8) << 12) | (bits(word, 31, 1) << 20);
    sign_extend(imm, 21)
}
