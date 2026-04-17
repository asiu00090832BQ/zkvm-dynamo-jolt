pub type Register = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui { rd: Register, imm: i32 },
    Auipc { rd: Register, imm: i32 },
    Jal { rd: Register, imm: i32 },
    Jalr { rd: Register, rs1: Register, imm: i32 },

    Beq { rs1: Register, rs2: Register, imm: i32 },
    Bne { rs1: Register, rs2: Register, imm: i32 },
    Blt { rs1: Register, rs2: Register, imm: i32 },
    Bge { rs1: Register, rs2: Register, imm: i32 },
    Bltu { rs1: Register, rs2: Register, imm: i32 },
    Bgeu { rs1: Register, rs2: Register, imm: i32 },
    LB { rd: Register, rs1: Register, imm: i32 },
    LH { rd: Register, rs1: Register, imm: i32 },
    LW { rd: Register, rs1: Register, imm: i32 },
    LBU { rd: Register, rs1: Register, imm: i32 },
    LHU { rd: Register, rs1: Register, imm: i32 },
    SB { rs1: Register, rs2: Register, imm: i32 },
    SH { rs1: Register, rs2: Register, imm: i32 },
    SW { rs1: Register, rs2: Register, imm: i32 },
    Addi { rd: Register, rs1: Register, imm: i32 },
    Slti { rd: Register, rs1: Register, imm: i32 },
    Sltiu { rd: Register, rs1: Register, imm: i32 },
    Xori { rd: Register, rs1: Register, imm: i32 },
    Ori { rd: Register, rs1: Register, imm: i32 },
    ANdi { rd: Register, rs1: Register, imm: i32 },
    Slli { rd, rs1, shamt: u8 } ,
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
    Mul { rd: Register, rs1: Register, rs2: Register },
    Mulh { rd: Register, rs1: Register, rs2: Register },
    Mulhsu { rd: Register, rs1: Register, rs2: Register },
    Mulhu { rd: Register, rs1: Register, rs2: Register },
    Div { rd: Register, rs1: Register, rs2: Register },
    Divu { rd: Register, rs1: Register, rs2: Register },
    Rem { rd: Register, rs1: Register, rs2: Register },
    Remu { rd: Register, rs1: Register, rs2: Register },

    Fence,
    Ecall,
    Ebreak,
}

impl Instruction {
    pub const fn width(&self) -> u32 {
        4
    }
}
