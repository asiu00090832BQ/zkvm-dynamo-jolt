use crate::error::DecodeError;

pub const REGISTER_COUNT: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Register {
    X0 = 0,
    X1 = 1,
    X2 = 2,
    X3 = 3,
    X4 = 4,
    X5 = 5,
    X6 = 6,
    X7 = 7,
    X8 = 8,
    X9 = 9,
    X10 = 10,
    X11 = 11,
    X12 = 12,
    X13 = 13,
    X14 = 14,
    X15 = 15,
    X16 = 16,
    X17 = 17,
    X18 = 18,
    X19 = 19,
    X20 = 20,
    X21 = 21,
    X22 = 22,
    X23 = 23,
    X24 = 24,
    X25 = 25,
    X26 = 26,
    X27 = 27,
    X28 = 28,
    X29 = 29,
    X30 = 30,
    X31 = 31,
}

impl Register {
    pub const fn index(self) -> usize {
        self as usize
    }
}

impl TryFrom<u8> for Register {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::X0),
            1 => Ok(Self::X1),
            2 => Ok(Self::X2),
            3 => Ok(Self::X3),
            4 => Ok(Self::X4),
            5 => Ok(Self::X5),
            6 => Ok(Self::X6),
            7 => Ok(Self::X7),
            8 => Ok(Self::X8),
            9 => Ok(Self::X9),
            10 => Ok(Self::X10),
            11 => Ok(Self::X11),
            12 => Ok(Self::X12),
            13 => Ok(Self::X13),
            14 => Ok(Self::X14),
            15 => Ok(Self::X15),
            16 => Ok(Self::X16),
            17 => Ok(Self::X17),
            18 => Ok(Self::X18),
            19 => Ok(Self::X19),
            20 => Ok(Self::X20),
            21 => Ok(Self::X21),
            22 => Ok(Self::X22),
            23 => Ok(Self::X23),
            24 => Ok(Self::X24),
            25 => Ok(Self::X25),
            26 => Ok(Self::X26),
            27 => Ok(Self::X27),
            28 => Ok(Self::X28),
            29 => Ok(Self::X29),
            30 => Ok(Self::X30),
            31 => Ok(Self::X31),
            _ => Err(DecodeError::InvalidRegister(value)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    Lb { rd: Register, rs1: Register, imm: i32 },
    Lh { rd: Register, rs1: Register, imm: i32 },
    Lw { rd: Register, rs1: Register, imm: i32 },
    Lbu { rd: Register, rs1: Register, imm: i32 },
    Lhu { rd: Register, rs1: Register, imm: i32 },

    Sb { rs1: Register, rs2: Register, imm: i32 },
    Sh { rs1: Register, rs2: Register, imm: i32 },
    Sw { rs1: Register, rs2: Register, imm: i32 },

    Addi { rd: Register, rs1: Register, imm: i32 },
    Slti { rd: Register, rs1: Register, imm: i32 },
    Sltiu { rd: Register, rs1: Register, imm: i32 },
    Xori { rd: Register, rs1: Register, imm: i32 },
    Ori { rd: Register, rs1: Register, imm: i32 },
    Andi { rd: Register, rs1: Register, imm: i32 },
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

    Fence { pred: u8, succ: u8, fm: u8 },
    Ecall,
    Ebreak,

    Mul { rd: Register, rs1: Register, rs2: Register },
    Mulh { rd: Register, rs1: Register, rs2: Register },
    Mulhsu { rd: Register, rs1: Register, rs2: Register },
    Mulhu { rd: Register, rs1: Register, rs2: Register },
    Div { rd: Register, rs1: Register, rs2: Register },
    Divu { rd: Register, rs1: Register, rs2: Register },
    Rem { rd: Register, rs1: Register, rs2: Register },
    Remu { rd: Register, rs1: Register, rs2: Register },
}
