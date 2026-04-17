#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RType {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IType {
    pub rd: u8,
    pub rs1: u8,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SType {
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BType {
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UType {
    pub rd: u8,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JType {
    pub rd: u8,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FenceFields {
    pub fm: u8,
    pub pred: u8,
    pub succ: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CsrType {
    pub rd: u8,
    pub rs1_or_zimm: u8,
    pub csr: u16,
    pub uses_immediate: bool,
}

impl CsrType {
    pub const fn rs1(self) -> Option<u8> {
        if self.uses_immediate {
            None
        } else {
            Some(self.rs1_or_zimm)
        }
    }

    pub const fn zimm(self) -> Option<u8> {
        if self.uses_immediate {
            Some(self.rs1_or_zimm)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BranchOp {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoadOp {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreOp {
    Sb,
    Sh,
    Sw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpImm {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
}

impl OpImm {
    pub fn execute(self, lhs: u32, imm: i32) -> u32 {
        match self {
            Self::Addi => lhs.wrapping_add(imm as u32),
            Self::Slti => ((lhs as i32) < imm) as u32,
            Self::Sltiu => (lhs < imm as u32) as u32,
            Self::Xori => lhs ^ imm as u32,
            Self::Ori => lhs | imm as u32,
            Self::Andi => lhs & imm as u32,
            Self::Slli => lhs.wrapping_shl((imm as u32) & 0x1f),
            Self::Srli => lhs.wrapping_shr((imm as u32) & 0x1f),
            Self::Srai => ((lhs as i32) >> ((imm as u32) & 0x1f)) as u32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
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
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

impl Op {
    pub const fn is_m_extension(self) -> bool {
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

    pub const fn uses_lemma_6_1_1(self) -> bool {
        matches!(self, Self::Mul | Self::Mulh | Self::Mulhsu | Self::Mulhu)
    }

    pub fn execute(self, lhs: u32, rhs: u32) -> u32 {
        match self {
            Self::Add => lhs.wrapping_add(rhs),
            Self::Sub => lhs.wrapping_sub(rhs),
            Self::Sll => lhs.wrapping_shl(rhs & 0x1f),
            Self::Slt => ((lhs as i32) < (rhs as i32)) as u32,
            Self::Sltu => (lhs < rhs) as u32,
            Self::Xor => lhs ^ rhs,
            Self::Srl => lhs.wrapping_shr(rhs & 0x1f),
            Self::Sra => ((lhs as i32) >> (rhs & 0x1f)) as u32,
            Self::Or => lhs | rhs,
            Self::And => lhs & rhs,
            Self::Mul => MulReduction16::from_operands(lhs, rhs).low_word(),
            Self::Mulh => MulReduction16::from_operands(lhs, rhs).high_word_signed(),
            Self::Mulhsu => MulReduction16::from_operands(lhs, rhs).high_word_signed_unsigned(),
            Self::Mulhu => MulReduction16::from_operands(lhs, rhs).high_word_unsigned(),
            Self::Div => signed_div(lhs, rhs),
            Self::Divu => {
                if rhs == 0 {
                    u32::MAX
                } else {
                    lhs / rhs
                }
            }
            Self::Rem => signed_rem(lhs, rhs),
            Self::Remu => {
                if rhs == 0 {
                    lhs
                } else {
                    lhs % rhs
                }
            }
        }
    }

    pub fn mul_reduction16(self, lhs: u32, rhs: u32) -> Option<MulReduction16> {
        if self.uses_lemma_6_1_1() {
            Some(MulReduction16::from_operands(lhs, rhs))
        } else {
            None
        }
    }
}

fn signed_div(lhs: u32, rhs: u32) -> u32 {
    let dividend = lhs as i32;
    let divisor = rhs as i32;

    if divisor == 0 {
        u32::MAX
    } else if dividend == i32::MIN && divisor == -1 {
        dividend as u32
    } else {
        (dividend / divisor) as u32
    }
}

fn signed_rem(lhs: u32, rhs: u32) -> u32 {
    let dividend = lhs as i32;
    let divisor = rhs as i32;

    if divisor == 0 {
        lhs
    } else if dividend == i32::MIN && divisor == -1 {
        0
    } else {
        (dividend % divisor) as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CsrOp {
    Csrrw,
    Csrrs,
    Csrrc,
    Csrrwi,
    Csrrsi,
    Csrrci,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    Lui(UType),
    Auipc(UType),
    Jal(JType),
    Jalr(IType),
    Branch(BranchOp, BType),
    Load(LoadOp, IType),
    Store(StoreOp, SType),
    OpImm(OpImm, IType),
    Op(Op, RType),
    Fence(FenceFields),
    FenceI,
    Ecall,
    Ebreak,
    Csr(CsrOp, CsrType),
}

impl Instruction {
    pub const fn rd(&self) -> Option<u8> {
        match *self {
            Self::Lui(UType { rd, .. })
            | Self::Auipc(UType { rd, .. })
            | Self::Jal(JType { rd, .. })
            | Self::Jalr(IType { rd, .. })
            | Self::Load(_, IType { rd, .. })
            | Self::OpImm(_, IType { rd, .. })
            | Self::Op(_, RType { rd, .. })
            | Self::Csr(_, CsrType { rd, .. }) => Some(rd),
            Self::Branch(..)
            | Self::Store(..)
            | Self::Fence(_)
            | Self::FenceI
            | Self::Ecall
            | Self::Ebreak => None,
        }
    }

    pub const fn rs1(&self) -> Option<u8> {
        match *self {
            Self::Jalr(IType { rs1, .. })
            | Self::Load(_, IType { rs1, .. })
            | Self::OpImm(_, IType { rs1, .. })
            | Self::Store(_, SType { rs1, .. })
            | Self::Branch(_, BType { rs1, .. })
            | Self::Op(_, RType { rs1, .. })
            | Self::Csr(_, CsrType { rs1_or_zimm: rs1, uses_immediate: false, .. }) => Some(rs1),
            Self::Lui(_)
            | Self::Auipc(_)
            | Self::Jal(_)
            | Self::Fence(_)
            | Self::FenceI
            | Self::Ecall
            | Self::Ebreak
            | Self::Csr(_, CsrType { uses_immediate: true, .. }) => None,
        }
    }

    pub const fn rs2(&self) -> Option<u8> {
        match *self {
            Self::Store(_, SType { rs2, .. })
            | Self::Branch(_, BType { rs2, .. })
            | Self::Op(_, RType { rs2, .. }) => Some(rs2),
            Self::Lui(_)
            | Self::Auipc(_)
            | Self::Jal(_)
            | Self::Jalr(_)
            | Self::Load(_, _)
            | Self::OpImm(_, _)
            | Self::Fence(_)
            | Self::FenceI
            | Self::Ecall
            | Self::Ebreak
            | Self::Csr(_, _) => None,
        }
    }

    pub const fn is_m_extension(&self) -> bool {
        match *self {
            Self::Op(op, _) => op.is_m_extension(),
            _ => false,
        }
    }

    pub fn lemma_6_1_1_reduction(&self, lhs: u32, rhs: u32) -> Option<MulReduction16> {
        match *self {
            Self::Op(op, _) => op.mul_reduction16(lhs, rhs),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MulReduction16 {
    pub a0: u16,
    pub a1: u16,
    pub b0: u16,
    pub b1: u16,
}

impl MulReduction16 {
    pub const LIMB_BITS: u32 = 16;
    pub const LIMB_BASE: u64 = 1u64 << Self::LIMB_BITS;

    pub const fn from_operands(a: u32, b: u32) -> Self {
        Self {
            a0: (a & 0xffff) as u16,
            a1: (a >> 16) as u16,
            b0: (b & 0xffff) as u16,
            b1: (b >> 16) as u16,
        }
    }

    pub const fn a(self) -> u32 {
        ((self.a1 as u32) << Self::LIMB_BITS) | self.a0 as u32
    }

    pub const fn b(self) -> u32 {
        ((self.b1 as u32) << Self::LIMB_BITS) | self.b0 as u32
    }

    pub const fn low_term(self) -> u64 {
        (self.a0 as u64) * (self.b0 as u64)
    }

    pub const fn cross_term(self) -> u64 {
        (self.a0 as u64) * (self.b1 as u64) + (self.a1 as u64) * (self.b0 as u64)
    }

    pub const fn high_term(self) -> u64 {
        (self.a1 as u64) * (self.b1 as u64)
    }

    pub const fn full_product(self) -> u64 {
        self.low_term() + (self.cross_term() << Self::LIMB_BITS) + (self.high_term() << 32)
    }

    pub const fn low_word(self) -> u32 {
        self.full_product() as u32
    }

    pub const fn high_word_unsigned(self) -> u32 {
        (self.full_product() >> 32) as u32
    }

    pub fn high_word_signed(self) -> u32 {
        let mut high = self.high_word_unsigned();

        if (self.a1 & 0x8000) != 0 {
            high = high.wrapping_sub(self.b());
        }

        if (self.b1 & 0x8000) != 0 {
            high = high.wrapping_sub(self.a());
        }

        high
    }

    pub fn high_word_signed_unsigned(self) -> u32 {
        let mut high = self.high_word_unsigned();

        if (self.a1 & 0x8000) != 0 {
            high = high.wrapping_sub(self.b());
        }

        high
    }

    pub fn signed_high_word(lhs: u32, rhs: u32) -> u32 {
        Self::from_operands(lhs, rhs).high_word_signed()
    }

    pub fn signed_unsigned_high_word(lhs: u32, rhs: u32) -> u32 {
        Self::from_operands(lhs, rhs).high_word_signed_unsigned()
    }
}
