use core::borrow::Borrow;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecoderConfig {
    pub enable_rv32m: bool,
}

impl Default for DecoderConfig {
    fn default() -> Self {
        Self { enable_rv32m: true }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Lui { rd: u8, imm: u32 },
    Auipc { rd: u8, imm: i32 },
    Jal { rd: u8, imm: i32 },
    Jalr { rd: u8, rs1: u8, imm: i32 },
    Branch { kind: BranchKind, rs1: u8, rs2: u8, imm: i32 },
    Load { kind: LoadKind, rd: u8, rs1: u8, imm: i32 },
    Store { kind: StoreKind, rs1: u8, rs2: u8, imm: i32 },
    OpImm { kind: OpImmKind, rd: u8, rs1: u8, imm: i32 },
    Op { kind: OpKind, rd: u8, rs1: u8, rs2: u8 },
    Fence,
    System(SystemInstruction),
}

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
pub enum SystemInstruction {
    Ecall,
    Ebreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    IllegalInstruction(u32),
    ExtensionDisabled { extension: &'static str, word: u32 },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DecodeError::IllegalInstruction(word) => {
                write!(f, "illegal instruction encoding 0x{word:08x}")
            }
            DecodeError::ExtensionDisabled { extension, word } => {
                write!(f, "disabled extension {extension} for word 0x{word:08x}")
            }
        }
    }
}

impl std::error::Error for DecodeError {}

const OPCODE_LUI: u8 = 0x37;
const OPCODE_AUIPC: u8 = 0x17;
const OPCODE_JAL: u8 = 0x6f;
const OPCODE_JALR: u8 = 0x67;
const OPCODE_BRANCH: u8 = 0x63;
const OPCODE_LOAD: u8 = 0x03;
const OPCODE_STORE: u8 = 0x23;
const OPCODE_OP_IMM: u8 = 0x13;
const OPCODE_OP: u8 = 0x33;
const OPCODE_MISC_MEM: u8 = 0x0f;
const OPCODE_SYSTEM: u8 = 0x73;

const FUNCT7_BASE: u8 = 0x00;
const FUNCT7_ALT: u8 = 0x20;
const FUNCT7_RV32M: u8 = 0x01;

pub fn decode<C>(word: u32, config: C) -> Result<Instruction, DecodeError>
where
    C: Borrow<DecoderConfig>,
{
    decode_inner(word, config.borrow())
}

fn decode_inner(word: u32, config: &DecoderConfig) -> Result<Instruction, DecodeError> {
    if (word & 0b11) != 0b11 {
        return Err(DecodeError::IllegalInstruction(word));
    }

    let op = opcode(word);

    let is_upper = select_upper(op);
    let is_control = select_control(op);
    let is_memory = select_memory(op);
    let is_alu = select_alu(op);
    let is_system = select_system_group(op);

    if is_upper {
        decode_upper(word)
    } else if is_control {
        decode_control(word)
    } else if is_memory {
        decode_memory(word)
    } else if is_alu {
        decode_alu(word, config)
    } else if is_system {
        decode_system(word)
    } else {
        Err(DecodeError::IllegalInstruction(word))
    }
}

fn decode_upper(word: u32) -> Result<Instruction, DecodeError> {
    match opcode(word) {
        OPCODE_LUI => Ok(Instruction::Lui {
            rd: rd(word),
            imm: decode_u_imm(word),
        }),
        OPCODE_AUIPC => Ok(Instruction::Auipc {
            rd: rd(word),
            imm: decode_u_imm(word) as i32,
        }),
        _ => Err(DecodeError::IllegalInstruction(word)),
    }
}

fn decode_control(word: u32) -> Result<Instruction, DecodeError> {
    let op = opcode(word);
    if select_jump(op) {
        decode_jump(word)
    } else {
        decode_branch(word)
    }
}

fn decode_jump(word: u32) -> Result<Instruction, DecodeError> {
    match opcode(word) {
        OPCODE_JAL => Ok(Instruction::Jal {
            rd: rd(word),
            imm: decode_j_imm(word),
        }),
        OPCODE_JALR => {
            if funct3(word) != 0 {
                return Err(DecodeError::IllegalInstruction(word));
            }
            Ok(Instruction::Jalr {
                rd: rd(word),
                rs1: rs1(word),
                imm: decode_i_imm(word),
            })
        }
        _ => Err(DecodeError::IllegalInstruction(word)),
    }
}

