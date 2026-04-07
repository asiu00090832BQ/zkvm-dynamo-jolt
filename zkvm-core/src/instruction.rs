use core::fmt;
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Register { X0 = 0, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28, X29, X30, X31 }
impl Register {
    pub fn from_u8(idx: u8) -> Option<Self> { if idx < 32 { Some(unsafe { std::mem::transmute(idx) }) } else { None } }
    pub fn index(self) -> usize { self as u8 as usize }
}
impl fmt::Display for Register { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, \"x{}\", *self as u8) } }
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BranchKind { Beq, Bne, Blt, Bge, Bltu, Bgeu }
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LoadKind { Lb, Lh, Lw, Lbu, Lhu }
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum StoreKind { Sb, Sh, Sw }
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ShiftKind { Sll, Srl, Sra }
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpKind { Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And, Mul, Mulh, Mulhsu, Mulhu, Div, Divu, Rem, Remu }
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpImmKind { Addi, Slti, Sltiu, Xori, Ori, Andi }
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CsrKind { Csrrw, Csrrs, Csrrc, Csrrwi, Csrrsi, Csrrci }
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Instruction {
    Lui { rd: Register, imm: i32 },
    Auipc { rd: Register, imm: i32 },
    Jal { rd: Register, offset: i32 },
    Jalr { rd: Register, rs1: Register, offset: i32 },
    Branch { kind: BranchKind, rs1: Register, rs2: Register, offset: i32 },
    Load { kind: LoadKind, rd: Register, rs1: Register, offset: i32 },
    Store { kind: StoreKind, rs1: Register, rs2: Register, offset: i32 },
    OpImm { kind: OpImmKind, rd: Register, rs1: Register, imm: i32 },
    OpImmShift { kind: ShiftKind, rd: Register, rs1: Register, shamt: u8 },
    Op { kind: OpKind, rd: Register, rs1: Register, rs2: Register },
    Fence { pred: u8, succ: u8 },
    FenceI, Ecall, Ebreak,
    Csr { kind: CsrKind, rd: Register, csr: u16, rs1: Option<Register>, zimm: Option<u8> },
    Unknown(u32),
}