pub mod opcode {
    pub const LOAD: u8 = 0x03; pub const MISC_MEM: u8 = 0x0f; pub const OP_IMM: u8 = 0x13; pub const AUIPC: u8 = 0x17; pub const STORE: u8 = 0x23; pub const OP: u8 = 0x33; pub const LUI: u8 = 0x37; pub const BRANCH: u8 = 0x63; pub const JALR: u8 = 0x67; pub const JAL: u8 = 0x6f; pub const SYSTEM: u8 = 0x73;
}
pub mod funct3 {
    pub mod branch { pub const BEQ: u8 = 0; pub const BNE: u8 = 1; pub const BLT: u8 = 4; pub const BGE: u8 = 5; pub const BLTU: u8 = 6; pub const BGEU: u8 = 7; }
    pub mod load { pub const LB: u8 = 0; pub const LH: u8 = 1; pub const LW: u8 = 2; pub const LBU: u8 = 4; pub const LHU: u8 = 5; }
    pub mod store { pub const SB: u8 = 0; pub const SH: u8 = 1; pub const SW: u8 = 2; }
    pub mod op_imm { pub const ADDI: u8 = 0; pub const SLLI: u8 = 1; pub const SLTI: u8 = 2; pub const SLTIU: u8 = 3; pub const XORI: u8 = 4; pub const SRLI_SRAI: u8 = 5; pub const ORI: u8 = 6; pub const ANDI: u8 = 7; }
    pub mod op { pub const ADD_SUB: u8 = 0; pub const SLL: u8 = 1; pub const SLT: u8 = 2; pub const SLTU: u8 = 3; pub const XOR: u8 = 4; pub const SRL_SRA: u8 = 5; pub const OR: u8 = 6; pub const AND: u8 = 7; }
    pub mod misc_mem { pub const FENCE: u8 = 0; pub const FENCE_I: u8 = 1; }
    pub mod system { pub const PRIV: u8 = 0; pub const CSRRW: u8 = 1; pub const CSRRS: u8 = 2; pub const CSRRC: u8 = 3; pub const CSRRWI: u8 = 5; pub const CSRRSI: u8 = 6; pub const CSRRCI: u8 = 7; }
    pub mod m { pub const MUL: u8 = 0; pub const MULH: u8 = 1; pub const MULHSU: u8 = 2; pub const MULHU: u8 = 3; pub const DIV: u8 = 4; pub const DIVU: u8 = 5; pub const REM: u8 = 6; pub const REMU: u8 = 7; }
}
pub mod funct7 { pub const BASE: u8 = 0x00; pub const SUB_SRA: u8 = 0x20; pub const M_EXTENSION: u8 = 0x01; }
#[inline] pub const fn opcode(word: u32) -> u8 { (word & 0x7f) as u8 }
#[inline] pub const fn rd(word: u32) -> u8 { ((word >> 7) & 0x1f) as u8 }
#[inline] pub const fn funct3(word: u32) -> u8 { ((word >> 12) & 0x07) as u8 }
#[inline] pub const fn rs1(word: u32) -> u8 { ((word >> 15) & 0x1f) as u8 }
#[inline] pub const fn rs2(word: u32) -> u8 { ((word >> 20) & 0x1f) as u8 }
#[inline] pub const fn funct7(word: u32) -> u8 { ((word >> 25) & 0x7f) as u8 }
#[inline] pub const fn shamt(word: u32) -> u8 { ((word >> 20) & 0x1f) as u8 }
#[inline] pub const fn csr(word: u32) -> u16 { ((word >> 20) & 0x0fff) as u16 }
#[inline] pub const fn zimm(word: u32) -> u8 { rs1(word) }
#[inline] pub const fn fence_succ(word: u32) -> u8 { ((word >> 20) & 0x0f) as u8 }
#[inline] pub const fn fence_pred(word: u32) -> u8 { ((word >> 24) & 0x0f) as u8 }
#[inline] pub const fn fence_fm(word: u32) -> u8 { ((word >> 28) & 0x0f) as u8 }
#[inline] pub const fn sign_extend(value: u32, bits: u8) -> i32 { let shift = 32 - bits as u32; ((value << shift) as i32) >> shift }
#[inline] pub const fn imm_i(word: u32) -> i32 { sign_extend((word >> 20) & 0xfff, 12) }
#[inline] pub const fn imm_s(word: u32) -> i32 { let imm = ((word >> 25) << 5) | ((word >> 7) & 0x1f); sign_extend(imm, 12) }
#[inline] pub const fn imm_b(word: u32) -> i32 { let imm = ((word >> 31) << 12) | (((word >> 7) & 1) << 11) | (((word >> 25) & 0x3f) << 5) | (((word >> 8) & 0xf) << 1); sign_extend(imm, 13) }
#[inline] pub const fn imm_u(word: u32) -> i32 { (word & 0xfffff000) as i32 }
#[inline] pub const fn imm_j(word: u32) -> i32 { let imm = ((word >> 31) << 20) | (((word >> 12) & 0xff) << 12) | (((word >> 20) & 1) << 11) | (((word >> 21) & 0x3ff) << 1); sign_extend(imm, 21) }
