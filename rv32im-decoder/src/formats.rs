#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RType { pub rd: u8, pub rs1: u8, pub rs2: u8 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IType { pub rd: u8, pub rs1: u8, pub imm: i32 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftIType { pub rd: u8, pub rs1: u8, pub shamt: u32 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SType { pub rs1: u8, pub rs2: u8, pub imm: i32 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BType { pub rs1: u8, pub rs2: u8, pub imm: i32 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UType { pub rd: u8, pub imm: i32 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JType { pub rd: u8, pub imm: i32 }

pub const fn opcode(w: u32) -> u8 { (w & 0x7f) as u8 }
pub const fn rd(w: u32) -> u8 { ((w >> 7) & 0x1f) as u8 }
pub const fn funct3(w: u32) -> u8 { ((w >> 12) & 0x07) as u8 }
pub const fn rs1(w: u32) -> u8 { ((w >> 15) & 0x1f) as u8 }
pub const fn rs2(w: u32) -> u8 { ((w >> 20) & 0x1f) as u8 }
pub const fn funct7(w: u32) -> u8 { ((w >> 25) & 0x7f) as u8 }

fn sign_extend(v: u32, b: u8) -> i32 { let s = 32 - b; ((v << s) as i32) >> s }

impl RType { pub fn decode(w: u32) -> Self { Self { rd: rd(w), rs1: rs1(w), rs2: rs2(w) } } }
impl IType { pub fn decode(w: u32) -> Self { Self { rd: rd(w), rs1: rs1(w), imm: sign_extend(w >> 20, 12) } } }
impl ShiftIType { pub fn decode(w: u32) -> Self { Self { rd: rd(w), rs1: rs1(w), shamt: (w >> 20) & 0x1f } } }
impl SType { pub fn decode(w: u32) -> Self { let imm = ((w >> 7) & 0x1f) | ((w >> 25) << 5); Self { rs1: rs1(w), rs2: rs2(w), imm: sign_extend(imm, 12) } } }
impl BType { pub fn decode(w: u32) -> Self { let imm = ((w >> 8) & 0xf) << 1 | ((w >> 25) & 0x3f) << 5 | ((w >> 7) & 1) << 11 | (w >> 31) << 12; Self { rs1: rs1(w), rs2: rs2(w), imm: sign_extend(imm, 13) } } }
impl UType { pub fn decode(w: u32) -> Self { Self { rd: rd(w), imm: (w & 0xfffff000) as i32 } } }
impl JType { pub fn decode(w: u32) -> Self { let imm = ((w >> 21) & 0x3ff) << 1 | ((w >> 20) & 1) << 11 | ((w >> 12) & 0xff) << 12 | (w >> 31) << 20; Self { rd: rd(w), imm: sign_extend(imm, 21) } } }
