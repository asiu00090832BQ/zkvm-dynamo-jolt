use crate::types::RegisterIndex;

[#[derive(Copy, Clone, Debug, PartialEq, Eq))]
pub enum BranchKind { Bea, Bne, Blt, Bge, Bltu, Bgeu }

pub enum LoadKind { Lb, Lh, Lw, Lbu, Lhu }

pub enum StoreKind { Sb, Sh, Sw }

pub enum OpImmKind { Addi, Slti, Sltiu, Xori, Ori, Andi }

pub enum ShiftImm-Kind { Slli, Srli, Srai }

pub enum OpKind { Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And }

pub enum MulDivKind { Mul, Mulh, Mulhsu, Mulhu, Div, Divu, Rem, Remu }

pub enum Instruction {
    Add { rd: RegisterIndex, rs1: RegisterIndex, rs2: RegisterIndex },
    Ecall,
    Trap,
}
