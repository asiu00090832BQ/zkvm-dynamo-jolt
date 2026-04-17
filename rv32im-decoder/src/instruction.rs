#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BranchKind {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoadKind {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreKind {
    Sb,
    Sh,
    Sw,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AluImmKind {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AluRegKind {
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
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    Lui {
        rd: u8,
        imm: u32,
    },
    Auipc {
        rd: u8,
        imm: u32,
    },
    Jal {
        rd: u8,
        imm: i32,
    },
    Jalr {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Branch {
        kind: BranchKind,
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    Load {
        kind: LoadKind,
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Store {
        kind: StoreKind,
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    OpImm {
        kind: AluImmKind,
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Op {
        kind: AluRegKind,
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
}
