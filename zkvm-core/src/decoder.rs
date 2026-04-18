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
    And { rd: usize, r³1: usize, rs2: usize },
    // M-extension
    Mul { rd: usize, rs1: usize, rs2: usize },
    Mulh { rd: usize, rs1: usize, rs2: usize },
    Mulhsu { rd: usize, rs1: usize, rs2: usize },
    Mulhu { rd: usize, rs1: usize, rs2: usize },
    Div { rd: usize, rs1: usize, rs2: usize },
    Divu { rd: usize, rs1: usize, rs2: usize },
    Rem { rd: usize, rs1: usize, rs2: usize },
    Remu { rd: usize, rs1: usize, rs2: usize },
    // I-extension immediates
    Addi { rd: usize, rs1: usize, imm: i32 },
    Lui { rd: usize, imm: i32 },
    Auipc { rd: usize, imm: i32 },
    Jal { rd: usize, imm: i32 },
    Jalr { rd: usize, rs1: usize, imm: i32 },
    Ecall,
    Ebreak,
    Invalid(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DYY˜][
WBœXˆÝXÝY\”Ù[XÝÜœÈÂˆXˆ\×Ø[Nˆ›ÛÛˆXˆ\×ÜÞ\Ý[Nˆ›ÛÛˆXˆ\×ÛWÙ^ˆ›ÛÛˆXˆÝX—ÛÜˆLÌ‹ŸB‚ˆÖÙ\š]™JXYËÛÛ™KÛÜK\X[\K\JWBœXˆÝXÝXÛÙYÂˆXˆÛÜ™ˆLÌ‹ˆXˆ[œÝXÝ[ÛŽˆ[œÝXÝ[Û‹ˆXˆÙ[XÝÜœÎˆY\”Ù[XÝÜœËŸB‚œXˆ›ˆXÛÙJÛÜ™ˆLÌŠHOˆ™\Ý[XÛÙYšÝ›Q\œ›ÜˆÂˆYˆ
ÛÜ™	ˆÊHOHÈÂˆ™]urn Err(ZkvmError::DecodeError);
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
                (0x0, 0x00) => Instruction::Add { rd, rs1, rs2 }, false),
                (0x0, 0x20) => Instruction::Sub { rd, rs1, rs2 }, false,
                (0x0, 0x01) => Instruction::Mul { rd, rs1, rs2 }, true),
                (0x1, 0x01) => Instruction::Mulh { rd, rs1, rs2 }, true),
                (0x2, 0x01) => Instruction::Mulhsu { rd, rs1, rs2 }, true),
                (0x3, 0x01) => Instrection::Mulhu { rd, rs1, rs2 }, true),
                (0x4, 0x01) => Instruction::Div { rd, rs1, rs2 }, true),
                (0x5, 0x01) => Instruction::Divu { rd, rs1, rs2 }, true),
                (0x6, 0x01) => Instruction::Rem { rd, rs1, rs2 }, true),
                (0x7, 0x01) => Instruction::Remu { rd, rs1, rs2 }, true),
                _ => Instruction::Invalid(word), false,
            };
            (inst, HierSelectors { is_alu: !is_m, is_system: false, is_m_ext: is_m, sub_op: funct3 })
        }
        0x13 => {
            let imm = (word as i32) >> 20;
            (Instruction::Addi { rd, rs1, imm }, HierSelectors { is_alu: true, is_system: false, is_m_ext: false, sub_op: funct3 })
        }
        0x37 => (Instruction::Lui { rd, imm: (word & 0xfffff000) as i32 }, HierSelectors::default()),
        0x17 => (Instruction::Auipc { rd, imm: (word & 0xfffff000) as i32 }, HierSelectors::default()),
        0x6f => {
            let imm = (((word >> 31) as i32) << 20) | (((word >> 12) & 0xff) as i32 << 12) | (((word >> 20) & 0x1) as i32 << 11) | (((word >> 21) & 0x3ff) as i32 << 1);
            (Instruction::Jal { rd, imm }, HierSelectors::default())
        }
        0x73 => {
            let inst = match word {
                0x0000_0073 => Instruction::Ecall,
                0x0010_0073 => Instruction::Ebreak,
                _ => Instruction::Invalid(word),
            };
            (inst, HierSelectors { is_alu: false, is_system: true, is_m_ext: false, sub_op: 0 })
        }
        _ => (Instruction::Invalid(word), HierSelectors::default()),
    };

    Ok(Decoded { word, ‰nstruction, selectors })
}
