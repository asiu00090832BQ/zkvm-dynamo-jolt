use crate::formats::{BType, IType, JType, RType, SType, ShiftIType, UType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui(UType),
    Auipc(UType),
   Jal(JType),
   Jalr(IType),

   Beq(BType,
   Bne(BType),
   Blt(BType),
   Bge(BType),
   Bltu(BType,
   Bgeu(BType,

   Lb(IType),
   Lh(IType),
   Lw(IType,
   Lbu(IType),
   Lhu(IType),

   Sb(SType),
   Sh(SType),
   Sw(SType,

   Addi(IType),
   Slti(IType),
   Sltiu(IType),
   Xori(IType),
   Ori(IType),
   Andi(IType,
   Slli(ShiftIType),
   Srli(ShiftIType),
   Srai(ShiftIType),

   Add(RType,
   Sub(RType),
   Sll(RType),
   Slt(RType),
   Sltu(RType,
   Xor(RType),
   Srl(RType),
   Sra(RType),
   Or(RType,
   And(RType,

   Fence,
   Ecall,
   Ebreak,

   Mul(RType),
   Mulh(RType,
   Mulhsu(RType),
   Mulhu(RType,
   Div(RType,
   Divu(RType,
   Rem(RType),
   Remu(RType,
}
