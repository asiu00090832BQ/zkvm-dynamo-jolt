pub type Register = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchKind {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadKind {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreKind {
    Sb,
    Sh,
    Sw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpImmKind {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpKind {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MKind {
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui {
        rd: Register,
        imm: u32,
    },
    Auipc {
        rd: Register,
        imm: u32,
    },
    Jal {
        rd: Register,
        imm: i32,
    },
    Jalr {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Branch {
        kind: BranchKind,
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    Load {
        kind: LoadKind,
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Store {
        kind: StoreKind,
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    OpImm {
        kind: OpImmKind,
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Op {
        kind: OpKind,
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    M {
        kind: MKind,
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Fence {
        pred: u8,
        succ: u8,
        fm: u8,
    },
    Ecall,
    Ebreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedInstruction {
    pub word: u32,
    pub instruction: Instruction,
}

impl DecodedInstruction {
    pub const fn new(word: u32, instruction: Instruction) -> Self {
        Self { word, instruction }
    }
}

impl Instruction {
    pub fn mnemonic(&self) -> &'static str {
        match self {
            Instruction::Lui { .. } => "lui",
            Instruction::Auipc { .. } => "auipc",
            Instruction::Jal { .. } => "jal",
            Instruction::Jalr { .. } => "jalr",
            Instruction::Branch { kind, .. } => match kind {
                BranchKind::Beq => "beq",
                BranchKind::Bne => "bne",
                BranchKind::Blt => "blt",
                BranchKind::Bge => "bge",
                BranchKind::Bltu => "bltu",
                BranchKind::Bgeu => "bgeu",
            },
            Instruction::Load { kind, .. } => match kind {
                LoadKind::Lb => "lb",
                LoadKind::Lh => "lh",
                LoadKind::Lw => "lw",
                LoadKind::Lbu => "lbu",
                LoadKind::Lhu => "lhu",
            },
            Instruction::Store { kind, .. } => match kind {
                StoreKind::Sb => "sb",
                StoreKind::Sh => "sh",
                StoreKind::Sw => "sw",
            },
            Instruction::OpImm { kind, .. } => match kind {
                OpImmKind::Addi => "addi",
                OpImmKind::Slti => "slti",
                OpImmKind::Sltiu => "sltiu",
                OpImmKind::Xori => "xori",
                OpImmKind::Ori => "ori",
                OpImmKind::Andi => "andi",
                OpImmKind::Slli => "slli",
                OpImmKind::Srli => "srli",
                OpImmKind::Srai => "srai",
            },
            Instruction::Op { kind, .. } => match kind {
                OpKind::Add => "add",
                OpKind::Sub => "sub",
                OpKind::Sll => "sll",
                OpKind::Slt => "slt",
                OpKind::Sltu => "sltu",
                OpKind::Xor => "xor",
                OpKind::Srl => "srl",
                OpKind::Sra => "sra",
                OpKind::Or => "or",
                OpKind::And => "and",
            },
            Instruction::M { kind, .. } => match kind {
                MKind::Mul => "mul",
                MKind::Mulh => "mulh",
                MKind::Mulhsu => "mulhsu",
                MKind::Mulhu => "mulhu",
                MKind::Div => "div",
                MKind::Divu => "divu",
                MKind::Rem => "rem",
                MKind::Remu => "remu",
            },
            Instruction::Fence { .. } => "fence",
            Instruction::Ecall => "ecall",
            Instruction::Ebreak => "ebreak",
        }
    }
}
