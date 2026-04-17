pub type Register = u8;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BranchKind { Beq, Bne, Blt, Bge, Bltu, Bgeu }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoadKind { Lb, Lh, Lw, Lbu, Lhu }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreKind { Sb, Sh, Sw }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpImmKind { Addi, Slti, Sltiu, Xori, Ori, Andi }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShiftImmKind { Slli, Srli, Srai }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpKind { Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MOpKind { Mul, Mulh, Mulhsu, Mulhu, Div, Divu, Rem, Remu }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseIInstruction {
    Lui { rd: Register, imm: i32 },
    Auipc { rd: Register, imm: i32 },
    Jal { rd: Register, imm: i32 },
    Jalr { rd: Register, rs1: Register, imm: i32 },
    Branch { kind: BranchKind, rs1: Register, rs2: Register, imm: i32 },
    Load { kind: LoadKind, rd: Register, rs1: Register, imm: i32 },
    Store { kind: StoreKind, rs1: Register, rs2: Register, imm: i32 },
    OpImm { kind: OpImmKind, rd: Register, rs1: Register, imm: i32 },
    ShiftImm { kind: ShiftImmKind, rd: Register, rs1: Register, shamt: u8 },
    Op { kind: OpKind, rd: Register, rs1: Register, rs2: Register },
    Fence { pred: u8, succ: u8, fm: u8 },
    Ecall, Ebreak,
}
impl MOpKind {
    pub const fn mnemonic(self) -> &'static str {
        match self { Self::Mul => "mul", Self::Mulh => "mulh", Self::Mulhsu => "mulhsu", Self::Mulhu => "mulhu", Self::Div => "div", Self::Divu => "divu", Self::Rem => "rem", Self::Remu => "remu" }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MInstruction {
    Mul { rd: Register, rs1: Register, rs2: Register },
    Mulh { rd: Register, rs1: Register, rs2: Register },
    Mulhsu { rd: Register, rs1: Register, rs2: Register },
    Mulhu { rd: Register, rs1: Register, rs2: Register },
    Div { rd: Register, rs1: Register, rs2: Register },
    Divu { rd: Register, rs1: Register, rs2: Register },
    Rem { rd: Register, rs1: Register, rs2: Register },
    Remu { rd: Register, rs1: Register, rs2: Register },
}
impl MInstruction {
    pub const fn rd(self) -> Register { match self { Self::Mul { rd, .. } | Self::Mulh { rd, .. } | Self::Mulhsu { rd, .. } | Self::Mulhu { rd, .. } | Self::Div { rd, .. } | Self::Divu { rd, .. } | Self::Rem { rd, .. } | Self::Remu { rd, .. } => rd } }
    pub const fn rs1(self) -> Register { match self { Self::Mul { rs1, .. } | Self::Mulh { rs1, .. } | Self::Mulhsu { rs1, .. } | Self::Mulhu { rs1, .. } | Self::Div { rs1, .. } | Self::Divu { rs1, .. } | Self::Rem { rs1, .. } | Self::Remu { rs1, .. } => rs1 } }
    pub const fn rs2(self) -> Register { match self { Self::Mul { rs2, .. } | Self::Mulh { rs2, .. } | Self::Mulhsu { rs2, .. } | Self::Mulhu { rs2, .. } | Self::Div { rs2, .. } | Self::Divu { rs2, .. } | Self::Rem { rs2, .. } | Self::Remu { rs2, .. } => rs2 } }
    pub const fn mnemonic(self) -> &'static str { match self { Self::Mul { .. } => "mul", Self::Mulh { .. } => "mulh", Self::Mulhsu { .. } => "mulhsu", Self::Mulhu { .. } => "mulhu", Self::Div { .. } => "div", Self::Divu { .. } => "divu", Self::Rem { .. } => "rem", Self::Remu { .. } => "remu" } }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction { BaseI(BaseIInstruction), M(MInstruction) }
impl Instruction {
    pub fn mnemonic(&self) -> &'static str { match self { Self::BaseI(i) => i.mnemonic(), Self::M(i) => i.mnemonic() } }
}
impl BaseIInstruction {
    pub fn mnemonic(&self) -> &'static str { match self { Self::Lui { .. } => "lui", Self::Auipc { .. } => "auipc", Self::Jal { .. } => "jal", Self::Jalr { .. } => "jalr", Self::Branch { .. } => "branch", Self::Load { .. } => "load", Self::Store { .. } => "store", Self::OpImm { .. } => "opimm", Self::ShiftImm { .. } => "shiftimm", Self::Op { .. } => "op", Self::Fence { .. } => "fence", Self::Ecall => "ecall", Self::Ebreak => "ebreak" } }
}
