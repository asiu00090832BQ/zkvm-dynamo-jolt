use crate::instruction::Register;
#[derive(Debug, Clone, Copy)]
pub struct RawInstruction { pub word: u32 }
impl RawInstruction {
    pub const fn new(word: u32) -> Self { Self { word } }
    pub const fn opcode(self) -> u8 { (self.word & 0x7f) as u8 }
    pub const fn rd(self) -> Register { ((self.word >> 7) & 0x1f) as Register }
    pub const fn funct3(self) -> u8 { ((self.word >> 12) & 0x07) as u8 }
    pub const fn rs1(self) -> Register { ((self.word >> 15) & 0x1f) as Register }
    pub const fn rs2(self) -> Register { ((self.word >> 20) & 0x1f) as Register }
    pub const fn funct7(self) -> u8 { ((self.word >> 25) & 0x7f) as u8 }
}
pub struct RType { pub rd: Register, pub funct3: u8, pub rs1: Register, pub rs2: Register, pub funct7: u8 }
impl RType { pub fn decode(w: u32) -> Self { let r = RawInstruction::new(w); Self { rd: r.rd(), funct3: r.funct3(), rs1: r.rs1(), rs2: r.rs2(), funct7: r.funct7() } } }
pub struct IType { pub rd: Register, pub funct3: u8, pub rs1: Register, pub imm: i32 }
impl IType { pub fn decode(w: u32) -> Self { let r = RawInstruction::new(w); Self { rd: r.rd(), funct3: r.funct3(), rs1: r.rs1(), imm: ((w as i32) >> 20) } } }
pub struct SType { pub funct3: u8, pub rs1: Register, pub rs2: Register, pub imm: i32 }
impl SType { pub fn decode(w: u32) -> Self { let r = RawInstruction::new(w); let imm = (((w >> 25) as i32) << 5) | (((w >> 7) & 0x1f) as i32); Self { funct3: r.funct3(), rs1: r.rs1(), rs2: r.rs2(), imm: (imm << 20) >> 20 } } }
pub struct BType { pub funct3: u8, pub rs1: Register, pub rs2: Register, pub imm: i32 }
impl BType { pub fn decode(w: u32) -> Self { let r = RawInstruction::new(w); let imm = (((w >> 31) as i32) << 12) | (((w >> 7) & 0x1) << 11) | ((((w >> 25) & 0x3f) as i32) << 5) | ((((w >> 8) & 0xf) as i32) << 1); Self { funct3: r.funct3(), rs1: r.rs1(), rs2: r.rs2(), imm: (imm << 19) >> 19 } } }
pub struct UType { pub rd: Register, pub imm: i32 }
impl UType { pub fn decode(w: u32) -> Self { let r = RawInstruction::new(w); Self { rd: r.rd(), imm: (w & 0xfffff000) as i32 } } }
pub struct JType { pub rd: Register, pub imm: i32 }
impl JType { pub fn decode(w: u32) -> Self { let r = RawInstruction::new(w); let imm = (((w >> 31) as i32) << 20) | ((((w >> 12) & 0xff) as i32) << 12) | ((((w >> 20) & 0x1) as i32) << 11) | ((((w >> 21) & 0x3ff) as i32) << 1); Self { rd: r.rd(), imm: (imm << 11) >> 11 } } }
