use crate::{
    bitfield::{funct3, funct7, imm_b, imm_i, imm_j, imm_s, imm_u, opcode, rd, rs1, rs2, shamt},
    Instruction, ZkvmError,
};

pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    match opcode(word) {
        0x37 => Ok(Instruction::Lui {
            rd: rd(word),
            imm: imm_u(word),
        }),
        0x17 => Ok(Instruction::Auipc {
            rd: rd(word),
            imm: imm_u(word),
        }),
        0x6f => Ok(Instruction::Jal {
            rd: rd(word),
            imm: imm_j(word),
        }),
        0x67 => decode_jalr(word),
        0x63 => decode_branch(word),
        0x03 => decode_load(word),
        0x23 => decode_store(word),
        0x13 => decode_op_imm(word),
        0x33 => decode_op(word),
        0x0f => decode_misc_mem(word),
        0x73 => decode_system(word),
        op => Err(ZkvmError::InvalidOpcode { opcode: op, word }),
    }
}

fn decode_jalr(word: u32) -> Result<Instruction, ZkvmError> {
    match funct3(word) {
        0x0 => Ok(Instruction::Jalr {
            rd: rd(word),
            rs1: rs1(word),
            imm: imm_i(word),
        }),
        f3 => Err(ZkvmError::InvalidFunct3 {
            opcode: opcode(word),
            funct3: f3,
            word,
        }),
    }
}

fn decode_branch(word: u32) -> Result<Instruction, ZkvmError> {
    let rs1 = rs1(word);
    let rs2 = rs2(word);
    let imm = imm_b(word);

    match funct3(word) {
        0x0 => Ok(Instruction::Beq { rs1, rs2, imm }),
        0x1 => Ok(Instruction::Bne { rs1, rs2, imm }),
        0x4 => Ok(Instruction::Blt { rs1, rs2, imm }),
        0x5 => Ok(Instruction::Bge { rs1, rs2, imm }),
        0x6 => Ok(Instruction::Bltu { rs1, rs2, imm }),
        0x7 => Ok(Instruction::Bgeu { rs1, rs2, imm }),
        f3 => Err(ZkvmError::InvalidFunct3 {
            opcode: opcode(word),
            funct3: f3,
            word,
        }),
    }
}

fn decode_load(word: u32) -> Result<Instruction, ZkvmError> {
    let rd = rd(word);
    let rs1 = rs1(word);
    let imm = imm_i(word);

    match funct3(word) {
        0x0 => Ok(Instruction::Lb { rd, rs1, imm }),
        0x1 => Ok(Instruction::Lh { rd, rs1, imm }),
        0x2 => Ok(Instruction::Lw { rd, rs1, imm }),
        0x4 => Ok(Instruction::Lbu { rd, rs1, imm }),
        0x5 => Ok(Instruction::Lhu { rd, rs1, imm }),
        f3 => Err(ZkvmError::InvalidFunct3 {
            opcode: opcode(word),
            funct3: f3,
            word,
        }),
    }
}

fn decode_store(word: u32) -> Result<Instruction, ZkvmError> {
    let rs1 = rs1(word);
    let rs2 = rs2(word);
    let imm = imm_s(word);

    match funct3(word) {
        0x0 => Ok(Instruction::Sb { rs1, rs2, imm }),
        0x1 => Ok(Instruction::Sh { rs1, rs2, imm }),
        0x2 => Ok(Instruction::Sw { rs1, rs2, imm }),
        f3 => Err(ZkvmError::InvalidFunct3 {
            opcode: opcode(word),
            funct3: f3,
            word,
        }),
    }
}

