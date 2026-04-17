#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    // RV32I subset
    Add { rd: u8, rs1: u8, rs2: u8 },
    Sub { rd: u8, rs1: u8, rs2: u8 },
    Addi { rd: u8, rs1: u8, imm: i32 },
    Lui { rd: u8, imm: i32 },
    Jal { rd: u8, imm: i32 },

    // RV32M extension
    Mul { rd: u8, rs1: u8, rs2: u8 },
    Mulh { rd: u8, rs1: u8, rs2: u8 },
    Mulhsu { rd: u8, rs1: u8, rs2: u8 },
    Mulhu { rd: u8, rs1: u8, rs2: u8 },
    Div { rd: u8, rs1: u8, rs2: u8 },
    Divu { rd: u8, rs1: u8, rs2: u8 },
    Rem { rd: u8, rs1: u8, rs2: u8 },
    Remu { rd: u8, rs1: u8, rs2: u8 },

    Ecall,
    Ebreak,
    Invalid(u32),
}