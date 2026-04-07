use ark_ff::PrimeField;

/// RV32IM Instruction Decoder for zkvm-dynamo-jolt
/// Spec: Artifact 36D70C87 / Lemma 5.1

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add(Register, Register, Register),
    Addi(Register, Register, i32),
    Lw(Register, Register, i32),
    Sw(Register, Register, i32),
    Beq(Register, Register, i32),
    Jal(Register, i32),
    Jalr(Register, Register, i32),
    Lui(Register, u32),
    Auipc(Register, u32),
    Mul(Register, Register, Register),
    Mulh(Register, Register, Register),
    Div(Register, Register, Register),
    Rem(Register, Register, Register),
    Ecall,
    Ebreak,
    Unknown(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    X0 = 0, X1 = 1, X2 = 2, X3 = 3, X4 = 4, X5 = 5, X6 = 6, X7 = 7,
    X8 = 8, X9 = 9, X10 = 10, X11 = 11, X12 = 12, X13 = 13, X14 = 14, X15 = 15,
    X16 = 16, X17 = 17, X18 = 18, X19 = 19, X20 = 20, X21 = 21, X22 = 22, X23 = 23,
    X24 = 24, X25 = 25, X26 = 26, X27 = 27, X28 = 28, X29 = 29, X30 = 30, X31 = 31,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Csr {
    Mhartid = 0xf14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    InvalidOpcode(u32),
    InvalidRegister(u8),
    InvalidInstruction,
}

pub struct Decoder;

impl Decoder {
    pub fn decode(inst_word: u32) -> Result<Instruction, DecodeError> {
        let opcode = inst_word & 0x7F;
        let rd = ((inst_word >> 7) & 0x1F) as u8;
        let rs1 = ((inst_word >> 15) & 0x1F) as u8;
        let rs2 = ((inst_word >> 20) & 0x1F) as u8;

        match opcode {
            0x33 => {
                let funct3 = (inst_word >> 12) & 0x7;
                let funct7 = (inst_word >> 25) & 0x7F;
                if funct7 == 1 {
                    match funct3 {
                        0 => Ok(Instruction::Mul(Self::reg(rd)?, Self::reg(rs1)?, Self::reg(rs2)?)),
                        _ => Ok(Instruction::Unknown(inst_word)),
                    }
                } else {
                    Ok(Instruction::Add(Self::reg(rd)?, Self::reg(rs1)?, Self::reg(rs2)?))
                }
            }
            0x13 => {
                let imm = (inst_word as i32) >> 20;
                Ok(Instruction::Addi(Self::reg(rd)?, Self::reg(rs1)?, imm))
            }
            _ => Ok(Instruction::Unknown(inst_word)),
        }
    }

    fn reg(n: u8) -> Result<Register, DecodeError> {
        match n {
            0 => Ok(Register::X0), 1 => Ok(Register::X1), 2 => Ok(Register::X2), 3 => Ok(Register::X3),
            4 => Ok(Register::X4), 5 => Ok(Register::X5), 6 => Ok(Register::X6), 7 => Ok(Register::X7),
            8 => Ok(Register::X8), 9 => Ok(Register::X3),
            12 => Ok(Register::X12, 13 => Ok(Register::X13), 14 => Ok(Register::X14), 15 => Ok(Register::X15),
            16 => Ok(Register::X16), 17 => Ok(Register::X17), 18 => Ok(Register::X18), 19 => Ok(Register::X19),
            20 => Ok(Register::X20), 21 => Ok(Register::X21), 22 => Ok(Register::X22), 23 => Ok(Register::X23),
            24 => Ok(Register::X24), 25 => Ok(Register::X25), 26 => Ok(Register::X26), 27 => Ok(Register::X27),
            28 => Ok(Register::X28), 29 => Ok(Register::X29), 30 => Ok(Register::X30), 31 => Ok(Register::X31),
            _ => Err(DecodeError::InvalidRegister(n)),
        }
    }
}