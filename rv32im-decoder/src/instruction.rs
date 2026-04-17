pub type Reg = u8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchKind {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadKind {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreKind {
    Sb,
    Sh,
    Sw,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemKind {
    Ecall,
    Ebreak,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    Lui { rd: Reg, imm: i32 },
    Auipc { rd: Reg, imm: i32 },
    Jal { rd: Reg, imm: i32 },
    Jalr { rd: Reg, rs1: Reg, imm: i32 },
    Branch { kind: BranchKind, rs1: Reg, rs2: Reg, imm: i32 },
    Load { kind: LoadKind, rd: Reg, rs1: Reg, imm: i32 },
    Store { kind: StoreKind, rs1: Reg, rs2: Reg, imm: i32 },
    OpImm { kind: OpImmKind, rd: Reg, rs1: Reg, imm: i32 },
    Op { kind: OpKind, rd: Reg, rs1: Reg, rs2: Reg },
    Fence,
    System { kind: SystemKind },
}
