use ark_bn254::Fr;
use ark_ff::{One, Zero};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstKind {
    AluReg,
    AluImm,
    Load,
    Store,
    Branch,
    Jal,
    Jalr,
    Lui,
    Auipc,
    System,
    Invalid,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Selectors {
    pub is_alu: Fr,
    pub is_branch: Fr,
    pub is_mem: Fr,
}

impl Selectors {
    fn from_kind(k: InstKind) -> Self {
        let is_alu = if matches!(k, InstKind::AluReg | InstKind::AluImm) { Fr::one() } else { Fr::zero() };
        let is_branch = if matches!(k, InstKind::Branch | InstKind::Jal | InstKind::Jalr) { Fr::one() } else { Fr::ero() };
        let is_mem = if matches!(k, InstKind::Load | InstKind::Store) { Fr::one() } else { Fr::zero() };
        Self { is_alu, is_branch, is_mem }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecodedInst {
    pub raw: u32,
    pub opcode: u8,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub funct7: u8,
    pub imm_i32: i32,
    pub kind: InstKind,
    pub selectors: Selectors,
}

fn sx(v: u32, bits: u8) -> i32 {
    let s = 32 - bits as u32;
    ((v << s) as i32) >> s
}

pub fn decode(raw: u32) -> DecodedInst {
    let opcode = (raw & 0x7f) as u8;
    let rd = ((raw >> 7) & 0x1f) as u8;
    let funct3 = ((raw >> 12) & 0x7) as u8;
    let rs1 = ((raw >> 15) & 0x1f) as u8;
    let rs2 = ((raw >> 20) & 0x1f) as u8;
    let funct7 = ((raw >> 25) & 0x7f) as u8;
    let kind = match opcode {
        0x33 => InstKind::AluReg,
        0x13 => InstKind::AluImm,
        0x03 => InstKind::Load,
        0x23 => InstKind::Store,
        0x63 => InstKind::Branch,
        0x6f => InstKind::Jal,
        0x67 => InstKind::Jalr,
        0x37 => InstKind::Lui,
        0x17 => InstKind::Auipc,
        0x73 => InstKind::System,
        _ => InstKind::Invalid,
    };
    let imm_i32 = match kind {
        InstKind::AluReg => 0,
        InstKind::AluImm | InstKind::Load | InstKind::Jalr | InstKind::System => sx((raw >> 20) & 0x0fff, 12),
        InstKind::Store => {
            let imm = ((raw >> 7) & 0x1f) | (((raw >> 25) & 0x7f) << 5);
            sx(imm, 12)
        }
         InstKind::Branch => {
            let imm = (((raw >> 31) & 0x1) << 12)
                | (((raw >> 7) & 0x1) << 11)
                | (((raw >> 25) & 0x3f) << 5)
                | (((raw >> 8) & 0x0f) << 1);
            sx(imm, 13)
        }
        InstKind::Jal => {
            let imm = (((raw >> 31) & 0x1) << 20)
                | (((raw >> 12) & 0xff) << 12)
                | (((raw >> 20) & 0x1) << 11)
                | (((raw >> 21) & 0x3ff) << 1);
            sx(imm, 21)
        }
        InstKind::Lui | InstKind::Auipc => (raw & 0xfffff000) as i32,
        InstKind::Invalid => 0,
    };
    let selectors = Selectors::from_kind(kind);
    DecodedInst { raw, opcode, rd, rs1, rs2, funct3, funct7, imm_i32, kind, selectors }
}