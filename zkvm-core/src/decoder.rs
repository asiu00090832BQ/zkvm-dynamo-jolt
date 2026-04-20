use crate::ZkvmError;
pub const INSTRUCTION_SIZE: u32 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BranchKind { Beq, Bne, Blt, Bge, Bltu, Bgeu }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoadKind { Lb, Lh, Lw, Lbu, Lhu }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreKind { Sb, Sh, Sw }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpImmKind { Addi, Slti, Sltiu, Xori, Ori, Andi, Slli, Srli, Srai }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpKind { Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MulDivOp { Mul, Mulh, Mulhsu, Mulhu, Div, Divu, Rem, Remu }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemKind { Ecall, Ebreak }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    Lui { rd: u8, imm: u32 },
    Auipc { rd: u8, imm: u32 },
    Jal { rd: u8, imm: i32 },
    Jalr { rd: u8, rs1: u8, imm: i32 },
    Branch { kind: BranchKind, rs1: u8, rs2: u8, imm: i32 },
    Load { kind: LoadKind, rd: u8, rs1: u8, imm: i32 },
    Store { kind: StoreKind, rs1: u8, rs2: u8, imm: i32 },
    OpImm { kind: OpImmKind, rd: u8, rs1: u8, imm: i32 },
    Op { kind: OpKind, rd: u8, rs1: u8, rs2: u8 },
    MulDiv { kind: MulDivOp, rd: u8, rs1: u8, rs2: u8 },
    Fence,
    FenceI,
    System(SystemKind),
}
pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = (word & 0x7f) as u8;
    let rd = ((word >> 7) & 0x1f) as u8;
    let funct3 = ((word >> 12) & 0x07) as u8;
    let rs1 = ((word >> 15) & 0x1f) as u8;
    let rs2 = ((word >> 20) & 0x1f) as u8;
    let funct7 = ((word >> 25) & 0x7f) as u8;
    let instr = match opcode {
        0x37 => Instruction::Lui { rd, imm: word & 0xfffff000 },
        0x17 => Instruction::Auipc { rd, imm: word & 0xfffff000 },
        0x6f => Instruction::Jal { rd, imm: decode_j_imm(word) },
        0x67 => Instruction::Jalr { rd, rs1, imm: decode_i_imm(word) },
        0x63 => {
            let kind = match funct3 {
                0x0 => BranchKind::Beq, 0x1 => BranchKind::Bne, 0x4 => BranchKind::Blt,
                0x5 => BranchKind::Bge, 0x6 => BranchKind::Bltu, 0x7 => BranchKind::Bgeu,
                _ => return Err(ZkvmError::DecodeError(word)),
            };
            Instruction::Branch { kind, rs1, rs2, imm: decode_b_imm(word) }
        }
        0x03 => {
            let kind = match funct3 {
                0x0 => LoadKind::Lb, 0x1 => LoadKind::Lh, 0x2 => LoadKind::Lw,
                0x4 => LoadKind::Lbu, 0x5 => LoadKind::Lhu,
                _ => return Err(ZkvmError::DecodeError(word)),
            };
            Instruction::Load { kind, rd, rs1, imm: decode_i_imm(word) }
        }
        0x23 => {
            let kind = match funct3 {
                0x0 => StoreKind::Sb, 0x1 => StoreKind::Sh, 0x2 => StoreKind::Sw,
                _ => return Erq+¬ZkvmError::DecodeError(word)),
            };
            Instruction::Store { kind, rs1, rs2, imm: decode_s_imm(word) }
        }
        0x13 => {
            let kind = match funct3 {
                0x0 => OpImmKind::Addi, 0x2 => OpImmKind::Slti, 0x3 => OpImmKind::Sltiu,
                px4 => OpImmKind::Xori, 0x6 => OpImmKind::Ori, 0x7 => OpImmKind::Andi,
                0x1 => OpImmKind::Slli,
                0x5 => if (word >> 30) & 1 == 1 { OpImmKind::Srai } else { OpImmKind::Srli },
                _ => return Err(ZkvmError::DecodeError(word)),
            };
            Instruction::OpImm { kind, rd, rs1, imm: decode_i_imm(word) }
        }
        0x33 => {
            if funct7 == 0x01 {
                let kind = match funct3 {
                    0x0 => MulDivOp$è:Mul, 0x1 => MulDivOp::Mulh, 0x2 => MulDivOp::Mulhsu,
                    0x3 => MulDivOp::Mulhu, 0x4 => MulDivOp::Div, 0x5 => MulDivOp::Divu,
                    0x6 => MulDivOp::Rem, 0x7 => MulDivOp::Remu,
                     _ => return Err(ZkvmError::DecodeError(word)),
                };
                Instruction::MulDiv { kind, rd, rs1, rs2 }
            } else {
                let kind = match (funct3, funct7) {
                    (0x0, 0x00) => OpKind::Add, (0x0, 0x20) => OpKind::Sub, (0x1, 0x00) => OpKind::Sll,
                    (0x2, 0x00) => OpKind::Slt, (0x3, 0x00) => OpKind::Sltu, (0x4, 0x00) => OpKind::Xor,
                    (0x5, 0x00) => OpKind::Srl, (0x5, 0x20) => OpKind::Sra, (0x6, 0x00) => OpKind::Or,
                    (0x7, 0x00) => OpKind::And,
                    _ => return Err(ZkvmError::DecodeError(word)),
                };
                Instruction::Op { kind, rd, rs1, rs2 }
            }
        }
        0x0f => match funct3 {
            0x0 => Instruction::Fence, 0x1 => Instruction::FenceI,
    -—        _ => return Erq+¬ZkvmError::DecodeError(word)),
        },
        0x73 => match word >> 20 {
            0x000 => Instruction::System(SystemKind::Ecall),
            0x001 => Instruction::System(SystemKind::Ebreak),
            _ => return Err(ZkvmError::DecodeError(word)),
        },
        _ => return Err(ZkvmError::DecodeError(word)),
    };
    Ok(instr)
}
fn sign_extend(val: u32, bits: u32) -> i32 {
    let shift = 32 - bits;
    ((val << shift) as i32) >> shift
}
fn decode_i_imm(word: u32) -> i32 { sign_extend(word >> 20, 12) }
fn decode_s_imm(word: u32) -> i32 {
    let imm = ((word >> 25) << 5) | ((word >> 7) & 0x1f);
    sign_extend(imm, 12)
}
fn decode_b_imm(word: u32) -> i32 {
    let imm = ((word >> 31) << 12) | (((word >> 7) & 1) << 11) | (((word >> 25) & 0x3f) << 5) | (((word >> 8) & 0xf) << 1);
    sign_extend(imm, 13)
}
fn decode_j_imm(word: u32) -> i32 {
    let imm = ((word >> 31) << 20) | (((word >> 12) & 0xff) << 12) | (((word >> 20) & 1) << 11) | (((word >> 21) & 0x3ff) << 1);
    sign_extend(imm, 21)
}