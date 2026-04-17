use core::fmt;

pub type Register = u8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTypeFields { pub rd: Register, pub rs1: Register, pub rs2: Register }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ITypeFields { pub rd: Register, pub rs1: Register, pub imm: i32 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STypeFields { pub rs1: Register, pub rs2: Register, pub imm: i32 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BTypeFields { pub rs1: Register, pub rs2: Register, pub imm: i32 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UTypeFields { pub rd: Register, pub imm: i32 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JTypeFields { pub rd: Register, pub imm: i32 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShiftImmediateFields { pub rd: Register, pub rs1: Register, pub shamt: u8 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FenceFields { pub pred: u8, pub succ: u8, pub fm: u8 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CsrFields { pub rd: Register, pub rs1: Register, pub csr: u16 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CsrImmediateFields { pub rd: Register, pub zimm: u8, pub csr: u16 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    Lui(UTypeFields), Auipc(UTypeFields), Jal(JTypeFields), Jalr(ITypeFields),
    Beq(BTypeFields), Bne(BTypeFields), Blt(BTypeFields), Bge(BTypeFields), Bltu(BTypeFields), Bgeu(BTypeFields),
    Lb(ITypeFields), Lh(ITypeFields), Lw(ITypeFields), Lbu(ITypeFields), Lhu(ITypeFields),
    Sb(STypeFields), Sh(STypeFields), Sw(STypeFields),
    Addi(ITypeFields), Slti(ITypeFields), Sltiu(ITypeFields), Xori(ITypeFields), Ori(ITypeFields), Andi(ITypeFields),
    Slli(ShiftImmediateFields), Srli(ShiftImmediateFields), Srai(ShiftImmediateFields),
    Add(RTypeFields), Sub(RTypeFields), Sll(RTypeFields), Slt(RTypeFields), Sltu(RTypeFields), Xor(RTypeFields), Srl(RTypeFields), Sra(RTypeFields), Or(RTypeFields), And(RTypeFields),
    Fence(FenceFields), FenceI(ITypeFields),
    Ecall, Ebreak, Csrrw(CsrFields), Csrrs(CsrFields), Csrrc(CsrFields), Csrrwi(CsrImmediateFields), Csrrsi(CsrImmediateFields), Csrrci(CsrImmediateFields),
    Mul(RTypeFields), Mulh(RTypeFields), Mulhsu(RTypeFields), Mulhu(RTypeFields), Div(RTypeFields), Divu(RTypeFields), Rem(RTypeFields), Remu(RTypeFields),
}

impl fmt::Display for Instruction { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") } }
