// Moved to instruction.rs and decode.rs based on common patterns, but I will provide a combined types.rs for this Redo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui, Auipc, Jal, Jalr,
    Beq, Bne, Blt, Bge, Bltu, Bgeu,
    Lb, Lhu, Lw, Lbu, Lhu,
    Sb, Sh, Sw,
    Addi, Slti, Sltiu, Xori, Ori, Andi, Slli, Srli, Srai,
    Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And,
    Mul, Mulh, Mulhfu, Mulhu, Div, Divu, Rem, Remu,
    Fence, Ecall, Ebreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub raw: u32,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
    pub instruction: Instruction,
}