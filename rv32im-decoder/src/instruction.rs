#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RTypeFields {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
}

impl RTypeFields {
    pub const fn new(rd: u8, rs1: u8, rs2: u8) -> Self {
        Self { rd, rs1, rs2 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ITypeFields {
    pub rd: u8,
    pub rs1: u8,
    pub imm: i32,
}

impl ITypeFields {
    pub const fn new(rd: u8, rs1: u8, imm: i32) -> Self {
        Self { rd, rs1, imm }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct STypeFields {
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

impl STypeFields {
    pub const fn new(rs1: u8, rs2: u8, imm: i32) -> Self {
        Self { rs1, rs2, imm }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BTypeFields {
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

impl BTypeFields {
    pub const fn new(rs1: u8, rs2: u8, imm: i32) -> Self {
        Self { rs1, rs2, imm }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UTypeFields {
    pub rd: u8,
    pub imm: i32,
}

impl UTypeFields {
    pub const fn new(rd: u8, imm: i32) -> Self {
        Self { rd, imm }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JTypeFields {
    pub rd: u8,
    pub imm: i32,
}

impl JTypeFields {
    pub const fn new(rd: u8, imm: i32) -> Self {
        Self { rd, imm }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftImmFields {
    pub rd: u8,
    pub rs1: u8,
    pub shamt: u8,
}

impl ShiftImmFields {
    pub const fn new(rd: u8, rs1: u8, shamt: u8) -> Self {
        Self { rd, rs1, shamt }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui(UTypeFields),
    Auipc(UTypeFields),
    Jal(JTypeFields),
    Jalr(ITypeFields),

    Beq(BTypeFields),
    Bne(BTypeFields),
    Blt(BTypeFields),
    Bge(BTypeFields),
    Bltu(BTypeFields),
    Bgeu(BTypeFields),

    Lb(ITypeFields),
    Lh(ITypeFields),
    Lw(ITypeFields),
    Lbu(ITypeFields),
    Lhu(ITypeFields),

    Sb(STypeFields),
    Sh(STypeFields),
    Sw(STypeFields),

    Addi(ITypeFields),
    Slti(ITypeFields),
    Sltiu(ITypeFields),
    Xori(ITypeFields),
    Ori(ITypeFields),
    Andi(ITypeFields),
    Slli(ShiftImmFields),
    Srli(ShiftImmFields),
    Srai(ShiftImmFields),

    Add(RTypeFields),
    Sub(RTypeFields),
    Sll(RTypeFields),
    Slt(RTypeFields),
    Sltu(RTypeFields),
    Xor(RTypeFields),
    Srl(RTypeFields),
    Sra(RTypeFields),
    Or(RTypeFields),
    And(RTypeFields),

    Mul(RTypeFields),
    Mulh(RTypeFields),
    Mulhsu(RTypeFields),
    Mulhu(RTypeFields),
    Div(RTypeFields),
    Divu(RTypeFields),
    Rem(RTypeFields),
    Remu(RTypeFields),

    Fence,
    FenceI,
    Ecall,
    Ebreak,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
