use crate::types::{Immediate, RegisterIndex, Word};

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
    Byte,
    Half,
    Word,
    ByteUnsigned,
    HalfUnsigned,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreKind {
    Byte,
    Half,
    Word,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpImmKind {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MulKind {
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
pub enum SystemKind {
    Ecall,
    Ebreak,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    Lui {
        rd: RegisterIndex,
        imm: Word,
    },
    Auipc {
        rd: RegisterIndex,
        imm: Word,
    },
    Jal {
        rd: RegisterIndex,
        imm: Immediate,
    },
    Jalr {
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: Immediate,
    },
    Branch {
        kind: BranchKind,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        imm: Immediate,
    },
    Load {
        kind: LoadKind,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: Immediate,
    },
    Store {
        kind: StoreKind,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        imm: Immediate,
    },
    OpImm {
        kind: OpImmKind,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        imm: Immediate,
    },
    Op {
        kind: OpKind,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Mul {
        kind: MulKind,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    },
    Fence,
    FenceI,
    System(SystemKind),
}
