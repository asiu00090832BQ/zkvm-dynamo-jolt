use crate::formats::{BType, IType, JType, RType, SType, UType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchKind {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

impl BranchKind {
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Beq => "beq",
            Self::Bne => "bne",
            Self::Blt => "blt",
            Self::Bge => "bge",
            Self::Bltu => "bltu",
            Self::Bgeu => "bgeu",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadKind {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

impl LoadKind {
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Lb => "lb",
            Self::Lh => "lh",
            Self::Lw => "lw",
            Self::Lbu => "lbu",
            Self::Lhu => "lhu",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreKind {
    Sb,
    Sh,
    Sw,
}

impl StoreKind {
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Sb => "sb",
            Self::Sh => "sh",
            Self::Sw => "sw",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl OpImmKind {
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Addi => "addi",
            Self::Slti => "slti",
            Self::Sltiu => "sltiu",
            Self::Xori => "xori",
            Self::Ori => "ori",
            Self::Andi => "andi",
            Self::Slli => "slli",
            Self::Srli => "srli",
            Self::Srai => "srai",
        }
    }
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
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

impl OpKind {
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Sll => "sll",
            Self::Slt => "slt",
            Self::Sltu => "sltu",
            Self::Xor => "xor",
            Self::Srl => "srl",
            Self::Sra => "sra",
            Self::Or => "or",
            Self::And => "and",
            Self::Mul => "mul",
            Self::Mulh => "mulh",
            Self::Mulhsu => "mulhsu",
            Self::Mulhu => "mulhu",
            Self::Div => "div",
            Self::Divu => "divu",
            Self::Rem => "rem",
            Self::Remu => "remu",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemKind {
    Ecall,
    Ebreak,
}

impl SystemKind {
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Ecall => "ecall",
            Self::Ebreak => "ebreak",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui(UType),
    Auipc(UType),
    Jal(JType),
    Jalr(IType),
    Branch { kind: BranchKind, format: BType },
    Load { kind: LoadKind, format: IType },
    Store { kind: StoreKind, format: SType },
    OpImm { kind: OpImmKind, format: IType },
    Op { kind: OpKind, format: RType },
    Fence,
    System(SystemKind),
}

impl Instruction {
    pub fn mnemonic(&self) -> &'static str {
        match self {
            Self::Lui(_) => "lui",
            Self::Auipc(_) => "auipc",
            Self::Jal(_) => "jal",
            Self::Jalr(_) => "jalr",
            Self::Branch { kind, .. } => kind.mnemonic(),
            Self::Load { kind, .. } => kind.mnemonic(),
            Self::Store { kind, .. } => kind.mnemonic(),
            Self::OpImm { kind, .. } => kind.mnemonic(),
            Self::Op { kind, .. } => kind.mnemonic(),
            Self::Fence => "fence",
            Self::System(kind) => kind.mnemonic(),
        }
    }

    pub fn rd(&self) -> Option<u8> {
        match self {
            Self::Lui(format) => Some(format.rd),
            Self::Auipc(format) => Some(format.rd),
            Self::Jal(format) => Some(format.rd),
            Self::Jalr(format) => Some(format.rd),
            Self::Branch { .. } => None,
            Self::Load { format, .. } => Some(format.rd),
            Self::Store { .. } => None,
            Self::OpImm { format, .. } => Some(format.rd),
            Self::Op { format, .. } => Some(format.rd),
            Self::Fence => None,
            Self::System(_) => None,
        }
    }

    pub fn rs1(&self) -> Option<u8> {
        match self {
            Self::Lui(_) => None,
            Self::Auipc(_) => None,
            Self::Jal(_) => None,
            Self::Jalr(format) => Some(format.rs1),
            Self::Branch { format, .. } => Some(format.rs1),
            Self::Load { format, .. } => Some(format.rs1),
            Self::Store { format, .. } => Some(format.rs1),
            Self::OpImm { format, .. } => Some(format.rs1),
            Self::Op { format, .. } => Some(format.rs1),
            Self::Fence => None,
            Self::System(_) => None,
        }
    }

    pub fn rs2(&self) -> Option<u8> {
        match self {
            Self::Branch { format, .. } => Some(format.rs2),
            Self::Store { format, .. } => Some(format.rs2),
            Self::Op { format, .. } => Some(format.rs2),
            _ => None,
        }
    }
}
