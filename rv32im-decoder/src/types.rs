use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui { rd: u8, imm: u32 },
    Auipc { rd: u8, imm: u32 },
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

    Mul { rd: u8, rs1: u8, rs2: u8 },
    Mulh { rd: u8, rs1: u8, rs2: u8 },
    Mulhsu { rd: u8, rs1: u8, rs2: u8 },
    Mulhu { rd: u8, rs1: u8, rs2: u8 },
    Div { rd: u8, rs1: u8, rs2: u8 },
    Divu { rd: u8, rs1: u8, rs2: u8 },
    Rem { rd: u8, rs1: u8, rs2: u8 },
    Remu { rd: u8, rs1: u8, rs2: u8 },

    Fence,
    FenceI,
    Ecall,
    Ebreak
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    UnsupportedOpcode { word: u32, opcode: u8 },
    UnsupportedFunct3 { word: u32, opcode: u8, funct3: u8 },
    UnsupportedFunct7 { word: u32, funct3: u8, funct7: u8 },
    InvalidShiftEncoding { word: u32, funct3: u8, funct7: u8 },
    InvalidFenceEncoding { word: u32 },
    InvalidSystemEncoding { word: u32 }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOpcode { word, opcode } => {
                write!(f, "unsupported opcode 0b{opcode:07b} in word 0x{word:08x}")
            }
            Self::UnsupportedFunct3 {
                word,
                opcode,
                funct3,
            } => write!(
                f,
                "unsupported funct3 0b{funct3:03b} for opcode 0b{opcode:07b} in word 0x{word:08x}"
            ),
            Self::UnsupportedFunct7 {
                word,
                funct3,
                funct7,
            } => write!(
                f,
                "unsupported funct7 0b{funct7:07b} for funct3 0b{funct3:03b} in word 0x{word:08x}"
            ),
            Self::InvalidShiftEncoding {
                word,
                funct3,
                funct7,
            } => write!(
                f,
                "invalid shift encoding funct3=0b{funct3:03b} funct7=0b{funct7:07b} in word 0x{word:08x}"
            ),
            Self::InvalidFenceEncoding { word } => {
                write!(f, "invalid fence encoding in word 0x{word:08x}")
            }
            Self::InvalidSystemEncoding { word } => {
                write!(f, "invalid system encoding in word 0x{word:08x}")
            }
        }
    }
}

impl std::error::Error for DecodeError {}
