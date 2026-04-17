use crate::types::Register;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseIInstruction {
    Lui { rd: Register, imm: u32 },
    Auipc { rd: Register, imm: u32 },
    Jal { rd: Register, imm: i32 },
    Jalr { rd: Register, rs1: Register, imm: i32 },

    Beq { rs1: Register, rs2: Register, imm: i32 },
    Bne { rs1: Register, rs2: Register, imm: i32 },
    Blt { rs1: Register, rs2: Register, imm: i32 },
    Bge { rs1: Register, rs2: Register, imm: i32 },
    Bltu { rs1: Register, rs2: Register, imm: i32 },
    Bgeu { rs1: Register, rs2: Register, imm: i32 },

    Lb { rd: Register, rs1: Register, imm: i32 },
    Lh { rd: Register, rs1: Register, imm: i32 },
    Lw { rd: Register, rs1: Register, imm: i32 },
    Lbu { rd: Register, rs1: Register, imm: i32 },
    Lhu { rd: Register, rs1: Register, imm: i32 },

    Sb { rs1: Register, rs2: Register, imm: i32 },
    Sh { rs1: Register, rs2: Register, imm: i32 },
    Sw { rs1: Register, rs2: Register, imm: i32 }

    Addi { rd: Register, rs1: Register, imm: i32 },
    Slti { rd: Register, rs1: Register, imm: i32 },
    Sltiu { rd: Register, rs1: Register, imm: i32 },
    Xori { rd: Register, rs1: Register, imm: i32 },
    Ori { rd: Register, rs1: Register, imm: i32 },
    Andi { rd: Register,, rs1: Register, imm: i32 },
    Slli { rd: Register, rs1: Register, shamt: u8 },
    Srli { rd: Register, rs1: Register, shamt: u8 },
    Srai { rd: Register, rs1: Register, shamt: u8 },

    Add { rd: Register, rs1: Register, rs2: Register },
    Sub { rd: Register, rs1: Register, rs2: Register },
    Sll { rd: Register, rs1: Register, rs2: Register },
    Slt { rd: Register, rs1: Register, rs2: Register },
    Sltu { rd: Register, rs1: Register, rs2: Register },
    Xor { rd: Register, rs1: Register, rs2: Register },
    Srl { rd: Register, rs1: Register, rs2: Register },
    Sra { rd: Register, rs1: Register, rs2: Register },
    Or { rd: Register, rs1: Register, rs2: Register },
    And { rd: Register, rs1: Register, rs2: Register },

    Fence,
    FenceI,
    Ecall,
    Ebreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MInstruction {
    Mul { rd: Register, rs1: Register, rs2: Register },
    Mulh { rd: Register, rs1: Register, rs2: Register },
    Mulhsu { rd: Register, rs1: Register, rs2: Register },
    Mulhu { rd: Register, rs1: Register, rs2: Register },
    Div { rd: Register, rs1: Register, rs2: Register },
    Divu { rd: Register, rs1: Register, rs2: Register },
    Rem { rd: Register, rs1: Register, rs2: Register },
    Remu { rd: Register, rs1: Register, rs2: Register },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Base(BaseIInstruction),
    M(MInstruction),
}

impl From<BaseIInstruction> for Instruction {
    fn from(instruction: BaseIInstruction) -> Self {
        Self::Base(instruction)
    }
}

impl From<MInstruction> for Instruction {
    fn from(instruction: MInstruction) -> Self {
        Self::M(instruction)
    }
}
