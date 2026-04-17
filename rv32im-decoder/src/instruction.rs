use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui { rd: u8, imm: i32 },
    Auipc { rd: u8, imm: i32 },
    Jal { rd: u8, imm: i32 },
    Jalr { rd: u8, rs1: u8, imm: i32 },

    Beq { rs1: u8, rs2: u8, imm: i32 },
    Bne { rs1: u8, rs2: u8, imm: i32 },
    Blt { rs1: u8, rs2: u8, imm: i32 },
    Bge { rs1: u8, rs2: u8, imm: i32 },
    Bltu { rs1: u8, rs2: u8, imm: i32 },
    Bgeu { rs1: u8, rs2: u8, imm: i32 },

    Lb { rd: u8, rs1: u8, imm: i32 },
    Lh { rd: u8, rs1: u8, imm: i32 },
    Lw { rd: u8, rs1: u8, imm: i32 },
    Lbu { rd: u8, rs1: u8, imm: i32 },
    Lhu { rd: u8, rs1: u8, imm: i32 },

    Sb { rs1: u8, rs2: u8, imm: i32 },
    Sh { rs1: u8, rs2: u8, imm: i32 },
    Sw { rs1: u8, rs2: u8, imm: i32 },

    Addi { rd: u8, rs1: u8, imm: i32 },
    Slti { rd: u8, rs1: u8, imm: i32 },
    Sltiu { rd: u8, rs1: u8, imm: i32 },
    Xori { rd: u8, rs1: u8, imm: i32 },
    Ori { rd: u8, rs1: u8, imm: i32 },
    Andi { rd: u8, rs1: u8, imm: i32 },
    Slli { rd: u8, rs1: u8, shamt: u8 },
    Srli { rd: u8, rs1: u8, shamt: u8 },
    Srai { rd: u8, rs1: u8, shamt: u8 },

    Add { rd: u8, rs1: u8, rs2: u8 },
    Sub { rd: u8, rs1: u8, rs2: u8 },
    Sll { rd: u8, rs1: u8, rs2: u8 },
    Slt { rd: u8, rs1: u8, rs2: u8 },
    Sltu { rd: u8, rs1: u8, rs2: u8 },
    Xor { rd: u8, rs1: u8, rs2: u8 },
    Srl { rd: u8, rs1: u8, rs2: u8 },
    Sra { rd: u8, rs1: u8, rs2: u8 },
    Or { rd: u8, rs1: u8, rs2: u8 },
    And { rd: u8, rs1: u8, rs2: u8 },

    Fence { fm: u8, pred: u8, succ: u8 },
    Ecall,
    Ebreak,

    Mul { rd: u8, rs1: u8, rs2: u8 },
    Mulh { rd: u8, rs1: u8, rs2: u8 },
    Mulhsu { rd: u8, rs1: u8, rs2: u8 },
    Mulhu { rd: u8, rs1: u8, rs2: u8 },
    Div { rd: u8, rs1: u8, rs2: u8 },
    Divu { rd: u8, rs1: u8, rs2: u8 },
    Rem { rd: u8, rs1: u8, rs2: u8 },
    Remu { rd: u8, rs1: u8, rs2: u8 },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lui { rd, imm } => write!(f, "lui x{rd}, 0x{:08x}", *imm as u32),
            Self::Auipc { rd, imm } => write!(f, "auipc x{rd}, 0x{:08x}", *imm as u32),
            Self::Jal { rd, imm } => write!(f, "jal x{rd}, {imm}"),
            Self::Jalr { rd, rs1, imm } => write!(f, "jalr x{rd}, {imm}(x{rs1})"),

            Self::Beq { rs1, rs2, imm } => write!(f, "beq x{rs1}, x{rs2}, {imm}"),
            Self::Bne { rs1, rs2, imm } => write!(f, "bne x{rs1}, x{rs2}, {imm}"),
            Self::Blt { rs1, rs2, imm } => write!(f, "blt x{rs1}, x{rs2}, {imm}"),
            Self::Bge { rs1, rs2, imm } => write!(f, "bge x{rs1}, x{rs2}, {imm}"),
            Self::Bltu { rs1, rs2, imm } => write!(f, "bltu x{rs1}, x{rs2}, {imm}"),
            Self::Bgeu { rs1, rs2, imm } => write!(f, "bgeu x{rs1}, x{rs2}, {imm}"),

            Self::Lb { rd, rs1, imm } => write!(f, "lb x{rd}, {imm}(x{rs1})"),
            Self::Lh { rd, rs1, imm } => write!(f, "lh x{rd}, {imm}(x{rs1})"),
            Self::Lw { rd, rs1, imm } => write!(f, "lw x{rd}, {imm}(x{rs1})"),
            Self::Lbu { rd, rs1, imm } => write!(f, "lbu x{rd}, {imm}(x{rs1})"),
            Self::Lhu { rd, rs1, imm } => write!(f, "lhu x{rd}, {imm}(x{rs1})"),

            Self::Sb { rs1, rs2, imm } => write!(f, "sb x{rs2}, {imm}(x{rs1})"),
            Self::Sh { rs1, rs2, imm } => write!(f, "sh x{rs2}, {imm}(x{rs1})"),
            Self::Sw { rs1, rs2, imm } => write!(f, "sw x{rs2}, {imm}(x{rs1})"),

            Self::Addi { rd, rs1, imm } => write!(f, "addi x{rd}, x{rs1}, {imm}"),
            Self::Slti { rd, rs1, imm } => write!(f, "slti x{rd}, x{rs1}, {imm}"),
            Self::Sltiu { rd, rs1, imm } => write!(f, "sltiu x{rd}, x{rs1}, {imm}"),
            Self::Xori { rd, rs1, imm } => write!(f, "xori x{rd}, x{rs1}, {imm}"),
            Self::Ori { rd, rs1, imm } => write!(f, "ori x{rd}, x{rs1}, {imm}"),
            Self::Andi { rd, rs1, imm } => write!(f, "andi x{rd}, x{rs1}, {imm}"),
            Self::Slli { rd, rs1, shamt } => write!(f, "slli x{rd}, x{rs1}, {shamt}"),
            Self::Srli { rd, rs1, shamt } => write!(f, "srli x{rd}, x{rs1}, {shamt}"),
            Self::Srai { rd, rs1, shamt } => write!(f, "srai x{rd}, x{rs1}, {shamt}"),

            Self::Add { rd, rs1, rs2 } => write!(f, "add x{rd}, x{rs1}, x{rs2}"),
            Self::Sub { rd, rs1, rs2 } => write!(f, "sub x{rd}, x{rs1}, x{rs2}"),
            Self::Sll { rd, rs1, rs2 } => write!(f, "sll x{rd}, x{rs1}, x{rs2}"),
            Self::Slt { rd, rs1, rs2 } => write!(f, "slt x{rd}, x{rs1}, x{rs2}"),
            Self::Sltu { rd, rs1, rs2 } => write!(f, "sltu x{rd}, x{rs1}, x{rs2}"),
            Self::Xor { rd, rs1, rs2 } => write!(f, "xor x{rd}, x{rs1}, x{rs2}"),
            Self::Srl { rd, rs1, rs2 } => write!(f, "srl x{rd}, x{rs1}, x{rs2}"),
            Self::Sra { rd, rs1, rs2 } => write!(f, "sra x{rd}, x{rs1}, x{rs2}"),
            Self::Or { rd, rs1, rs2 } => write!(f, "or x{rd}, x{rs1}, x{rs2}"),
            Self::And { rd, rs1, rs2 } => write!(f, "and x{rd}, x{rs1}, x{rs2}"),

            Self::Fence { fm, pred, succ } => {
                write!(f, "fence fm={fm}, pred={pred}, succ={succ}")
            }
            Self::Ecall => write!(f, "ecall"),
            Self::Ebreak => write!(f, "ebreak"),

            Self::Mul { rd, rs1, rs2 } => write!(f, "mul x{rd}, x{rs1}, x{rs2}"),
            Self::Mulh { rd, rs1, rs2 } => write!(f, "mulh x{rd}, x{rs1}, x{rs2}"),
            Self::Mulhsu { rd, rs1, rs2 } => write!(f, "mulhsu x{rd}, x{rs1}, x{rs2}"),
            Self::Mulhu { rd, rs1, rs2 } => write!(f, "mulhu x{rd}, x{rs1}, x{rs2}"),
            Self::Div { rd, rs1, rs2 } => write!(f, "div x{rd}, x{rs1}, x{rs2}"),
            Self::Divu { rd, rs1, rs2 } => write!(f, "divu x{rd}, x{rs1}, x{rs2}"),
            Self::Rem { rd, rs1, rs2 } => write!(f, "rem x{rd}, x{rs1}, x{rs2}"),
            Self::Remu { rd, rs1, rs2 } => write!(f, "remu x{rd}, x{rs1}, x{rs2}"),
        }
    }
}
