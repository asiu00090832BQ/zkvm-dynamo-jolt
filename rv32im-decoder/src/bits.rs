pub const fn bit_range(value: u32, lo: u8, width: u8) -> u32 { (value >> lo) & ((1u32 << width) - 1) }
pub const fn sign_extend(value: u32, bits: u8) -> i32 { let shift = 32 - bits as u32; ((value << shift) as i32) >> shift }
pub const fn i_imm(word: u32) -> i32 { sign_extend(bit_range(word, 20, 12), 12) }
pub const fn s_imm(word: u32) -> i32 { let imm = bit_range(word, 7, 5) | (bit_range(word, 25, 7) << 5); sign_extend(imm, 12) }
pub const fn b_imm(word: u32) -> i32 { let imm = (bit_range(word, 8, 4) << 1) | (bit_range(word, 25, 6) << 5) | (bit_range(word, 7, 1) << 11) | (bit_range(word, 31, 1) << 12); sign_extend(imm, 13) }
pub const fn u_imm(word: u32) -> i32 { (word & 0xfffff000) as i32 }
pub const fn j_imm(word: u32) -> i32 { let imm = (bit_range(word, 21, 10) << 1) | (bit_range(word, 20, 1) << 11) | (bit_range(word, 12, 8) << 12) | (bit_range(word, 31, 1) << 20); sign_extend(imm, 21) }
pub const fn shamt(word: u32) -> u8 { bit_range(word, 20, 5) as u8 }
