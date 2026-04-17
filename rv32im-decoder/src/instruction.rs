use core::fmt;

/// Logical register index (0–31).
pub type Reg = u8;

/// Decoded RV32I/M instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    // U-type
    Lui { rd: Reg, imm: i32 },
    Auipc { rd: Reg, imm: i32 },

    // J-type
    Jal { rd: Reg, imm: i32 },

    // I-type
    Jalr { rd: Reg, rs1: Reg, imm: i32 },
    Load { funct3: u8, rd: Reg, rs1: Reg, imm: i32 },

    // B-type
    Branch { funct3: u8, rs1: Reg, rs2: Reg, imm: i32 },

    // S-type
    Store { funct3: u8, rs1: Reg, rs2: Reg, imm: i32 },

    // R-type integer ALU
    Add { rd: Reg, rs1: Reg, rs2: Reg },
    Sub { rd: Reg, rs1: Reg, rs2: Reg },
    Sll { rd: Reg, rs1: Reg, rs2: Reg },
    Slt { rd: Reg, rs1: Reg, rs2: Reg },
    Sltu { rd: Reg, rs1: Reg, rs2: Reg },
    Xor { rd: Reg, rs1: Reg, rs2: Reg },
    Srl { rd: Reg, rs1: Reg, rs2: Reg },
    Sra { rd: Reg, rs1: Reg, rs2: Reg },
    Or { rd: Reg, rs1: Reg, rs2: Reg },
    And { rd: Reg, rs1: Reg, rs2: Reg },

    /// RV32M multiply/divide/remainder instructions.
    Mul { rd: Reg, rs1: Reg, rs2: Reg },
    Mulh { rd: Reg, rs1: Reg, rs2: Reg },
    Mulhsu { rd: Reg, rs1: Reg, rs2: Reg },
    Mulhu { rd: Reg, rs1: Reg, rs2: Reg },
    Div { rd: Reg, rs1: Reg, rs2: Reg },
    Divu { rd: Reg, rs1: Reg, rs2: Reg },
    Rem { rd: Reg, rs1: Reg, rs2: Reg },
    Remu { rd: Reg, rs1: Reg, rs2: Reg },

    // Misc system
    Fence,
    Ecall,
    Ebreak,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;
        match *self {
            Lui { rd, imm } => write!(f, "lui x{}, {:#x}", rd, imm),
            Auipc { rd, imm } => write!(f, "auipc x{}, {:#x}", rd, imm),
            Jal { rd, imm } => write!(f, "jal x{}, {:#x}", rd, imm),
            Jalr { rd, rs1, imm } => write!(f, "jalr x{}, x{}, {:#x}", rd, rs1, imm),
            Load { funct3, rd, rs1, imm } => {
                write!(f, "load(f3={:#x}) x{}, {:#x}(x{})", funct3, rd, imm, rs1)
            }
            Branch { funct3, rs1, rs2, imm } => {
                write!(f, "branch(f3={:#x}) x{}, x{}, {:#x}", funct3, rs1, rs2, imm)
            }
            Store { funct3, rs1, rs2, imm } => {
                write!(f, "store(f3={:#x}) x{}, x{}, {:#x}", funct3, rs1, rs2, imm)
            }
            Add { rd, rs1, rs2 } => write!(f, "add x{}, x{}, x{}", rd, rs1, rs2),
            Sub { rd, rs1, rs2 } => write!(f, "sub x{}, x{}, x{}", rd, rs1, rs2),
            Sll { rd, rs1, rs2 } => write!(f, "sll x{}, x{}, x{}", rd, rs1, rs2),
            Slt { rd, rs1, rs2 } => write!(f, "slt x{}, x{}, x{}", rd, rs1, rs2),
            Sltu { rd, rs1, rs2 } => write!(f, "sltu x{}, x{}, x{}", rd, rs1, rs2),
            Xor { rd, rs1, rs2 } => write!(f, "xor x{}, x{}, x{}", rd, rs1, rs2),
            Srl { rd, rs1, rs2 } => write!(f, "srl x{}, x{}, x{}", rd, rs1, rs2),
            Sra { rd, rs1, rs2 } => write!(f, "sra x{}, x{}, x{}", rd, rs1, rs2),
            Or { rd, rs1, rs2 } => write!(f, "or x{}, x{}, x{}", rd, rs1, rs2),
            And { rd, rs1, rs2 } => write!(f, "and x{}, x{}, x{}", rd, rs1, rs2),
            Mul { rd, rs1, rs2 } => write!(f, "mul x{}, x{}, x{}", rd, rs1, rs2),
            Mulh { rd, rs1, rs2 } => write!(f, "mulh x{}, x{}, x{}", rd, rs1, rs2),
            Mulhsu { rd, rs1, rs2 } => write!(f, "mulhsu x{}, x{}, x{}", rd, rs1, rs2),
            Mulhu { rd, rs1, rs2 } => write!(f, "mulhu x{}, x{}, x{}", rd, rs1, rs2),
            Div { rd, rs1, rs2 } => write!(f, "div x{}, x{}, x{}", rd, rs1, rs2),
            Divu { rd, rs1, rs2 } => write!(f, "divu x{}, x{}, x{}", rd, rs1, rs2),
            Rem { rd, rs1, rs2 } => write!(f, "rem x{}, x{}, x{}", rd, rs1, rs2),
            Remu { rd, rs1, rs2 } => write!(f, "remu x{}, x{}, x{}", rd, rs1, rs2),
            Fence => write!(f, "fence"),
            Ecall => write!(f, "ecall"),
            Ebreak => write!(f, "ebreak"),
        }
    }
}
