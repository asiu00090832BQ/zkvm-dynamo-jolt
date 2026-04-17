#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rv32Extension {
    I,
    M,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rv32Opcode {
    Lui,
    Auipc,
    Jal,
    Jalr,
    Branch,
    Load,
    Store,
    OpImm,
    Op,
    MiscMem,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IInstruction {
    Lui,
    Auipc,
    Jal,
    Jalr,
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
    Sb,
    Sh,
    Sw,
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,
    Fence,
    Ecall,
    Ebreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MInstruction {
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionKind {
    I(IInstruction),
    M(MInstruction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedInstruction {
    pub word: u32,
    pub opcode: Rv32Opcode,
    pub extension: Rv32Extension,
    pub kind: InstructionKind,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub funct7: u8,
    pub imm: i32,
}

impl DecodedInstruction {
    pub const fn new_i(
        word: u32,
        opcode: Rv32Opcode,
        kind: IInstruction,
        rd: u8,
        rs1: u8,
        rs2: u8,
        funct3: u8,
        funct7: u8,
        imm: i32,
    ) -> Self {
        Self {
            word,
            opcode,
            extension: Rv32Extension::I,
            kind: InstructionKind::I(kind),
            rd,
            rs1,
            rs2,
            funct3,
            funct7,
            imm,
        }
    }

    pub const fn new_m(
        word: u32,
        opcode: Rv32Opcode,
        kind: MInstruction,
        rd: u8,
        rs1: u8,
        rs2: u8,
        funct3: u8,
        funct7: u8,
    ) -> Self {
        Self {
            word,
            opcode,
            extension: Rv32Extension::M,
            kind: InstructionKind::M(kind),
            rd,
            rs1,
            rs2,
            funct3,
            funct7,
            imm: 0,
        }
    }
}
