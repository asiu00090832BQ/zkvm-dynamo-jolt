use crate::isa::{
    BTypeFields, ITypeFields, JTypeFields, RTypeFields, Register, STypeFields, ShiftImmFields,
    UTypeFields,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ecall;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sub {
    pub rd: Register,
    pub rs1: Register,
    pub rs2: Register,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rv32I {
    Lui(UTypeFields),
    Auipc(UTypeFields),
    Jal(JTypeFields),
    Jalr(ITypeFields),
    Beq(BTypeFields),
    Bne(BTypeFields),
    Blt(BTypeFields),
    Bge(BTypeFields),
    Bltu(BTypeFields),
    Bgeu(BTypeFields),
    Lb(ITypeFields),
    Lh(ITypeFields),
    Lw(ITypeFields),
    Lbu(ITypeFields),
    Lhu(ITypeFields),
    Sb(STypeFields),
    Sh(STypeFields),
    Sw(STypeFields),
    Addi(ITypeFields),
    Slti(ITypeFields),
    Sltiu(ITypeFields),
    Xori(ITypeFields),
    Ori(ITypeFields),
    Andi(ITypeFields),
    Slli(ShiftImmFields),
    Srli(ShiftImmFields),
    Srai(ShiftImmFields),
    Add(RTypeFields),
    Sub(Sub),
    Sll(RTypeFields),
    Slt(RTypeFields),
    Sltu(RTypeFields),
    Xor(RTypeFields),
    Srl(RTypeFields),
    Sra(RTypeFields),
    Or(RTypeFields),
    And(RTypeFields),
    Fence,
    FenceI,
    Ecall(Ecall),
    Ebreak,
}
