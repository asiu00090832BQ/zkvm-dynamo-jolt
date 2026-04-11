#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    LUI { rd: usize, imm: i32 },
    AUIPC { rd: usize, imm: i32 },
    JAL { rd: usize, imm: i32 },
    JALR { rd: usize, rs1: usize, imm: i32 },
    BEQ { rs1: usize, rs2: usize, imm: i32 },
    BNE { rs1: usize, rs2: usize, imm: i32 },
    BLT { rs1: usize, rs2: usize, imm: i32 },
    BGE { rs1: usize, rs2: usize, imm: i32 },
    BLTU { rs1: usize, rs2: usize, imm: i32 },
    BGEU { rs1: usize, rs2: usize, imm: i32 },
    LB { rd: usize, rs1: usize, imm: i32 },
    LH { rd: usize, rs1: usize, imm: i32 },
    LW { rd: usize, rs1: usize, imm: i32 },
    LBU { rd: usize, rs1: usize, imm: i32 },
    LHU { rd: usize, rs1: usize, imm: i32 },
    SB { rs1: usize, rs2: usize, imm: i32 },
    SH { rs1: usize, rs2: usize, imm: i32 },
    SW { rs1: usize, rs2: usize, imm: i32 },
    ADDI { rd: usize, rs1: usize, imm: i32 },
    SLTI { rd: usize, rs1: usize, imm: i32 },
    SLTIU { rd: usize, rs1: usize, imm: i32 },
    XORI { rd: usize, rs1: usize, imm: i32 },
    ORI { rd: usize, rs1: usize, imm: i32 },
    ANDI { rd: usize, rs1: usize, imm: i32 },
    SLLI { rd: usize, rs1: usize, shamt: u32 },
    SRLI { rd: usize, rs1: usize, shamt: u32 },
    SRAI { rd: usize, rs1: usize, shamt: u32 },
    ADD { rd: usize, rs1: usize, rs2: usize },
    SUB { rd: usize, rs1: usize, rs2: usize },
    SLL { rd: usize, rs1: usize, rs2: usize },
    SLT { rd: usize, rs1: usize, rs2: usize },
    SLTU { rd: usize, rs1: usize, rs2: usize },
    XOR { rd: usize, rs1: usize, rs2: usize },
    SRL { rd: usize, rs1: usize, rs2: usize },
    SRA { rd: usize, rs1: usize, rs2: usize },
    OR { rd: usize, rs1: usize, rs2: usize },
    AND { rd: usize, rs1: usize, rs2: usize },
    MUL { rd: usize, rs1: usize, rs2: usize },
    MULH { rd: usize, rs1: usize, rs2: usize },
    MULHSU { rd: usize, rs1: usize, rs2: usize },
    MULHU { rd: usize, rs1: usize, rs2: usize },
    DIV { rd: usize, rs1: usize, rs2: usize },
    DIVU { rd: usize, rs1: usize, rs2: usize },
    REM { rd: usize, rs1: usize, rs2: usize },
    REMU { rd: usize, rs1: usize, rs2: usize },
    FENCE,
    FENCEI,
    ECALL,
    EBREAK,
    INVALID(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Invalid(u32),
    Unsupported(u32),
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = word & 0x7f;
    match opcode {
        0x73 => Ok(Instruction::ECALL),
        _ => Ok(Instruction::INVALID(word)),
    }
}
