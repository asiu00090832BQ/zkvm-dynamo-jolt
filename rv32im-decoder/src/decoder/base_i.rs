use crate::decoder::invariants;
use crate::error::{Result, ZkvmError};
use crate::fields::{BType, IType, JType, RType, RawInstruction, SType, UType};
use crate::instruction::{
    ArithmeticOp, BranchOp, Instruction, LoadOp, OpImmOp, StoreOp, SystemOp,
};

pub(crate) fn decode_base_i(raw: RawInstruction) -> Result<Instruction> {
    match raw.opcode() {
        0b0110111 => Ok(Instruction::Lui(UType::from(raw))),
        0b0010111 => Ok(Instruction::Auipc(UType::from(raw))),
        0b1101111 => Ok(Instruction::Jal(JType::from(raw))),
        0b1100111 => decode_jalr(raw),
        0b1100011 => decode_branch(raw),
        0b0000011 => decode_load(raw),
        0b0100011 => decode_store(raw),
        0b0010011 => decode_op_imm(raw),
        0b0110011 => decode_r_type(raw),
        0b0001111 => decode_misc_mem(raw),
        0b1110011 => decode_system(raw),
        _ => Err(ZkvmError::UnsupportedOpcode {
            opcode: raw.opcode(),
            word: raw.word(),
        }),
    }
}

pub(crate) fn decode_r_type(raw: RawInstruction) -> Result<Instruction> {
    invariants::validate_rtype_funct7(raw)?;
    let fields = RType::from(raw);

    let op = match (raw.funct3(), raw.funct7()) {
        (0b000, 0b0000000) => ArithmeticOp::Add,
        (0b000, 0b0100000) => ArithmeticOp::Sub,
        (0b001, 0b0000000) => ArithmeticOp::Sll,
        (0b010, 0b0000000) => ArithmeticOp::Slt,
        (0b011, 0b0000000) => ArithmeticOp::Sltu,
        (0b100, 0b0000000) => ArithmeticOp::Xor,
        (0b101, 0b0000000) => ArithmeticOp::Srl,
        (0b101, 0b0100000) => ArithmeticOp::Sra,
        (0b110, 0b0000000) => ArithmeticOp::Or,
        (0b111, 0b0000000) => ArithmeticOp::And,
        _ => {
            return Err(ZkvmError::UnsupportedFunct7 {
                funct7: raw.funct7(),
                funct3: raw.funct3(),
                opcode: raw.opcode(),
                word: raw.word(),
            })
        }
    };

    Ok(Instruction::Op(op, fields))
}

fn decode_jalr(raw: RawInstruction) -> Result<Instruction> {
    if raw.funct3() != 0b000 {
        return Err(ZkvmError::UnsupportedFunct3 {
            funct3: raw.funct3(),
            opcode: raw.opcode(),
            word: raw.word(),
        });
    }

    Ok(Instruction::Jalr(IType::from(raw)))
}

fn decode_branch(raw: RawInstruction) -> Result<Instruction> {
    let op = match raw.funct3() {
        0b000 => BranchOp::Beq,
        0b001 => BranchOp::Bne,
        0b100 => BranchOp::Blt,
        0b101 => BranchOp::Bge,
        0b110 => BranchOp::Bltu,
        0b111 => BranchOp::Bgeu,
        _ => {
            return Err(ZkvmError::UnsupportedFunct3 {
                funct3: raw.funct3(),
                opcode: raw.opcode(),
                word: raw.word(),
            })
        }
    };

    Ok(Instruction::Branch(op, BType::from(raw)))
}

fn decode_load(raw: RawInstruction) -> Result<Instruction> {
    let op = match raw.funct3() {
        0b000 => LoadOp::Lb,
        0b001 => LoadOp::Lh,
        0b010 => LoadOp::Lw,
        0b100 => LoadOp::Lbu,
        0b101 => LoadOp::Lhu,
        _ => {
            return Err(ZkvmError::UnsupportedFunct3 {
                funct3: raw.funct3(),
                opcode: raw.opcode(),
                word: raw.word(),
            })
        }
    };

    Ok(Instruction::Load(op, IType::from(raw)))
}

fn decode_store(raw: RawInstruction) -> Result<Instruction> {
    let op = match raw.funct3() {
        0b000 => StoreOp::Sb,
        0b001 => StoreOp::Sh,
        0b010 => StoreOp::Sw,
        _ => {
            return Err(ZkvmError::UnsupportedFunct3 {
                funct3: raw.funct3(),
                opcode: raw.opcode(),
                word: raw.word(),
            })
        }
    };

    Ok(Instruction::Store(op, SType::from(raw)))
}

fn decode_op_imm(raw: RawInstruction) -> Result<Instruction> {
    let op = match raw.funct3() {
        0b000 => OpImmOp::Addi,
        0b010 => OpImmOp::Slti,
        0b011 => OpImmOp::Sltiu,
        0b100 => OpImmOp::Xori,
        0b110 => OpImmOp::Ori,
        0b111 => OpImmOp::Andi,
        0b001 => {
            if raw.funct7() != 0b0000000 {
                return Err(ZkvmError::UnsupportedFunct7 {
                    funct7: raw.funct7(),
                    funct3: raw.funct3(),
                    opcode: raw.opcode(),
                    word: raw.word(),
                });
            }
            OpImmOp::Slli
        }
        0b101 => {
            invariants::validate_shift_funct7(raw)?;
            match raw.funct7() {
                0b0000000 => OpImmOp::Srli,
                0b0100000 => OpImmOp::Srai,
                _ => {
                    return Err(ZkvmError::UnsupportedFunct7 {
                        funct7: raw.funct7(),
                        funct3: raw.funct3(),
                        opcode: raw.opcode(),
                        word: raw.word(),
                    })
                }
            }
        }
        _ => {
            return Err(ZkvmError::UnsupportedFunct3 {
                funct3: raw.funct3(),
                opcode: raw.opcode(),
                word: raw.word(),
            })
        }
    };

    Ok(Instruction::OpImm(op, IType::from(raw)))
}

fn decode_misc_mem(raw: RawInstruction) -> Result<Instruction> {
    match raw.funct3() {
        0b000 => Ok(Instruction::Fence),
        _ => Err(ZkvmError::UnsupportedFunct3 {
            funct3: raw.funct3(),
            opcode: raw.opcode(),
            word: raw.word(),
        }),
    }
}

fn decode_system(raw: RawInstruction) -> Result<Instruction> {
    match raw.word() {
        0x0000_0073 => Ok(Instruction::System(SystemOp::Ecall)),
        0x0010_0073 => Ok(Instruction::System(SystemOp::Ebreak)),
        _ => Err(ZkvmError::InvalidEncoding {
            word: raw.word(),
            reason: "only ecall and ebreak are supported in the decoder",
        }),
    }
}
