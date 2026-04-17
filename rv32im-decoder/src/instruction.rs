pub enum Instruction {
    Lui, Auipc, Jal, Jalr,
    Beq, Bne, Blt, Bge, Bltu, Bgeu,
    Lb, Lh, Lw, Lbu, Lhu,
    Sb, Sh, Sw,
    Addi, Slti, Sltiu, Xori, Ori, Andi, Slli, Srli, Srai,
    Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And,
    Fence, FenceI, Ecall, Ebreak,
    Csrrw, Csrrs, Csrrc, Csrrwi, Csrrsi, Csrrci,
    Mul, Mulh, Mulhsu, Mulhu, Div, Divu, Rem, Remu,
}

impl Instruction {
    pub const fn mnemonic(self) -> &'static str {
        match self {
            Self::Lui => "lui", Self::Auipc => "auipc", Self::Jal => "jal", Self::Jalr => "jalr",
            Self::Beq => "beq", Self::Bne => "bne", Self::Blt => "blt", Self::Bge => "bge", Self::Bltu => "bltu", Self::Bgeu => "bgeu",
            Self::Lb => "lb", Self::Lh => "lh", Self::Lw => "lw", Self::Lbu => "lbu", Self::Lhu => "lhu",
            Self::Sb => "sb", Self::Sh => "sh", Self::Sw => "sw",
            Self::Addi => "addi", Self::Slti => "slti", Self::Sltiu => "sltiu", Self::Xori => "xori", Self::Ori => "ori", Self::Andi => "andi", Self::Slli => "slli", Self::Srli => "srli", Self::Srai => "srai",
            Self::Add => "add", Self::Sub => "sub", Self::Sll => "sll", Self::Slt => "slt", Self::Sltu => "sltu", Self::Xor => "xor", Self::Srl => "srl", Self::Sra => "sra", Self::Or => "or", Self::And => "and",
            Self::Fence => "fence", Self::FenceI => "fence.i", Self::Ecall => "ecall", Self::Ebreak => "ebreak",
            Self::Csrrw => "csrrw", Self::Csrrs => "csrrs", Self::Csrrc => "csrrc", Self::Csrrwi => "csrrwi", Self::Csrrsi => "csrrsi", Self::Csrrci => "csrrci",
            Self::Mul => "mul", Self::Mulh => "mulh", Self::Mulhsu => "mulhsu", Self::Mulhu => "mulhu", Self::Div => "div", Self::Divu => "divu", Self::Rem => "rem", Self::Remu => "remu",
        }
    }
}
