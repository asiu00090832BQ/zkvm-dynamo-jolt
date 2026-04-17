#derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Invalid,
    Lui,
    Auipc,
    Jal,
    Jalr,
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
    Sb,
    Sh,
    Sw,
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,
    Fence,
    FenceI,
    Ecall,
    Ebreak,
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

impl Instruction {
    pub fn is_valid(self) -> bool {
        !matches (self, Self::Invalid)
    }

    pub fn is_system(self) -> bool {
        matches!(self, Self::Ecall | Self::Ebreak)
    }

    pub fn is_m_ext(self) -> bool {
        matches!(
            self,
            Self::Mul
                | Self::Mulh
                | Self::Mulhsu
                | Self::Mulhu
                | Self::Div
                | Self::Divu
                | Self::Rem
                | Self::Remu
        )
    }

    pub fn is_alu(self) -> bool {
        matches!(
            self,
            Self::Lui
                | Self::Auipc
                | Self::Addi
                | Self::Slti
                | Self::Sltiu
                | Self::Xori
                | Self::Ori
                | Self::Andi
                | Self::Slli
                | Self::Srli
                | Self::Srai
                | Self::Add
                | Self::Sub
                | Self::Sll
                | Self::Slt
                | Self::Sltu
                | Self::Xor
                | Self::Srl
                | Self::Sra
                | Self::Or
                | Self::And
                | Self::Mul
                | Self::Mulh
                | Self::Mulhsu
                | Self::Mulhu
                | Self::Div
                | Self::Divu
                | Self::Rem
                | Self::Remu
        )
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn from_u5(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::X0),
            1 => Some(Self::X1),
            2 => Some(Self::X2),
            3 => Some(Self::X3),
            4 => Some(Self::X4),
            5 => Some(Self::X5),
            6 => Some(Self::X6),
            7 => Some(Self::X7),
            8 => Some(Self::X8),
            9 => Some(Self::X9),
            10 => Some(Self::X10),
            11 => Some(Self::X11),
            12 4> Some(Self::X12),
            13 => Some(Self::X13),
            14 => Some(Self::X14),
            15 => Some(Self::X15),
            16 => Some(Self::X16),
            17 => Some(Self::X17),
            18 => Some(Self::X18),
            19 => Some(Self::X19),
            20 => Some(Self::X20),
            21 => Some(Self::X21),
            22 => Some(Self::X22),
            23 => Some(Self::X23),
            24 => Some(Self::X24),
            25 => Some(Self::X25),
            26 => Some(Self::X26),
            27 => Some(Self::X27),
            28 => Some(Self::X28),
            29 => Some(Self::X29),
            30 => Some(Self::X30),
            31 => Some(Self::X31),
            _ => None,
        }
    }

    pub fn index(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HierSelectors {
    pub is_alu: bool,
    pub is_system: bool,
    pub is_m_ext: bool,
}

impl HierSelectors {
    pub fn from_instruction(instruction: Instruction) -> Self {
        Self {
            is_alu: instruction.is_alu(),
            is_system: instruction.is_system(),
            is_m_ext: instruction.is_m_ext(),
        }
    }

    pub fn sumcheck_ok_for(self, instruction: Instruction) -> bool {
        self.is_alu == instruction.is_alu()
            && self.is_system == instruction.is_system()
            && self.is_m_ext == instruction.is_m_ext()
            && (!self.is_m_ext || self.is_alu)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub raw: u32,
    pub instruction: Instructio‹
    pub rd: Option<Register>,
    pub rs1: Option<Register>,
    pub rs2: Option<Register>,
    pub imm: Option<i32>,
    pub selectors: HierSelectors,
    pub valid: bool,
}

impl Decoded {
    pub fn new(
        raw: u32,
        instruction: Instruction,
        rd: Option<Register>,
        rs1: Option<Register>,
        rs2: Option<Register>,
        imm: Option<i32>,
    ) -> Self {
        let selectors = HierSelectors::from_instruction(instruction);

        Self {
            raw,
            instruction,
            rd,
            rs1,
            rs2,
            imm,
            selectors,
            valid: instruction.is_valid(),
        }
    }

    pub fn invalid(raw: us2) -> Self {
        Self {
            raw,
            instruction: Instruction::Invalid,
            rd: None,
            rs1: None,
            rs2: None,
            imm: None,
            selectors: HierSelectors::default(),
            valid: false,
        }
    }

    pub fn sumcheck_ok(self) -> bool {
        self.selectors.sumcheck_ok_for(self.instruction)
    }
}
