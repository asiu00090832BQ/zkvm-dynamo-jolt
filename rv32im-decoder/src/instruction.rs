#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    Lui, Auipc, Jal, Jalr, Beq, Bne, Blt, Bge, Bltu, Bgeu, Lb, Lh, Lw, Lbu, Lhu, Sb, Sh, Sw, Addi, Slti, Sltiu, Xori, Ori, Andi, Slli, Srli, Srai, Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And, Fence, Ecall, Ebreak, Mul, Mulh, Mulhsu, Mulhu, Div, Divu, Rem, Remu,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecodedInstruction {
    pub raw: u32, pub instruction: Instruction, pub rd: Option<u8>, pub rs1: Option<u8>, pub rs2: Option<u8>, pub imm: Option<i32>,
}

impl DecodedInstruction {
    #[inline] pub const fn new(raw: u32, instruction: Instruction, rd: Option<u8>, rs1: Option<u8>, rs2: Option<u8>, imm: Option<i32>) -> Self {
        Self { raw, instruction, rd, rs1, rs2, imm }
    }
}
