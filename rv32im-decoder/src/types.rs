#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Register { X0 = 0, X1 = 1, X2 = 2, X3 = 3, X4 = 4, X5 = 5, X6 = 6, X7 = 7, X8 = 8, X9 = 9, X10 = 10, X11 = 11, X12 = 12, X13 = 13, X14 = 14, X15 = 15, X16 = 16, X17 = 17, X18 = 18, X19 = 19, X20 = 20, X21 = 21, X22 = 22, X23 = 23, X24 = 24, X25 = 25, X26 = 26, X27 = 27, X28 = 28, X29 = 29, X30 = 30, X31 = 31 }
impl Register { pub fn from_u8(value: u8) -> Option<Self> { if value < 32 { Some(unsafe { std::mem::transmute(value) }) } else { None } } pub const fn index(self) -> usize { self as usize } }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction { Lui, Auipc, Jal, Jalr, Beq, Bne, Blt, Bge, Bltu, Bgeu, Lb, Lh, Lw, Lbu, Lhu, Sb, Sh, Sw, Addi, Slti, Sltiu, Xori, Ori, Andi, Slli, Srli, Srai, Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And, Fence, Ecall, Ebreak, Mul, Mulh, Mulhsu, Mulhu, Div, Divu, Rem, Remu }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedInstruction { pub instruction: Instruction, pub rd: Option<Register>, pub rs1: Option<Register>, pub rs2: Option<Register>, pub imm: i32, pub raw: u32 }
impl DecodedInstruction { pub const fn new(instruction: Instruction, rd: Option<Register>, rs1: Option<Register>, rs2: Option<Register>, imm: i32, raw: u32) -> Self { Self { instruction, rd, rs1, rs2, imm, raw } } }
