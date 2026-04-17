#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add { rd: u8, rs1: u8, rs2: u8 },
    Sub { rd: u8, rs1: u8, rs2: u8 },
    MulDiv { kind: MulDivKind, rd: u8, rs1: u8, rs2: u8 },
    Ecall,
    Invalid(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MulDivKind {
    Mul, Mulh, Mulhsu, Mulhu,
    Div, Divu, Rem, Remu,
}