fn decode_branch(word: u32) -> Result<Instruction, DecodeError> {
    let kind = branch_kind(funct3(word)).ok_or_else(|| DecodeError::IllegalInstruction(word))?;
    Ok(Instruction::Branch {
        kind,
        rs1: rs1(word),
        rs2: rs2(word),
        imm: decode_b_imm(word),
    })
}

fn decode_memory(word: u32) -> Result<Instruction, DecodeError> {
    let op = opcode(word);
    if select_data_memory(op) {
        decode_data_memory(word)
    } else {
        decode_misc_memory(word)
    }
}

fn decode_data_memory(word: u32) -> Result<Instruction, DecodeError> {
    if select_load(opcode(word)) {
        let kind = load_kind(funct3(word)).ok_or_else(|| DecodeError::IllegalInstruction(word))?;
        Ok(Instruction::Load {
            kind,
            rd: rd(word),
            rs1: rs1(word),
            imm: decode_i_imm(word),
        })
    } else {
        let kind = store_kind(funct3(word)).ok_or_else(|| DecodeError::IllegalInstruction(word)))?;
        Ok(Instructio::Store {
            kind,
            rs1: rs1(word),
            rs2: rs2(word),
            imm: decode_s_imm(word),
        })
    }
}

fn decode_misc_memory(word: u32) -> Result<Instruction, DecodeError> {
    if funct3(word) != 0 {
        return Err(DecodeError::IllegalInstruction(word));
    }
    Ok(Instruction::Fence)
}

fn decode_alu(word: u32, config: &DecoderConfig) -> Result<Instruction, DecodeError> {
    if select_op_imm(opcode(word)) {
        decode_op_imm(word)
    } else {
        decode_op_reg(word, config)
    }
}

fn decode_op_imm(word: u32) -> Result<Instruction, DecodeError> {
    let f3 = funct3(word);
    let kind = match f3 {
        0b000 => OpImmKind::Addi,
        0b010 => OpImmKind::Slti,
        0b011 => OpImmKind::Sltiu,
        0b100 => OpImmKind::Xori,
        0b110 => OpImmKind::Ori,
        0b111 => OpImmKind::Andi,
        0b001 => {
            if funct7(word) != FUNCT7_BASE {
                return Err(DecodeError::IllegalInstruction(word));
            }
            OpImmKind::Slli
        }
        0b101 => match funct7(word) {
            FUNCT7_BASE => OpImmKind::Srli,
            FUNCT7_ALT => OpImmKind::Srai,
            _ => return Err(DecodeError::IllegalInstruction(word)),
        },
        _ => return Err(DecodeError::IllegalInstruction(word)),
    };

    let imm = if f3 == 0b001 || f3 == 0b101 {
        shamt(word) as i32
    } else {
        decode_i_imm(word)
    };

    Ok(Instruction::OpImm {
        kind,
        rd: rd(word),
        rs1: rs1(word),
        imm,
    })
}

fn decode_op_reg(word: u32, config: &DecoderConfig) -> Result<Instruction, DecodeError> {
    let f3 = funct3(word);
    let f7 = funct7(word);

    let is_rv32m = f7 == FUNCT7_RV32M;

    if is_rv32m {
        if !config.enable_rv32m {
            return Err(DecodeError::ExtensionDisabled {
                extension: "rv32m",
                word,
            });
        }
        let kind = op_kind_rv32m(f3).ok_or_else(|| DecodeError::IllegalInstruction(word))?;
        return Ok(Instruction::Op {
            kind,
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        });
    }

    let kind = op_kind_base(f7, f3).ok_or_else(|| DecodeError::IllegalInstruction(word))?;
    Ok(Instruction::Op {
        kind,
        rd: rd(word),
        rs1: rs1(word),
        rs2: rs2(word),
    })
}

fn decode_system(word: u32) -> Result<Instruction, DecodeError> {
    if funct3(word) != 0 {
        return Err(DecodeError::IllegalInstruction(word));
    }

    match wbit11 = ((word >> 20) & 0x1) << 11;
    let bits10_1 = ((word >> 21) & 0x03ff) << 1;
    sign_extend(bit20 | bits19_12 | bit11 | bits10_1, 21)
}
