use crate::types::{Instruction, Decoded, HierSelectors, DecodeError};

pub fn decode(word: u32) -> Result<Decoded, DecodeError> {
    if (word & 0x3) != 0x3 {
        return Err(DecodeError::InvalidAlignment);
    }
    let opcode = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as usize;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = ((word >> 15) & 0x1f) as usize;
    let rs2 = ((word >> 20) & 0x1f) as usize;
    let funct7 = (word >> 25) & 0x7f;

    let (instruction, selectors) = match opcode {
        0x33 => {
            let (inst, is_m) = match (funct3, funct7) {
                (0x0, 0x00) => (Instruction::Add { rd, rs1, rs2 }, false),
                (0x0, 0x20) => (Instruction::Sub { rd, rs1, rs2 }, false),
                (0x0, 0x01) => (Instruction::Mul { rd, rs1, rs2 }, true),
                (0x4, 0x01) => (Instruction::Div { rd, rs1, rs2 }, true),
                (0x6, 0x01) => (Instruction::Rem { rd, rs1, rs2 }, true),
                _ => (Instruction::Invalid(word), false),
            };
            (inst, HierSelectors { is_alu: !is_m, is_m_ext: is_m, is_system: false, sub_op: funct3 })
        }
        0x13 => {
            let imm = (word as i32) >> 20;
            let inst = match funct3 {
                0x0 => Instruction::Addi { rd, rs1, imm },
                _ => Instruction::Invalid(word),
            };
            (inst, HierSelectors { is_alu: true, is_m_ext: false, is_system: false, sub_op: funct3 })
        }
        0x37 => {
            let imm = (word & 0xffff_f000) as i32;
            (Instruction::Lui { rd, imm }, HierSelectors::default())
        }
        0x6f => {
            let imm20 = (word >> 31) & 0x1;
            let imm10_1 = (word >> 21) & 0x3ff;
            let imm11 = (word >> 20) & 0x1;
            let imm19_12 = (word >> 12) & 0xff;
            let mut imm = (imm20 << 20) | (imm19_12 << 12) | (imm11 << 11) | (imm10_1 << 1);
            if imm20 != 0 { imm |= !0xfffff; }
            (Instruction::Jal { rd, imm: imm as i32 }, HierSelectors::default())
        }
        0x73 => {
            let inst = match word {
                0x0000_0073 => Instruction::Ecall,
                0x0010_0073 => Instruction::Ebreak,
                _ => Instruction::Invalid(word),
            };
            (inst, HierSelectors { is_alu: false, is_m_ext: false, is_system: true, sub_op: 0 })
        }
        _ => (Instruction::Invalid(word), HierSelectors::default()),
    };

    Ok(Decoded { word, instruction, selectors })
}
