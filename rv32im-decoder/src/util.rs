pub const fn opcode(word: u32) -> u8 { (word & 0x7f) as u8 }
pub const fn rd(word: u32) -> u8 { ((word >> 7) & 0x1f) as u8 }
pub const fn funct3(word: u32) -> u8 { ((word >> 12) & 0x7) as u8 }
pub const fn rs1(word: u32) -> u8 { ((word >> 15) & 0x1f) as u8 }
pub const fn rs2(word: u32) -> u8 { ((word >> 20) & 0x1f) as u8 }
pub const fn funct7(word: u32) -> u8 { ((word >> 25) & 0x7f) as u8 }

pub const fn imm_i(word: u32) -> i32 { (word as i32) >> 20 }
pub const fn imm_s(word: u32) -> i32 {
    (((word & 0xfe000000) as i32) >> 20) | (((word >> 7) & 0x1f) as i32)
}
pub const fn imm_b(word: u32) -> i32 {
    (((word & 0x80000000) as i32) >> 19) | (((word & 0x7e000000) >> 20) as i32) |
    (((word >> 7) & 0x1e) as i32) | (((word << 4) & 0x800) as i32)
}
pub const fn imm_u(word: u32) -> i32 { (word & 0xfffff000) as i32 }
pub const fn imm_j(word: u32) -> i32 {
    (((word & 0x80000000) as i32) >> 11) | ((word & 0xff000) as i32) |
    (((word >> 9) & 0x800) as i32) | (((word >> 20) & 0x7fe) as i32)
}