fn decode_op_imm(word: u32) -> Result<Instruction, ZkvmError> {
    let rd = rd(word);
    let rs1 = rs1(word);

    match funct3(word) {
        0x0 => Ok(Instruction::Addi {
            rd,
            rs1,
            imm: imm_i(word),
        }),
        0x2 => Ok(Instruction::Slti {
            rd,
            rs1,
            imm: imm_i(word),
        }),
        0x3 => Ok(Instruction::Sltiu {
            rd,
            rs1,
            imm: imm_i(word),
        }),
        0x4 => Ok(Instruction::Xori {
            rd,
            rs1,
            imm: imm_i(word),
        }),
        0x6 => Ok(Instruction::Ori {
            rd,
            rs1,
            imm: imm_i(word),
        }),
        0x7 => Ok(Instruction::Andi {
            rd,
            rs1,
            imm: imm_i(word),
        }),
        0x1 => match funct7(word) {
            0x00 => Ok(Instruction::Slli {
                rd,
                rs1,
                shamt: shamt(word),
            }),
            f7 => Err(ZkvmError::InvalidFunct7 {
                opcode: opcode(word),
                funct3: funct3(word),
                funct7: f7,
                word,
            }),
        },
        0x5 => match funct7(word) {
            0x00 => Ok(Instruction::Srli {
                rd,
                rs1,
                shamt: shamt(word),
            }),
            0x20 => Ok(Instruction::Srai {
                rd,
                rs1,
                shamt: shamt(word),
            }),
            f7 => Err(ZkvmError::InvalidFunct7 {
                opcode: opcode(word),
                funct3: funct3(word),
                funct7: f7,
                word,
            }),
        },
        f3 => Err(ZkvmError::InvalidFunct3 {
            opcode: opcode(word),
            funct3: f3,
            word,
        }),
    }
}

fn decode_op(word: u32) -> Result<Instruction, ZkvmError> {
    let rd = rd(word);
    let rs1 = rs1(word);
    let rs2 = rs2(word);

    match (funct7(word), funct3(word)) {
        (0x00, 0x0) => Ok(Instruction::Add { rd, rs1, rs2 }),
        (0x20, 0x0) => Ok(Instruction::Sub { rd, rs1, rs2 }),
        (0x00, 0x1) => Ok(Instruction::Sll { rd, rs1, rs2 }),
        (0x00, 0x2) => Ok(Instruction::Slt { rd, rs1, rs2 }),
        (0x00, 0x3) => Ok(Instruction::Sltu { rd, rs1, rs2 }),
        (0x00, 0x4) => Ok(Instruction::Xor { rd, rs1, rs2 }),
        (0x00, 0x5) => Ok(Instruction::Srl { rd, rs1, rs2 }),
        (0x20, 0x5) => Ok(Instruction::Sra { rd, rs1, rs2 }),
        (0x00, 0x6) => Ok(Instruction::Or { rd, rs1, rs2 }),
        (0x00, 0x7) => Ok(Instruction::And { rd, rs1, rs2 }),
        (f7, f3) if f7 != 0x00 && f7 != 0x20 => Err(ZkvmError::InvalidFunct7 {
            opcode: opcode(word),
            funct3: f3,
            funct7: f7,
            word,
        }),
        (_, f3) => Err(ZkvmError::InvalidFunct3 {
            opcode: opcode(word),
            funct3: f3,
            word,
        }),
    }
}

fn decode_misc_mem(word: u32) -> Result<Instruction, ZkvmError> {
    match funct3(word) {
        0x0 => {
            if rd(word) != 0 || rs1(word) != 0 {
                return Err(ZkvmError::UnsupportedInstruction {
                    word,
                    reason: "FENCE requires rd=x0 and rs1=x0",
                });
            }

            let imm = ((word >> 20) & 0x0fff) as u16;
            let succ = (imm & 0x0f) as u8;
            let pred = ((imm >> 4) & 0x0f) as u8;
            let fm = ((imm >> 8) & 0x0f) as u8;

            Ok(Instruction::Fence { fm, pred, succ })
        }
        0x1 => Err(ZkvmError::UnsupportedInstruction {
            word,
            reason: "FENCE.I is not part of the RV32I/RV32M target set",
        }),
        f3 => Err(ZkvmError::InvalidFunct3 {
            opcode: opcode(word),
            funct3: f3,
            word,
        }),
    }
}

fn decode_system(word: u32) -> Result<Instruction, ZkvmError> {
    if funct3(word) != 0x0 {
        return Err(ZkvmError::UnsupportedInstruction {
            word,
            reason: "CSR instructions are not supported",
        });
    }

    if rd(word) != 0 || rs1(word) != 0 {
        return Err(ZkvmError::UnsupportedInstruction {
            word,
            reason: "only ECALL and EBREAK are supported SYSTEM instructions",
        });
    }

    match (word >> 20) & 0x0fff {
        0x000 => Ok(Instruction::Ecall),
        0x001 => Ok(Instruction::Ebreak),
        _ => Err(ZkvmError::UnsupportedInstruction {
            word,
            reason: "unsupported SYSTEM instruction",
        }),
    }
}
