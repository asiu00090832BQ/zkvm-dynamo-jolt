use core::fmt;

use crate::{format::Format, register::Register};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BranchKind {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

impl BranchKind {
    pub const fn mnemonic(self) -> &'static str {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoadKind {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

impl LoadKind {
    pub const fn mnemonic(self) -> &'static str {
        match self {
            Self::Lb => "lb",
            Self::Lh => "lh",
            Self::Lw => "lw",
            Self::Lbu => "lbu",
            Self::Lhu => "lhu",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreKind {
    Sb,
    Sh,
    Sw,
}

impl StoreKind {
    pub const fn mnemonic(self) -> &'static str {
        match self {
            Self::Sb => "sb",
            Self::Sh => "sh",
            Self::Sw => "sw",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpImmKind {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
}

impl OpImmKind {
    pub const fn mnemonic(self) -> &'static str {
        match self {
            Self::Addi => "addi",
            Self::Slti => "slti",
            Self::Sltiu => "sltiu",
            Self::Xori => "xori",
            Self::Ori => "ori",
            Self::Andi => "andi",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShiftImmKind {
    Slli,
    Srli,
    Srai,
}

impl ShiftImmKind {
    pub const fn mnemonic(self) -> &'static str {
        match self {
            Self::Slli => "slli",
            Self::Srli => "srli",
            Self::Srai => "srai",
        }
    }
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
    pub const fn mnemonic(self) -> &'static str {
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

    pub const fn is_m(self) -> bool {
        matches!(
            self,
            Self::Mul
                | Self::Mulh
                | Self::Mulhsu
                | Self::Mulhu
                | Self::Div
                | Self::Divu
                | Self::Rem
                | Self::Remu
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemKind {
    Ecall,
    Ebreak,
}

impl SystemKind {
    pub const fn mnemonic(self) -> &'static str {
        match self {
            Self::Ecall => "ecall",
            Self::Ebreak => "ebreak",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CsrKind {
    Csrrw,
    Csrrs,
    Csrrc,
    Csrrwi,
    Csrrsi,
    Csrrci,
}

impl CsrKind {
    pub const fn mnemonic(self) -> &'static str {
        match self {
            Self::Csrrw => "csrrw",
            Self::Csrrs => "csrrs",
            Self::Csrrc => "csrrc",
            Self::Csrrwi => "csrrwi",
            Self::Csrrsi => "csrrsi",
            Self::Csrrci => "csrrci",
        }
    }

    pub const fn is_immediate(self) -> bool {
        matches!(self, Self::Csrrwi | Self::Csrrsi | Self::Csrrci)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    Lui {
        rd: Register,
        imm: u32,
    },
    Auipc {
        rd: Register,
        imm: u32,
    },
    Jal {
        rd: Register,
        imm: i32,
    },
    Jalr {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Branch {
        kind: BranchKind,
        rs1: Register,
        rs2: Register,
        imm: i32,
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
    ShiftImm {
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
    Fence {
        fm: u8,
        pred: u8,
        succ: u8,
    },
    FenceI,
    System {
        kind: SystemKind,
    },
    Csr {
        kind: CsrKind,
        rd: Register,
        rs1: Register,
        csr: u16,
    },
    CsrImm {
        kind: CsrKind,
        rd: Register,
        zimm: u8,
        csr: u16,
    },
}

impl Instruction {
    pub const fn format(self) -> Format {
        match self {
            Self::Lui { .. } | Self::Auipc { .. } => Format::U,
            Self::Jal { .. } => Format::J,
            Self::Jalr { .. }
            | Self::Load { .. }
            | Self::OpImm { .. }
            | Self::ShiftImm { .. } => Format::I,
            Self::Store { .. } => Format::S,
            Self::Branch { .. } => Format::B,
            Self::Op { .. } => Format::R,
            Self::Fence { .. } | Self::FenceI => Format::Fence,
            Self::System { .. } | Self::Csr { .. } | Self::CsrImm { .. } => Format::System,
        }
    }

    pub const fn mnemonic(self) -> &'static str {
        match self {
            Self::Lui { .. } => "lui",
            Self::Auipc { .. } => "auipc",
            Self::Jal { .. } => "jal",
            Self::Jalr { .. } => "jalr",
            Self::Branch { kind, .. } => kind.mnemonic(),
            Self::Load { kind, .. } => kind.mnemonic(),
            Self::Store { kind, .. } => kind.mnemonic(),
            Self::OpImm { kind, .. } => kind.mnemonic(),
            Self::ShiftImm { kind, .. } => kind.mnemonic(),
            Self::Op { kind, .. } => kind.mnemonic(),
            Self::Fence { .. } => "fence",
            Self::FenceI => "fence.i",
            Self::System { kind } => kind.mnemonic(),
            Self::Csr { kind, .. } | Self::CsrImm { kind, .. } => kind.mnemonic(),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Lui { rd, imm } => write!(f, "lui {rd}, 0x{imm:08x}"),
            Self::Auipc { rd, imm } => write!(f, "auipc {rd}, 0x{imm:08x}"),
            Self::Jal { rd, imm } => write!(f, "jal {rd}, {imm}"),
            Self::Jalr { rd, rs1, imm } => write!(f, "jalr {rd}, {imm}({rs1})"),
            Self::Branch {
                kind,
                rs1,
                rs2,
                imm,
            } => write!(f, "{} {rs1}, {rs2}, {imm}", kind.mnemonic()),
            Self::Load { kind, rd, rs1, imm } => {
                write!(f, "{} {rd}, {imm}({rs1})", kind.mnemonic())
            }
            Self::Store { kind, rs1, rs2, imm } => {
                write!(f, "{} {rs2}, {imm}({rs1})", kind.mnemonic())
            }
            Self::OpImm { kind, rd, rs1, imm } => {
                write!(f, "{} {rd}, {rs1}, {imm}", kind.mnemonic())
            }
            Self::ShiftImm {
                kind,
                rd,
                rs1,
                shamt,
            } => write!(f, "{} {rd}, {rs1}, {shamt}", kind.mnemonic()),
            Self::Op { kind, rd, rs1, rs2 } => {
                write!(f, "{} {rd}, {rs1}, {rs2}", kind.mnemonic())
            }
            Self::Fence { fm, pred, succ } => {
                write!(f, "fence fm={fm:#x}, pred={pred:#x}, succ={succ:#x}")
            }
            Self::FenceI => f.write_str("fence.i"),
            Self::System { kind } => f.write_str(kind.mnemonic()),
            Self::Csr { kind, rd, rs1, csr } => {
                write!(f, "{} {rd}, 0x{csr:03x}, {rs1}", kind.mnemonic())
            }
            Self::CsrImm {
                kind,
                rd,
                zimm,
                csr,
            } => write!(f, "{} {rd}, 0x{csr:03x}, {zimm}", kind.mnemonic()),
        }
    }
}
