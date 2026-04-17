use crate::instruction::{Instruction, Op};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HierSelectors {
    pub lui: u8,
    pub auipc: u8,
    pub jal: u8,
    pub jalr: u8,
    pub branch: u8,
    pub load: u8,
    pub store: u8,
    pub op_imm: u8,
    pub op: u8,
    pub misc_mem: u8,
    pub system: u8,
    pub op_base: u8,
    pub op_m: u8,
    pub mul_family: u8,
    pub div_family: u8,
    pub mul: u8,
    pub mulh: u8,
    pub mulhsu: u8,
    pub mulhu: u8,
    pub div: u8,
    pub divu: u8,
    pub rem: u8,
    pub remu: u8,
}

impl HierSelectors {
    pub fn from_instruction(instruction: &Instruction) -> Self {
        let mut selectors = Self::default();

        match instruction {
            Instruction::Lui(_) => selectors.lui = 1,
            Instruction::Auipc(_) => selectors.auipc = 1,
            Instruction::Jal(_) => selectors.jal = 1,
            Instruction::Jalr(_) => selectors.jalr = 1,
            Instruction::Branch(_, _) => selectors.branch = 1,
            Instruction::Load(_, _) => selectors.load = 1,
            Instruction::Store(_, _) => selectors.store = 1,
            Instruction::OpImm(_, _) => selectors.op_imm = 1,
            Instruction::Fence(_) | Instruction::FenceI => selectors.misc_mem = 1,
            Instruction::Ecall | Instruction::Ebreak | Instruction::Csr(_, _) => selectors.system = 1,
            Instruction::Op(op, _) => {
                selectors.op = 1;

                if op.is_m_extension() {
                    selectors.op_m = 1;

                    match op {
                        Op::Mul => {
                            selectors.mul_family = 1;
                            selectors.mul = 1;
                        }
                        Op::Mulh => {
                            selectors.mul_family = 1;
                            selectors.mulh = 1;
                        }
                        Op::Mulhsu => {
                            selectors.mul_family = 1;
                            selectors.mulhsu = 1;
                        }
                        Op::Mulhu => {
                            selectors.mul_family = 1;
                            selectors.mulhu = 1;
                        }
                        Op::Div => {
                            selectors.div_family = 1;
                            selectors.div = 1;
                        }
                        Op::Divu => {
                            selectors.div_family = 1;
                            selectors.divu = 1;
                        }
                        Op::Rem => {
                            selectors.div_family = 1;
                            selectors.rem = 1;
                        }
                        Op::Remu => {
                            selectors.div_family = 1;
                            selectors.remu = 1;
                        }
                        _ => {}
                    }
                } else {
                    selectors.op_base = 1;
                }
            }
        }

        selectors
    }

    pub const fn top_sum(&self) -> u8 {
        self.lui
            + self.auipc
            + self.jal
            + self.jalr
            + self.branch
            + self.load
            + self.store
            + self.op_imm
            + self.op
            + self.misc_mem
            + self.system
    }

    pub const fn m_leaf_sum(&self) -> u8 {
        self.mul + self.mulh + self.mulhsu + self.mulhu + self.div + self.divu + self.rem + self.remu
    }

    pub const fn is_m_extension(&self) -> bool {
        self.op_m == 1
    }

    pub const fn as_mle_vector(&self) -> [u8; 23] {
        [
            self.lui,
            self.auipc,
            self.jal,
            self.jalr,
            self.branch,
            self.load,
            self.store,
            self.op_imm,
            self.op,
            self.misc_mem,
            self.system,
            self.op_base,
            self.op_m,
            self.mul_family,
            self.div_family,
            self.mul,
            self.mulh,
            self.mulhsu,
            self.mulhu,
            self.div,
            self.divu,
            self.rem,
            self.remu,
        ]
    }

    pub const fn is_well_formed(&self) -> bool {
        let op_split = self.op_base + self.op_m;
        let m_split = self.mul_family + self.div_family;
        let mul_leaf_sum = self.mul + self.mulh + self.mulhsu + self.mulhu;
        let div_leaf_sum = self.div + self.divu + self.rem + self.remu;

        self.top_sum() == 1
            && (self.op == 0 || op_split == 1)
            && (self.op_m == 0 || m_split == 1)
            && (self.mul_family == 0 || mul_leaf_sum == 1)
            && (self.div_family == 0 || div_leaf_sum == 1)
            && (self.op_m == 0 || self.m_leaf_sum() == 1)
    }
}
