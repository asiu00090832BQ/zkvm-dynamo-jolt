use core::fmt;

pub type Register = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchKind {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadKind {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreKind {
    Sb,
    Sh,
    Sw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpImmKind {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftImmKind {
    Sll,
    Srl,
    Sra,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui { rd: Register, imm: i32 },
    Auipc { rd: Register, imm: i32 },
    Jal { rd: Register, offset: i32 },
    Jalr {
        rd: Register,
        rs1: Register,
        offset: i32,
    },
    Branch {
        kind: BranchKind,
        rs1: Register,
        rs2: Register,
        offset: i32,
    },
    Load {
        kind: LoadKind,
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Store {
        kind: StoreKind,
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    OpImm {
        kind: OpImmKind,
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    OpImmShift {
        kind: ShiftImmKind,
        rd: Register,
        rs1: Register,
        shamt: u8,
    },
    Op {
        kind: OpKind,
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    MulDiv {
        kind: MulDivKind,
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Fence { pred: u8, succ: u8 },
    FenceI,
    Ecall,
    Ebreak,
}
