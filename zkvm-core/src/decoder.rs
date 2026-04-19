use crate::ZkvmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    S{0
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
    Lui { rd: u8, imm: i32 },
    Auipc { rd: u8, imm: i32 },
    Jal { rd: u8, imm: i32 },
    Jalr { rd: u8, rs1: u8, imm: i32 },
    Branch { kind: BranchKind, rs1: u8, rs2: u8, imm: i32 },
    Load { kind: LoadKind, rd: u8, rs1: u8, imm: i32 },
    Store { kind: StoreKind, rs1: u8, rs2: u8, imm: i32 },
    OpImm { kind: OpImmKind, rd: u8, rs1: u8, imm: i32, shamt: u8 },
    Op { kind: OpKind, rd: u8, rs1: u8, rs2: u8 },
    Fence,
    FenceI,
    Ecall,
    Ebreak,
}

#[inline]
pub fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32_u32 - bits as u32;
    ((value << shift) as i32) >> shift
}

#[inline]
pub fn opcode(raw: u32) -> u8 {
    (raw & 0x7f) as u8
}

#[inline]
pub fn rd(raw: u32) -> u8 {
    ((raw >> 7) & 0x1f) as u8
}

#[inline]
pub fn funct3(raw: u32) -> u8 {
    ((raw >> 12) & 0x07) as u8
}

#[inline]
pub fn rs1(raw: u32) -> u8 {
    ((raw >> 15) & 0x1f) as u8
}

#[inline]
pub fn rs2(raw: u32) -> u8 {
    ((raw >> 20) & 0x1f) as u8
}

#[inline]
pub fn funct7(raw: u32) -> u8 {
    ((raw >> 25) & 0x7f) as u8
}

