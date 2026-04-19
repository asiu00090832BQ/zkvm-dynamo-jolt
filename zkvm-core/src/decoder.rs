use crate::vm::ZkvmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add { rd: usize, rs1: usize, rs2: usize },
    Sub { rd: usize, rs1: usize, rs2: usize },
    Sll { rd: usize, rs1: usize, rs2: usize },
    Slt { rd: usize, rs1: usize, rs2: usize },
    Sltu { rd: usize, rs1: usize, rs2: usize },
    Xor { rd: usize, rs1: usize, rs2: usize },
    Srl { rd: usize, rs1: usize, rs2: usize },
    Sra { rd: usize, rs1: usize, rs2: usize },
    Or { rd: usize, rs1: usize, rs2: usize },
    And { rd: usize, rs1: usize, rs2: usize },
    Mul { rd: usize, rs1: usize, rs2: usize },
    Mulh { rd: usize, rs1: usize, rs2: usize },
    Mulhsu { rd: usize, rs1: usize, rs2: usize },
    Mulhu { rd: usize, rs1: usize, rs2: usize },
    Div { rd: usize, rs1: usize, rs2: usize },
    Divu { rd: usize, rs1: usize, rs2: usize },
    Rem { rd: usize, rs1: usize, rs2: usize },
    Remu { rd: usize, rs1: usize, rs2: usize },
    Addi { rd: usize, rs1: usize, imm: i32 },
    Lui { rd: usize, imm: i32 },
    Auipc { rd: usize, imm: i32 },
    Jal { rd: usize, imm: i32 },
    Jalr { rd: usize, rs1: usize, imm: i32 },
    Ecall,
    Ebreak,
    Invalid(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HierSelectors {
    pub is_alu: bool,
    pub is_system: bool,
    pub is_m_ext: bool,
    pub sub_op: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decoded {
    pub word: u32,
    pub instruction: Instruction,
    pub selectors: HierSelectors,
}

pub fn decode(word: u32) -> Result<Decoded, ZkvmError> {
    let opcode = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as usize;
    let funct3 = ((word >> 12) & 0x7) as u8;
    let rs1 = ((word >> 15) & 0x1f) as usize;
    let rs2 = ((word >> 20) & 0x1f) as usize;
    let funct7 = ((word >> 25) & 0x7f) as u8;

    let imm_i = (word as i32) >> 20;
    let imm_u = (word & 0xfffff000) as i32;
    let imm_j = sign_extend_j(word);

    let mut is_m_ext = false;
    let mut is_alu = false;
    let mut is_system = false;

    let instruction = match opcode {
        0x33 => {
            is_alu = true;
            match (funct7, funct3) {
                (0x00, 0x0) => Instruction::Add { rd, rs1, rs2 },
                (0x20, 0x0) => Instruction::Sub { rd, rs1, rs2 },
                (0x00, 0x1) => Instruction::Sll { rd, rs1, rs2 },
                (0x00, 0x2) => Instruction::Slt { rd, rs1, rs2 },
                (0x00, 0x3) => Instruction::Sltu { rd, rs1, rs2 },
                (0x00, 0x4) => Instruction::Xor { rd, rs1, rs2 },
                (0x00, 0x5) => Instruction::Srl { rd, rs1, rs2 },
                (0x20, 0x5) => Instruction::Sra { rd, rs1, rs2 },
                (0x00, 0x6) => Instruction::Or { rd, rs1, rs2 },
                (0x00, 0x7) => Instruction::And { rd, rs1, rs2 },
                (0x01, 0x0) => { is_m_ext = true; Instruction::Mul { rd, rs1, rs2 } },
                (0x01, 0x1) => { is_m_ext = true; Instruction::Mulh { rd, rs1, rs2 } },
                (0x01, 0x2) => { is_m_ext = true; Instruction::Mulhsu { rd, rs1, rs2 } },
                (0x01, 0x3) => { is_m_ext = true; Instruction::Mulhu { rd, rs1, rs2 } },
                (0x01, 0x4) => { is_m_ext = true; Instruction::Div { rd, rs1, rs2 } },
                (0x01, 0x5) => { is_m_ext = true; Instruction::Divu { rd, rs1, rs2 } },
                (0x01, 0x6) => { is_m_ext = true; Instruction::Rem { rd, rs1, rs2 } },
                (0x01, 0x7) => { is_m_ext = true; Instruction::Remu { rd, rs1, rs2 } },
                _ => Instruction::Invalid(word),
            }
        }
        0x13 => {
            is_alu = true;
            match funct3 {
                0x0 => Instruction::Addi { rd, rs1, imm: imm_i },
                _ => Instruction::Invalid(word),
            }
        }
        0x37 => Instruction::Lui { rd, imm: imm_u },
        0x17 => Instruction::Auipc { rd, imm: imm_u },
        0x6f => Instruction::Jal { rd, imm: imm_j },
        0x67 => Instruction::Jalr { rd, rs1, imm: imm_i },
        0x73 => {
            is_system = true;
            match word {
                0x00000073 => Instruction::Ecall,
                0x00100073 => Instruction::Ebreak,
                _ => Instruruction::Invalid(word),
            }
        }
        _ => Instruction::Invalid(word),
    };

    if let Instruction::Invalid(_) = instruction {
        return Err(ZkzmError::DecodeError);
    }

    Ok(Decoded {
        word,
        instruction,
        selectors: HierSelectors {
            is_alu,
            is_system,
            is_m_ext,
            sub_op: funct3,
        },
    })
}

fn sign_extend_j(wordd: u32) -> i32 {
    let imm20 = (word >> 31) & 1;
    let imm10_1 = (word >> 21) & 0x3ff;
    let imm11 = (word >> 20) & 1;
    let imm19_12 = (word >> 12) & 0xff;
    let imm = ((imm20 as i32) << 20) | ((imm19_12 as i32) << 12) | ((imm11 as i32) << 11) | ((imm10_1 as i32) << 1);
    if imm20 != 0 { imm | !0x1fffff } else { imm }
}