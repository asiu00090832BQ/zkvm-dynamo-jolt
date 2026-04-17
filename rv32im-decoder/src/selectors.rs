use crate::error::DecodeError;
use crate::invariants::ensure_one_hot_slice;
use crate::types::Instruction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectorRow {
    pub opcode_class: [u8; 8],
    pub format_class: [u8; 6],
    pub extension_class: [u8; 2],
}

impl Default for SelectorRow {
    fn default() -> Self {
        Self {
            opcode_class: [0; 8],
            format_class: [0; 6],
            extension_class: [0; 2],
        }
    }
}

impl SelectorRow {
    pub fn from_instruction(instruction: &Instruction) -> Self {
        let mut row = Self::default();
        row.opcode_class[opcode_class(instruction)] = 1;
        row.format_class[format_class(instruction)] = 1;
        row.extension_class[extension_class(instruction)] = 1;
        row
    }

    pub fn validate(&self) -> Result<(), DecodeError> {
        ensure_one_hot_slice(&self.opcode_class, "selector opcode class must be one-hot")?;
        ensure_one_hot_slice(&self.format_class, "selector format class must be one-hot")?;
        ensure_one_hot_slice(
            &self.extension_class,
            "selector extension class must be one-hot",
        )?;
        Ok(())
    }
}

fn opcode_class(instruction: &Instruction) -> usize {
    match instruction {
        Instruction::Lui { .. } | Instruction::Auipc { .. } => 0,
        Instruction::Jal { .. } | Instruction::Jalr { .. } => 1,
        Instruction::Beq { .. }
        | Instruction::Bne { .. }
        | Instruction::Blt { .. }
        | Instruction::Bge { .. }
        | Instruction::Bltu { .. }
        | Instruction::Bgeu { .. } => 2,
        Instruction::Lb { .. }
        | Instruction::Lh { .. }
        | Instruction::Lw { .. }
        | Instruction::Lbu { .. }
        | Instruction::Lhu { .. } => 3,
        Instruction::Sb { .. } | Instruction::Sh { .. } | Instruction::Sw { .. } => 4,
        Instruction::Addi { .. }
        | Instruction::Slti { .. }
        | Instruction::Sltiu { .. }
        | Instruction::Xori { .. }
        | Instruction::Ori { .. }
        | Instruction::Andi { .. }
        | Instruction::Slli { .. }
        | Instruction::Srli { .. }
        | Instruction::Srai { .. } => 5,
        Instruction::Add { .. }
        | Instruction::Sub { .. }
        | Instruction::Sll { .. }
        | Instruction::Slt { .. }
        | Instruction::Sltu { .. }
        | Instruction::Xor { .. }
        | Instruction::Srl { .. }
        | Instruction::Sra { .. }
        | Instruction::Or { .. }
        | Instruction::And { .. }
        | Instruction::Mul { .. }
        | Instruction::Mulh { .. }
        | Instruction::Mulhsu { .. }
        | Instruction::Mulhu { .. }
        | Instruction::Div { .. }
        | Instruction::Divu { .. }
        | Instruction::Rem { .. }
        | Instruction::Remu { .. } => 6,
        Instruction::Fence { .. } | Instruction::Ecall | Instruction::Ebreak => 7,
    }
}

fn format_class(instruction: &Instruction) -> usize {
    match instruction {
        Instruction::Add { .. }
        | Instruction::Sub { .. }
        | Instruction::Sll { .. }
        | Instruction::Slt { .. }
        | Instruction::Sltu { .. }
        | Instruction::Xor { .. }
        | Instruction::Srl { .. }
        | Instruction::Sra { .. }
        | Instruction::Or { .. }
        | Instruction::And { .. }
        | Instruction::Mul { .. }
        | Instruction::Mulh { .. }
        | Instruction::Mulhsu { .. }
        | Instruction::Mulhu { .. }
        | Instruction::Div { .. }
        | Instruction::Divu { .. }
        | Instruction::Rem { .. }
        | Instruction::Remu { .. } => 0,

        Instruction::Jalr { .. }
        | Instruction::Lb { .. }
        | Instruction::Lh { .. }
        | Instruction::Lw { .. }
        | Instruction::Lbu { .. }
        | Instruction::Lhu { .. }
        | Instruction::Addi { .. }
        | Instruction::Slti { .. }
        | Instruction::Sltiu { .. }
        | Instruction::Xori { .. }
        | Instruction::Ori { .. }
        | Instruction::Andi { .. }
        | Instruction::Slli { .. }
        | Instruction::Srli { .. }
        | Instruction::Srai { .. }
        | Instruction::Fence { .. }
        | Instruction::Ecall
        | Instruction::Ebreak => 1,

        Instruction::Sb { .. } | Instruction::Sh { .. } | Instruction::Sw { .. } => 2,

        Instruction::Beq { .. }
        | Instruction::Bne { .. }
        | Instruction::Blt { .. }
        | Instruction::Bge { .. }
        | Instruction::Bltu { .. }
        | Instruction::Bgeu { .. } => 3,

        Instruction::Lui { .. } | Instruction::Auipc { .. } => 4,

        Instruction::Jal { .. } => 5,
    }
}

fn extension_class(instruction: &Instruction) -> usize {
    match instruction {
        Instruction::Mul { .. }
        | Instruction::Mulh { .. }
        | Instruction::Mulhsu { .. }
        | Instruction::Mulhu { .. }
        | Instruction::Div { .. }
        | Instruction::Divu { .. }
        | Instruction::Rem { .. }
        | Instruction::Remu { .. } => 1,
        _ => 0,
    }
}
