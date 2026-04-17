use crate::instruction::Instruction;

/// Boolean selectors for downstream 0/1 gating.
///
/// Concrete instruction selectors are one-hot for valid decodes.
/// Family selectors may co-assert with a concrete selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DecodeSelectors {
    pub is_valid: bool,
    pub is_r_type: bool,
    pub is_i_type: bool,
    pub is_u_type: bool,
    pub is_j_type: bool,
    pub is_system: bool,
    pub is_m_extension: bool,
    pub is_ecall: bool,
    pub is_ebreak: bool,
    pub is_add: bool,
    pub is_sub: bool,
    pub is_addi: bool,
    pub is_lui: bool,
    pub is_jal: bool,
    pub is_mul: bool,
    pub is_mulh: bool,
    pub is_mulhsu: bool,
    pub is_mulhu: bool,
    pub is_div: bool,
    pub is_divu: bool,
    pub is_rem: bool,
    pub is_remu: bool,
}

impl DecodeSelectors {
    pub fn from_instruction(instruction: Instruction) -> Self {
        match instruction {
            Instruction::Add { .. } => Self {
                is_valid: true,
                is_r_type: true,
                is_add: true,
                ..Self::default()
            },
            Instruction::Sub { .. } => Self {
                is_valid: true,
                is_r_type: true,
                is_sub: true,
                ..Self::default()
            },
            Instruction::Addi { .. } => Self {
                is_valid: true,
                is_i_type: true,
                is_addi: true,
                ..Self::default()
            },
            Instruction::Lui { .. } => Self {
                is_valid: true,
                is_u_type: true,
                is_lui: true,
                ..Self::default()
            },
            Instruction::Jal { .. } => Self {
                is_valid: true,
                is_j_type: true,
                is_jal: true,
                ..Self::default()
            },
            Instruction::Mul { .. } => Self {
                is_valid: true,
                is_r_type: true,
                is_m_extension: true,
                is_mul: true,
                ..Self::default()
            },
            Instruction::Mulh { .. } => Self {
                is_valid: true,
                is_r_type: true,
                is_m_extension: true,
                is_mulh: true,
                ..Self::default()
            },
            Instruction::Mulhsu { .. } => Self {
                is_valid: true,
                is_r_type: true,
                is_m_extension: true,
                is_mulhsu: true,
                ..Self::default()
            },
            Instruction::Mulhu { .. } => Self {
                is_valid: true,
                is_r_type: true,
                is_m_extension: true,
                is_mulhu: true,
                ..Self::default()
            },
            Instruction::Div { .. } => Self {
                is_valid: true,
                is_r_type: true,
                is_m_extension: true,
                is_div: true,
                ..Self::default()
            },
            Instruction::Divu { .. } => Self {
                is_valid: true,
                is_r_type: true,
                is_m_extension: true,
                is_divu: true,
                ..Self::default()
            },
            Instruction::Rem { .. } => Self {
                is_valid: true,
                is_r_type: true,
                is_m_extension: true,
                is_rem: true,
                ..Self::default()
            },
            Instruction::Remu { .. } => Self {
                is_valid: true,
                is_r_type: true,
                is_m_extension: true,
                is_remu: true,
                ..Self::default()
            },
            Instruction::Ecall => Self {
                is_valid: true,
                is_system: true,
                is_ecall: true,
                ..Self::default()
            },
            Instruction::Ebreak => Self {
                is_valid: true,
                is_system: true,
                is_ebreak: true,
                ..Self::default()
            },
            Instruction::Invalid(_) => Self::default(),
        }
    }

    pub fn from_word(word: u32) -> Self {
        Self::from_instruction(crate::decoder::decode(word))
    }
}