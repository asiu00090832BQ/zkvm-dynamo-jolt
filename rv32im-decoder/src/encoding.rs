pub const OPCODE_LOAD: u8 = 0b0000011;
pub const OPCODE_MISC_MEM: u8 = 0b0001111;
pub const OPCODE_OP_IMM: u8 = 0b0010011;
pub const OPCODE_AUIPC: u8 = 0b0010111;
pub const OPCODE_STORE: u8 = 0b0100011;
pub const OPCODE_OP: u8 = 0b0110011;
pub const OPCODE_LUI: u8 = 0b0110111;
pub const OPCODE_BRANCH: u8 = 0b1100011;
pub const OPCODE_JALR: u8 = 0b1100111;
pub const OPCODE_JAL: u8 = 0b1101111;
pub const OPCODE_SYSTEM: u8 = 0b1110011;

pub const FUNCT7_BASE: u8 = 0b0000000;
pub const FUNCT7_ALT: u8 = 0b0100000;
pub const FUNCT7_M: u8 = 0b0000001;

#[inline(always)]
pub const fn opcode(word: u32) -> u8 {
    (word & 0x7f) as u8
}

#[inline(always)]
pub const fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1f) as u8
}

#[inline(always)]
pub const fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x07) as u8
}

#[inline(always)]
pub const fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}

#[inline(always)]
pub const fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

#[inline(always)]
pub const fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7f) as u8
}

#[inline(always)]
pub const fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits as u32;
    ((value << shift) as i32) >> shift
}

#[inline(always)]
pub const fn imm_i(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

#[inline(always)]
pub const fn imm_s(word: u32) -> i32 {
    let value = ((word >> 25) << 5) | ((word >> 7) & 0x1f);
    sign_extend(value, 12)
}

#[inline(always)]
pub const fn imm_b(word: u32) -> i32 {
    let value = (((word >> 31) & 0x1) << 12)
        | (((word >> 7) & 0x1) << 11)
        | (((word >> 25) & 0x3f) << 5)
        | (((word >> 8) & 0x0f) << 1);
    sign_extend(value, 13)
}

#[inline(always)]
pub const fn imm_u(word: u32) -> i32 {
    (word & 0xfffff000) as i32
}

#[inline(always)]
pub const fn imm_j(word: u32) -> i32 {
    let value = (((word >> 31) & 0x1) << 20)
        | (((word >> 12) & 0xff) << 12)
        | (((word >> 20) & 0x1) << 11)
        | (((word >> 21) & 0x03ff) << 1);
    sign_extend(value, 21)
}
