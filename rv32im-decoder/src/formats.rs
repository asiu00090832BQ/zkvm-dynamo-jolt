#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RFormat { pub opcode: u8, pub rd: u8, pub funct3: u8, pub rs1: u8, pub rs2: u8, pub funct7: u8 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IFormat { pub opcode: u8, pub rd: u8, pub funct3: u8, pub rs1: u8, pub imm: i32 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SFormat { pub opcode: u8, pub funct3: u8, pub rs1: u8, pub rs2: u8, pub imm: i32 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BFormat { pub opcode: u8, pub funct3: u8, pub rs1: u8, pub rs2: u8, pub imm: i32 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UFormat { pub opcode: u8, pub rd: u8, pub imm: i32 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JFormat { pub opcode: u8, pub rd: u8, pub imm: i32 }

#[inline] pub const fn opcode(word: u32) -> u8 { (word & 0x7f) as u8 }
#[inline] const fn sign_extend(value: u32, bits: u32) -> i32 { let shift = 32 - bits; ((value << shift) as i32) >> shift }

#[inline] pub const fn extract_r(word: u32) -> RFormat { RFormat { opcode: opcode(word), rd: ((word >> 7) & 0x1f) as u8, funct3: ((word >> 12) & 0x07) as u8, rs1: ((word >> 15) & 0x1f) as u8, rs2: ((word >> 20) & 0x1f) as u8, funct7: ((word >> 25) & 0x7f) as u8 } }
#[inline] pub const fn extract_i(word: u32) -> IFormat { IFormat { opcode: opcode(word), rd: ((word >> 7) & 0x1f) as u8, funct3: ((word >> 12) & 0x07) as u8, rs1: ((word >> 15) & 0x1f) as u8, imm: sign_extend((word >> 20) & 0x0fff, 12) } }
#[inline] pub const fn extract_s(word: u32) -> SFormat { let imm = (((word >> 25) & 0x7f) << 5) | ((word >> 7) & 0x1f); SFormat { opcode: opcode(word), funct3: ((word >> 12) & 0x07) as u8, rs1: ((word >> 15) & 0x1f) as u8, rs2: ((word >> 20) & 0x1f) as u8, imm: sign_extend(imm, 12) } }
#[inline] pub const fn extract_b(word: u32) -> BFormat { let imm = (((word >> 31) & 0x1) << 12) | (((word >> 7) & 0x1) << 11) | (((word >> 25) & 0x3f) << 5) | (((word >> 8) & 0x0f) << 1); BFormat { opcode: opcode(word), funct3: ((word >> 12) & 0x07) as u8, rs1: ((word >> 15) & 0x1f) as u8, rs2: ((word >> 20) & 0x1f) as u8, imm: sign_extend(imm, 13) } }
#[inline] pub const fn extract_u(word: u32) -> UFormat { UFormat { opcode: opcode(word), rd: ((word >> 7) & 0x1f) as u8, imm: (word & 0xffff_f000) as i32 } }
#[inline] pub const fn extract_j(word: u32) -> JFormat { let imm = (((word >> 31) & 0x1) << 20) | (((word >> 12) & 0xff) << 12) | (((word >> 20) & 0x1) << 11) | (((word >> 21) & 0x03ff) << 1); JFormat { opcode: opcode(word), rd: ((word >> 7) & 0x1f) as u8, imm: sign_extend(imm, 21) } }
