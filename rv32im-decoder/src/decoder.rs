use crate::instruction::{
    BType, BranchOp, CsrOp, CsrType, FenceFields, IType, Instruction, JType, LoadOp, Op, OpImm,
    RType, SType, StoreOp, UType,
};
use crate::{DecodeResult, HierSelectors, Zkvm, ZkvmError};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Decoder;

impl Decoder {
    pub const fn new() -> Self {
        Self
    }

    pub fn decode_word(word: u32) -> DecodeResult<Instruction> {
        decode_impl(word)
    }

    pub fn selectors(word: u32) -> DecodeResult<HierSelectors> {
        Self::decode_word(word).map(|instruction| HierSelectors::from_instruction(&instruction))
    }

    pub fn decode_with_selectors(word: u32) -> DecodeResult<(Instruction, HierSelectors)> {
        let instruction = Self::decode_word(word)?;
        let selectors = HierSelectors::from_instruction(&instruction);
        Ok((instruction, selectors))
    }
}

impl Zkvm for Decoder {
    fn decode(&self, word: u32) -> DecodeResult<Instruction> {
        decode_impl(word)
    }
}

fn decode_impl(word: u32) -> DecodeResult<Instruction> {
    let op = opcode(word);

    match op {
        0x37 => Ok(Instruction::Lui(UType {
            rd: rd(word),
            imm: imm_u(word),
        })),
        0x17 => Ok(Instruction::Auipc(UType {
            rd: rd(word),
            imm: imm_u(word),
        })),
        0x6f => Ok(Instruction::Jal(JType {
            rd: rd(word),
            imm: imm_j(word),
        })),
        0x67 => decode_jalr(word),
        0x63 => decode_branch(word),
        0x03 => decode_load(word),
        0x23 => decode_store(word),
        0x13 => decode_op_imm(word),
        0x33 => decode_op(word),
        0x0f => decode_misc_mem(word),
        0x73 => decode_system(word),
        _ => Err(ZkvmError::InvalidOpcode { word, opcode: op }),
    }
}

fn decode_jalr(word: u32) -> DecodeResult<Instruction> {
    let op = opcode(word);
    let f3 = funct3(word);

    if f3 != 0 {
        return Err(ZkvmError::InvalidFunct3 {
            word,
            opcode: op,
            funct3: f3,
        });
    }

    Ok(Instruction::Jalr(IType {
        rd: rd(word),
        rs1: rs1(word),
        imm: imm_i(word),
    }))
}

fn decode_branch(word: u32) -> DecodeResult<Instruction> {
    let op = opcode(word);
    let branch = match funct3(word) {
        0b000 => BranchOp::Beq,
        0b001 => BranchOp::Bne,
        0b100 => BranchOp::Blt,
        0b101 => BranchOp::Bge,
        0b110 => BranchOp::Bltu,
        0b111 => BranchOp::Bgeu,
        funct3 => {
            return Err(ZkvmError::InvalidFunct3 {
                word,
                opcode: op,
                funct3,
            })
        }
    };

    Ok(Instruction::Branch(
        branch,
        BType {
            rs1: rs1(word),
            rs2: rs2(word),
            imm: imm_b(word),
        },
    ))
}

fn decode_load(word: u32) -> DecodeResult<Instruction> {
    let op = opcode(word);
    let load = match funct3(word) {
        0b000 => LoadOp::Lb,
        0b001 => LoadOp::Lh,
        0b010 => LoadOp::Lw,
        0b100 => LoadOp::Lbu,
        0b101 => LoadOp::Lhu,
        funct3 => {
            return Err(ZkvmError::InvalidFunct3 {
                word,
                opcode: op,
                funct3,
            })
        }
    };

    Ok(Instruction::Load(
        load,
        IType {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        },
    ))
}

