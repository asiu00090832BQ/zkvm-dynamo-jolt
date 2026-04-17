use core::fmt;
pub type Register = u8;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    Lui { rd: Register, imm: u32 },
    Auipc { rd: Register, imm: u32 },
    Jal { rd: Register, imm: i32 },
    Jalr { rd: Register, rs1: Register, imm: i32 },
    Branch { op: BranchOp, rs1: Register, rs2: Register, imm: i32 },
    Load { op: LoadOp, rd: Register, rs1: Register, imm: i32 },
    Store { op: StoreOp, rs1: Register, rs2: Register, imm: i32 },
    OpImm { op: OpImmOp, rd: Register, rs1: Register, imm: i32, shamt: u8 },
    Op { op: RegOp, rd: Register, rs1: Register, rs2: Register },
    M { op: Rv32mOp, rd: Register, rs1: Register, rs2: Register },
    Fence, FenceI, Ecall, Ebreak,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BranchOp { Beq, Bne, Blt, Bge, Bltu, Bgeu }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoadOp { Lb, Lh, Lw, Lbu, Lhu }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreOp { Sb, Sh, Sw }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpImmOp { Addi, Slti, Sltiu, Xori, Ori, Andi, Slli, Srli, Srai }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegOp { Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rv32mOp { Mul, Mulh, Mulhsu, Mulhu, Div, Divu, Rem, Remu }
