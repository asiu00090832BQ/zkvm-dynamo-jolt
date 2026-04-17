use std::fmt;

use crate::fields::{BType, IType, JType, RType, SType, UType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchOp {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadOp {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreOp {
    Sb,
    Sh,
    Sw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpImmOp {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticOp {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemOp {
    Ecall,
    Ebreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui(UType),
    Auipc(UType),
    Jal(JType),
    Jalr(IType),
    Branch(BranchOp, BType),
    Load(LoadOp, IType),
    Store(StoreOp, SType),
    OpImm(OpImmOp, IType),
    Op(ArithmeticOp, RType),
    Fence,
    System(SystemOp),
}

impl Instruction {
    pub fn mnemonic(&self) -> &'static str {
        match self {
            Instruction::Lui(_) => "lui",
            Instruction::Auipc(_) => "auipc",
            Instruction::Jal(_) => "jal",
            Instruction::Jalr(_) => "jalr",
            Instruction::Branch(op, _) => match op {
                BranchOp::Beq => "beq",
                BranchOp::Bne => "bne",
                BranchOp::Blt => "blt",
                BranchOp::Bge => "bge",
                BranchOp::Bltu => "bltu",
                BranchOp::Bgeu => "bgeu",
            },
            Instruction::Load(op, _) => match op {
                LoadOp::Lb => "lb",
                LoadOp::Lh => "lh",
                LoadOp::Lw => "lw",
                LoadOp::Lbu => "lbu",
                LoadOp::Lhu => "lhu",
            },
            Instruction::Store(op, _) => match op {
                StoreOp::Sb => "sb",
                StoreOp::Sh => "sh",
                StoreOp::Sw => "sw",
            },
            Instruction::OpImm(op, _) => match op {
                OpImmOp::Addi => "addi",
                OpImmOp::Slti => "slti",
                OpImmOp::Sltiu => "sltiu",
                OpImmOp::Xori => "xori",
                OpImmOp::Ori => "ori",
                OpImmOp::Andi => "andi",
                OpImmOp::Slli => "slli",
                OpImmOp::Srli => "srli",
                OpImmOp::Srai => "srai",
            },
            Instruction::Op(op, _) => match op {
                ArithmeticOp::Add => "add",
                ArithmeticOp::Sub => "sub",
                ArithmeticOp::Sll => "sll",
                ArithmeticOp::Slt => "slt",
                ArithmeticOp::Sltu => "sltu",
                ArithmeticOp::Xor => "xor",
                ArithmeticOp::Srl => "srl",
                ArithmeticOp::Sra => "sra",
                ArithmeticOp::Or => "or",
                ArithmeticOp::And => "and",
                ArithmeticOp::Mul => "mul",
                ArithmeticOp::Mulh => "mulh",
                ArithmeticOp::Mulhsu => "mulhsu",
                ArithmeticOp::Mulhu => "mulhu",
                ArithmeticOp::Div => "div",
                ArithmeticOp::Divu => "divu",
                ArithmeticOp::Rem => "rem",
                ArithmeticOp::Remu => "remu",
            },
            Instruction::Fence => "fence",
            Instruction::System(op) => match op {
                SystemOp::Ecall => "ecall",
                SystemOp::Ebreak => "ebreak",
            },
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.mnemonic())
    }
}
