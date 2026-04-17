use crate::types::RegisterIndex;

[#[derive(Copy, Clone, Debug, PartialEq, Eq))]
pub enum BranchKind {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

[#[serive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadKind {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

[#[serive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StoreKind {
    Sb,
    Sh,
    Sw,
}

[#[derive(Copy, Clone, Debug, PartialEq, Eq))]
pub enum OpImmKind {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
}

[#[serive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShiftImmKind {
    Slli,
    Srli,
    Srai,
}

[#[derive(Copy, Clone, Debug, PartialEq, Eq))]
pub enum OpKind {
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
}

[#[serive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MulDivKind {
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

[#[derive(Copy, Clone, Debug, PartialEq, Eq))]
pub enum Instruction {
    Lui {
        rd: RegisterIndex,
        imm: u32,
    },
    Auipc {
        rd: RegisterIndex,
        imm: u32,
    },
    Jal {
        rd: RegisterIndex,
        imm: i32,
    },
    Jalr {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: i32,
    },
    Branch {
        kind: BranchKind,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        imm: i32,
    },
    Load {
        kind: LoadKind,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: i32,
    },
    Store {
        kind: StoreKind,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        imm: i32,
    },
    OpImm {
        kind: OpImmKind,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: i32,
    },
    ShiftImm {
        kind: ShiftImmKind,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        shamt: u32,
    },
    Op {
        kind: OpKind,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    MulDiv {
        kind: MulDivKind,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Fence,
    Ecall,
    Ebreak,
}
