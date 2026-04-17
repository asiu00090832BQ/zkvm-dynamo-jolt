#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And,
    Mul, Mulh, Mulhsu, Mulhu, Div, Divu, Rem, Remu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedInstruction {
    pub raw: u32,
    pub instruction: Instruction,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
}