fn decode_store(word: u32) -> DecodeResult<Instruction> {
    let op = opcode(word);
    let store = match funct3(word) {
        0b000 => StoreOp::Sb,
        0b001 => StoreOp::Sh,
        0b010 => StoreOp::Sw,
        funct3 => {
            return Err(ZkvmError::InvalidFunct3 {
                word,
                opcode: op,
                funct3,
            })
        }
    };

    Ok(Instruction::Store(
        store,
        SType {
            rs1: rs1(word),
            rs2: rs2(word),
            imm: imm_s(word),
        },
    ))
}

fn decode_op_imm(word: u32) -> DecodeResult<Instruction> {
    let op = opcode(word);
    let f3 = funct3(word);
    let f7 = funct7(word);

    let instruction = match f3 {
        0b000 => Instruction::OpImm(
            OpImm::Addi,
            IType {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
        ),
        0b010 => Instruction::OpImm(
            OpImm::Slti,
            IType {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
        ),
        0b011 => Instruction::OpImm(
            OpImm::Sltiu,
            IType {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
        ),
        0b100 => Instruction::OpImm(
            OpImm::Xori,
            IType {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
        ),
        0b110 => Instruction::OpImm(
            OpImm::Ori,
            IType {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
        ),
        0b111 => Instruction::OpImm(
            OpImm::Andi,
            IType {
                rd: rd(word),
                rs1: rs1(word),
                imm: imm_i(word),
            },
        ),
        0b001 => {
            if f7 != 0x00 {
                return Err(ZkvmError::InvalidShiftEncoding { word, funct7: f7 });
            }

            Instruction::OpImm(
                OpImm::Slli,
                IType {
                    rd: rd(word),
                    rs1: rs1(word),
                    imm: shamt(word) as i32,
                },
            )
        }
        0b101 => {
            let op_imm = match f7 {
                0x00 => OpImm::Srli,
                0x20 => OpImm::Srai,
                _ => return Err(ZkvmError::InvalidShiftEncoding { word, funct7: f7 }),
            };

            Instruction::OpImm(
                op_imm,
                IType {
                    rd: rd(word),
                    rs1: rs1(word),
                    imm: shamt(word) as i32,
                },
            )
        }
        funct3 => {
            return Err(ZkvmError::InvalidFunct3 {
                word,
                opcode: op,
                funct3,
            })
        }
    };

    Ok(instruction)
}

fn decode_op(word: u32) -> DecodeResult<Instruction> {
    let op = opcode(word);
    let f3 = funct3(word);
    let f7 = funct7(word);

    let alu_op = match (f7, f3) {
        (0x00, 0b000) => Op::Add,
        (0x00, 0b001) => Op::Sll,
        (0x00, 0b010) => Op::Slt,
        (0x00, 0b011) => Op::Sltu,
        (0x00, 0b100) => Op::Xor,
        (0x00, 0b101) => Op::Srl,
        (0x00, 0b110) => Op::Or,
        (0x00, 0b111) => Op::And,
        (0x20, 0b000) => Op::Sub,
        (0x20, 0b101) => Op::Sra,
        (0x01, 0b000) => Op::Mul,
        (0x01, 0b001) => Op::Mulh,
        (0x01, 0b010) => Op::Mulhsu,
        (0x01, 0b011) => Op::Mulhu,
        (0x01, 0b100) => Op::Div,
        (0x01, 0b101) => Op::Divu,
        (0x01, 0b110) => Op::Rem,
        (0x01, 0b111) => Op::Remu,
        (_, funct3) if f7 == 0x00 || f7 == 0x20 || f7 == 0x01 => {
            return Err(ZkvmError::InvalidFunct3 {
                word,
                opcode: op,
                funct3,
            })
        }
        _ => {
            return Err(ZkvmError::InvalidFunct7 {
                word,
                opcode: op,
                funct3: f3,
                funct7: f7,
            })
        }
    };

    Ok(Instruction::Op(
        alu_op,
        RType {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        },
    ))
}

fn decode_misc_mem(word: u32) -> DecodeResult<Instruction> {
    let op = opcode(word);

    match funct3(word) {
        0b000 => Ok(Instruction::Fence(FenceFields {
            fm: ((word >> 28) & 0x0f) as u8,
            pred: ((word >> 24) & 0x0f) as u8,
            succ: ((word >> 20) & 0x0f) as u8,
        })),
        0b001 => Ok(Instruction::FenceI),
        funct3 => Err(ZkvmError::InvalidFunct3 {
            word,
            opcode: op,
            funct3,
        }),
    }
}

fn decode_system(word: u32) -> DecodeResult<Instruction> {
    match funct3(word) {
        0b000 => match funct12(word) {
            0x000 => Ok(Instruction::Ecall),
            0x001 => Ok(Instruction::Ebreak),
            funct12 => Err(ZkvmError::InvalidFunct12 { word, funct12 }),
        },
        0b001 => Ok(Instruction::Csr(
            CsrOp::Csrrw,
            CsrType {
                rd: rd(word),
                rs1_or_zimm: rs1(word),
                csr: csr(word),
                uses_immediate: false,
            },
        )),
        0b010 => Ok(Instruction::Csr(
            CsrOp::Csrrs,
            CsrType {
                rd: rd(word),
                rs1_or_zimm: rs1(word),
                csr: csr(word),
                uses_immediate: false,
            },
        )),
        0b011 => Ok(Instruction::Csr(
            CsrOp::Csrrc,
            CsrType {
                rd: rd(word),
                rs1_or_zimm: rs1(word),
                csr: csr(word),
                uses_immediate: false,
            },
        )),
        0b101 => Ok(Instruction::Csr(
            CsrOp::Csrrwi,
            CsrType {
                rd: rd(word),
                rs1_or_zimm: rs1(word),
                csr: csr(word),
                uses_immediate: true,
            },
        )),
        0b110 => Ok(Instruction::Csr(
            CsrOp::Csrrsi,
            CsrType {
                rd: rd(word),
                rs1_or_zimm: rs1(word),
                csr: csr(word),
                uses_immediate: true,
            },
        )),
        0b111 => Ok(Instruction::Csr(
            CsrOp::Csrrci,
            CsrType {
                rd: rd(word),
                rs1_or_zimm: rs1(word),
                csr: csr(word),
                uses_immediate: true,
            },
        )),
        funct3 => Err(ZkvmError::InvalidFunct3 {
            word,
            opcode: opcode(word),
            funct3,
        }),
    }
}

const fn opcode(word: u32) -> u8 {
    (word & 0x7f) as u8
}

const fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1f) as u8
}

const fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x07) as u8
}

const fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1f) as u8
}

const fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

const fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7f) as u8
}

const fn funct12(word: u32) -> u16 {
    ((word >> 20) & 0x0fff) as u16
}

const fn csr(word: u32) -> u16 {
    ((word >> 20) & 0x0fff) as u16
}

const fn shamt(word: u32) -> u8 {
    ((word >> 20) & 0x1f) as u8
}

fn imm_i(word: u32) -> i32 {
    sign_extend(word >> 20, 12)
}

fn imm_s(word: u32) -> i32 {
    let imm = ((word >> 25) << 5) | ((word >> 7) & 0x1f);
    sign_extend(imm, 12)
}

fn imm_b(word: u32) -> i32 {
    let imm = (((word >> 31) & 0x1) << 12)
        | (((word >> 7) & 0x1) << 11)
        | (((word >> 25) & 0x3f) << 5)
        | (((word >> 8) & 0x0f) << 1);
    sign_extend(imm, 13)
}

fn imm_u(word: u32) -> i32 {
    (word & 0xfffff000) as i32
}

fn imm_j(word: u32) -> i32 {
    let imm = (((word >> 31) & 0x1) << 20)
        | (((word >> 12) & 0xff) << 12)
        | (((word >> 20) & 0x1) << 11)
        | (((word >> 21) & 0x03ff) << 1);
    sign_extend(imm, 21)
}

fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits as u32;
    ((value << shift) as i32) >> shift
}