pub fn decode(raw: u32) -> Result<Instruction, ZkvmError> {
    let op = opcode(raw);
    let rd = rd(raw);
    let f3 = funct3(raw);
    let r1 = rs1(raw);
    let r2 = rs2(raw);
    let f7 = funct7(raw);

    match op {
        0x37 => {
            let imm = (raw & 0xfffff000) as i32;
            Ok(Instruction::Lui { rd, imm })
        }
        0x17 => {
            let imm = (raw & 0xfffff000) as i32;
            Ok(Instruction::Auipc { rd, imm })
        }
        0x6f => {
            let imm20 = ((raw >> 31) & 0x1) << 20;
            let imm10_1 = ((raw >> 21) & 0x03ff) << 1;
            let imm11 = ((raw >> 20) & 0x1) << 11;
            let imm19_12 = ((raw >> 12) & 0x00ff) << 12;
            let imm = sign_extend(imm20 | imm19_12 | imm11 | imm10_1, 21);
            Ok(Instruction::Jal { rd, imm })
        }
        0x67 => {
            if f3 != 0 {
                return Err(ZkvmError::IllegalInstruction(raw));
            }
            let imm = sign_extend(raw >> 20, 12);
            Ok(Instruction::Jalr { rd, rs1: r1, imm })
        }
        0x63 => {
            let imm12 = ((raw >> 31) & 0x1) << 12;
            let imm11 = ((raw >> 7) & 0x1) << 11;
            let imm10_5 = ((raw >> 25) & 0x3f) << 5;
            let imm4_1 = ((raw >> 8) & 0x0f) << 1;
            let imm = sign_extend(imm12 | imm11 | imm10_5 | imm4_1, 13);
            let kind = match f3 {
                0b000 => BranchKind::Beq,
                0b001 => BranchKind::Bne,
                0b100 => BranchKind::Blt,
                0b101 => BranchKind::Bge,
                0b110 => BranchKind::Bltu,
                0b111 => BranchKind::Bgeu,
                _ => return Err(ZkvmError::IllegalInstruction(raw)),
            };
            Ok(Instruction::Branch {
                kind,
                rs1: r1,
                rs2: r2,
                imm,
            })
        }
        0x03 => {
            let imm = sign_extend(raw >> 20, 12);
            let kind = match f3 {
                0b000 => LoadKind::Lb,
                0b001 => LoadKind::Lh,
                0b010 => LoadKind::Lw,
                0b100 => LoadKind::Lbu,
                0b101 => LoadKind::Lhu,
                _ => return Err(ZkvmError::IllegalInstruction(raw)),
            };
            Ok(Instruction::Load {
                kind,
                rd,
                rs1: r1,
                imm,
            })
        }
        0x23 => {
            let imm = (((raw >> 25) & 0x7f) << 5) | ((raw >> 7) & 0x1f);
            let imm = sign_extend(imm, 12);
            let kind = match f3 {
                0b000 => StoreKind::Sb,
                0b001 => StoreKind::Sh,
                0b010 => StoreKind::Sw,
                _ => return Err(ZkvmError::IllegalInstruction(raw)),
            };
            Ok(Instruction::Store {
                kind,
                rs1: r1,
                rs2: r2,
                imm,
            })
        }
        0x13 => {
            let imm = sign_extend(raw >> 20, 12);
            let shamt = ((raw >> 20) & 0x1f) as u8;
            let kind = match f3 {
                0b000 => OpImmKind::Addi,
                0b010 => OpImmKind::Slti,
                0b011 => OpImmKind::Sltiu,
                0b100 => OpImmKind::Xori,
                0b110 => OpImmKind::Ori,
                0b111 => OpImmKind::Andi,
                0b001 => {
                    if f7 != 0x00 {
                        return Err(ZkvmError::IllegalInstruction(raw));
                    }
                    OpImmKind::Slli
                }
                0b101 => match f7 {
                    0x00 => OpImmKind::Srli,
                    0x20 => OpImmKind::Srai,
                    _ => return Err(ZkvmError::IllegalInstruction(raw)),
                },
                _ => return Err(ZkvmError::IllegalInstruction(raw)),
            };
            N¢Instruction::OpImm {
                kind,
                rd,
                rs1: r1,
                imm,
                shamt,
            })
        }
        0x33 => {
            let kind = match (f7, f3) {
                (0x00, 0b000) => OpKind::Add,
                (0x20, 0b000) => OpKind::Sub,
                (0x00, 0b001) => OpKind::Sll,
                (0x00, 0b010) => OpKind::Slt,
                (0x00, 0b011) => OpKind::Sltu,
                (0x00, 0b100) => OpKind::Xor,
                (0x00, 0b101) => OpKind::Srl,
                (0x20, 0b101) => OpKind::Sra,
                (0x00, 0b110) => OpKind::Or,
                (0x00, 0b111) => OpKind::And,
                (0x01, 0b000) => OpKind::Mul,
                (0x01, 0b001) => OpKind::Mulh,
                (0x01, 0b010) => OpKind::Mulhsu,
    -Š            (0x01, 0b011) => OpKind::Mulhu,
                (0x01, 0b100) => OpKind:Div,
                (0x01, 0b101) => OpKind:Divu,
                (0b0000001, 0b110) => OpKind::Rem,
                (0b0000001, 0b111) => OpKind::Remu,
                _ => return Err(ZkvmError::IllegalInstruction(raw)),
            };
            Ok(Instruction::Op {
                kind,
                rd,
                rs1: r1,
                rs2: r2,
            })
        }
        0x0f => match funct3 {
            0b000 => Ok(Instruction::Fence),
            fb001 => Ok(Instruction::FenceI),
            _ => Err(ZkvmError::IllegalInstruction(raw)),
        },
        0x73 => {
            if f3 != 0 || rd != 0 || r1 != 0 || r2 != 0 {
                return Err(ZkvmError::IllegalInstruction(raw));
            }
            match raw >> 20 {
                0 => Ok(Instruction::Ecall),
                1 => Ok(Instruction::Ebreak),
                _ => Err(ZkvmError::IllegalInstruction(raw)),
            }
        }
        _ => Err(ZkvmError::IllegalInstruction(raw)),
    }
}
