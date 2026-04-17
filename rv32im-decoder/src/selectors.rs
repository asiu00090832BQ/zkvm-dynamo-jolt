pub const OPCODE_LUI: u32 = 0b0110111;
pub const OPCODE_AUIPC: u32 = 0b0010111;
pub const OPCODE_JAL: u32 = 0b1101111;
pub const OPCODE_JALR: u32 = 0b1100111;
pub const OPCODE_BRANCH: u32 = 0b1100011;
pub const OPCODE_LOAD: u32 = 0b0000011;
pub const OPCODE_STORE: u32 = 0b0100011;
pub const OPCODE_OP_IMM: u32 = 0b0010011;
pub const OPCODE_OP: u32 = 0b0110011;
pub const OPCODE_MISC_MEM: u32 = 0b0001111;
pub const OPCODE_SYSTEM: u32 = 0b1110011;

pub const fn bits(word: u32, offset: u32, width: u32) -> u32 {
    (word >> offset) & ((1u32 << width) - 1)
}

pub const fn opcode(word: u32) -> u32 {
    bits(word, 0, 7)
}

pub const fn rd(word: u32) -> u8 {
    bits(word, 7, 5) as u8
}

pub const fn funct3(word: u32) -> u32 {
    bits(word, 12, 3)
}

pub const fn rs1(word: u32) -> u8 {
    bits(word, 15, 5) as u8
}

pub const fn rs2(word: u32) -> u8 {
    bits(word, 20, 5) as u8
}

pub const fn funct7(word: u32) -> u32 {
    bits(word, 25, 7)
}

pub const fn shamt(word: u32) -> u8 {
    bits(word, 20, 5) as u8
}

pub const fn sign_extend(value: u32, width: u32) -> i32 {
    let shift = 32 - width;
    ((value << shift) as i32) >> shift
}

pub const fn imm_i(word: u32) -> i32 {
    sign_extend(bits(word, 20, 12), 12)
}

pub const fn imm_s(word: u32) -> i32 {
    sign_extend(bits(word, 7, 5) | (bits(word, 25, 7) << 5), 12)
}

pub const fn imm_b(word: u32) -> i32 {
    sign_extend(
        (bits(word, 8, 4) << 1)
            | (bits(word, 25, 6) << 5)
            | (bits(word, 7, 1) << 11)
            | (bits(word, 31, 1) << 12),
        13,
    )
}

pub const fn imm_u(word: u32) -> i32 {
    (word & 0xffff_f000) as i32
}

pub const fn imm_j(word: u32) -> i32 {
    sign_extend(
        (bits(word, 21, 10) << 1)
            | (bits(word, 20, 1) << 11)
            | (bits(word, 12, 8) << 12)
            | (bits(word, 31, 1) << 20),
        21,
    )
}
