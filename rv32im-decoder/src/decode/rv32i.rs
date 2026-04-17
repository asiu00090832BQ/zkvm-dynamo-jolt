use crate::{
    bits,
    decode::Register,
    error::{Result, ZkvmError},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RType {
    pub rd: Register,
    pub rs1: Register,
    pub rs2: Register,
}

impl RType {
    #[inline]
    pub fn decode(word: u32) -> Result<Self> {
        Ok(Self {
            rd: Register::new(bits::rd(word))?,
            rs1: Register::new(bits::rs1(word))?,
            rs2: Register::new(bits::rs2(word))?,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IType {
    pub rd: Register,
    pub rs1: Register,
    pub imm: i32,
}

impl IType {
    #[inline]
    pub fn decode(word: u32) -> Result<Self> {
        Ok(Self {
            rd: Register::new(bits::rd(word))?,
            rs1: Register::new(bits::rs1(word))?,
            imm: bits::imm_i(word),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShiftImmediate {
    pub rd: Register,
    pub rs1: Register,
    pub shamt: u8,
}

impl ShiftImmediate {
    #[inline]
    pub fn decode(word: u32) -> Result<Self> {
        Ok(Self {
            rd: Register::new(bits::rd(word))?,
            rs1: Register::new(bits::rs1(word))?,
            shamt: bits::shamt(word),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SType {
    pub rs1: Register,
    pub rs2: Register,
    pub imm: i32,
}

impl SType {
    #[inline]
    pub fn decode(word: u32) -> Result<Self> {
        Ok(Self {
            rs1: Register::new(bits::rs1(word))?,
            rs2: Register::new(bits::rs2(word))?,
            imm: bits::imm_s(word),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BType {
    pub rs1: Register,
    pub rs2: Register,
    pub imm: i32,
}

impl BType {
    #[inline]
    pub fn decode(word: u32) -> Result<Self> {
        Ok(Self {
            rs1: Register::new(bits::rs1(word))?,
            rs2: Register::new(bits::rs2(word))?,
            imm: bits::imm_b(word),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UType {
    pub rd: Register,
    pub imm: u32,
}

impl UType {
    #[inline]
    pub fn decode(word: u32) -> Result<Self> {
        Ok(Self {
            rd: Register::new(bits::rd(word))?,
            imm: bits::imm_u(word),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JType {
    pub rd: Register,
    pub imm: i32,
}

impl JType {
    #[inline]
    pub fn decode(word: u32) -> Result<Self> {
        Ok(Self {
            rd: Register::new(bits::rd(word))?,
            imm: bits::imm_j(word),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FenceOperands {
    pub pred: u8,
    pub succ: u8,
    pub fm: u8,
}

impl FenceOperands {
    #[inline]
    pub const fn decode(word: u32) -> Self {
        Self {
            pred: bits::fence_pred(word),
            succ: bits::fence_succ(word),
            fm: bits::fence_fm(word),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rv32iInstruction {
    Lui(UType),
    Auipc(UType),
    Jal(JType),
    Jalr(IType),
    Beq(BType),
    Bne(BType),
    Blt(BType),
    Bge(BType),
    Bltu(BType),
    Bgeu(BType),
    Lb(IType),
    Lh(IType),
    Lw(IType),
    Lbu(IType),
    Lhu(IType),
    Sb(SType),
    Sh(SType),
    Sw(SType),
    Addi(IType),
    Slti(IType),
    Sltiu(IType),
    Xori(IType),
    Ori(IType),
    Andi(IType),
    Slli(ShiftImmediate),
    Srli(ShiftImmediate),
    Srai(ShiftImmediate),
    Add(RType),
    Sub(RType),
    Sll(RType),
    Slt(RType),
    Sltu(RType),
    Xor(RType),
    Srl(RType),
    Sra(RType),
    Or(RType),
    And(RType),
    Fence(FenceOperands),
    Ecall,
    Ebreak,
}

#[inline]
pub fn decode_rv32i(word: u32) -> Result<Rv32iInstruction> {
    let opcode = bits::opcode(word);

    match opcode {
        0b0110111 => Ok(Rv32iInstruction::Lui(UType::decode(word)?)),
        0b0010111 => Ok(Rv32iInstruction::Auipc(UType::decode(word)?)),
        0b1101111 => Ok(Rv32iInstruction::Jal(JType::decode(word)?)),
        0b1100111 => match bits::funct3(word) {
            0b000 => Ok(Rv32iInstruction::Jalr(IType::decode(word)?)),
            funct3 => Err(ZkvmError::UnknownFunct3 { opcode, funct3, word }),
        },
        0b1100011 => {
            let operands = BType::decode(word)?;
            match bits::funct3(word) {
                0b000 => Ok(Rv32iInstruction::Beq(operands)),
                0b001 => Ok(Rv32iInstruction::Bne(operands)),
                0b100 => Ok(Rv32iInstruction::Blt(operands)),
                0b101 => Ok(Rv32iInstruction::Bge(operands)),
                0b110 => Ok(Rv32iInstruction::Bltu(operands)),
                0b111 => Ok(Rv32iInstruction::Bgeu(operands)),
                funct3 => Err(ZkvmError::UnknownFunct3 { opcode, funct3, word }),
            }
        }
        0b0000011 => {
            let operands = IType::decode(word)?;
            match bits::funct3(word) {
                0b000 => Ok(Rv32iInstruction::Lb(operands)),
                0b001 => Ok(Rv32iInstruction::Lh(operands)),
                0b010 => Ok(Rv32iInstruction::Lw(operands)),
                0b100 => Ok(Rv32iInstruction::Lbu(operands)),
                0b101 => Ok(Rv32iInstruction::Lhu(operands)),
                funct3 => Err(ZkvmError::UnknownFunct3 { opcode, funct3, word }),
            }
        }
        0b0100011 => {
            let operands = SType::decode(word)?;
            match bits::funct3(word) {
                0b000 => Ok(Rv32iInstruction::Sb(operands)),
                0b001 => Ok(Rv32iInstruction::Sh(operands)),
                0b010 => Ok(Rv32iInstruction::Sw(operands)),
                funct3 => Err(ZkvmError::UnknownFunct3 { opcode, funct3, word }),
            }
        }
        0b0010011 => match bits::funct3(word) {
            0b000 => Ok(Rv32iInstruction::Addi(IType::decode(word)?)),
            0b010 => Ok(Rv32iInstruction::Slti(IType::decode(word)?)),
            0b011 => Ok(Rv32iInstruction::Sltiu(IType::decode(word)?)),
            0b100 => Ok(Rv32iInstruction::Xori(IType::decode(word)?)),
            0b110 => Ok(Rv32iInstruction::Ori(IType::decode(word)?)),
            0b111 => Ok(Rv32iInstruction::Andi(IType::decode(word)?)),
            0b001 => {
                let operands = ShiftImmediate::decode(word)?;
                let funct7 = bits::funct7(word);
                if funct7 == 0b0000000 {
                    Ok(Rv32iInstruction::Slli(operands))
                } else {
                    Err(ZkvmError::InvalidShiftEncoding { funct7, word })
                }
            }
            0b101 => {
                let operands = ShiftImmediate::decode(word)?;
                match bits::funct7(word) {
                    0b0000000 => Ok(Rv32iInstruction::Srli(operands)),
                    0b0100000 => Ok(Rv32iInstruction::Srai(operands)),
                    funct7 => Err(ZkvmError::InvalidShiftEncoding { funct7, word }),
                }
            }
            funct3 => Err(ZkvmError::UnknownFunct3 { opcode, funct3, word }),
        },
        0b0110011 => {
            let operands = RType::decode(word)?;
            let funct3 = bits::funct3(word);
            let funct7 = bits::funct7(word);

            match (funct7, funct3) {
                (0b0000000, 0b000) => Ok(Rv32iInstruction::Add(operands)),
                (0b0100000, 0b000) => Ok(Rv32iInstruction::Sub(operands)),
                (0b0000000, 0b001) => Ok(Rv32iInstruction::Sll(operands)),
                (0b0000000, 0b010) => Ok(Rv32iInstruction::Slt(operands)),
                (0b0000000, 0b011) => Ok(Rv32iInstruction::Sltu(operands)),
                (0b0000000, 0b100) => Ok(Rv32iInstruction::Xor(operands)),
                (0b0000000, 0b101) => Ok(Rv32iInstruction::Srl(operands)),
                (0b0100000, 0b101) => Ok(Rv32iInstruction::Sra(operands)),
                (0b0000000, 0b110) => Ok(Rv32iInstruction::Or(operands)),
                (0b0000000, 0b111) => Ok(Rv32iInstruction::And(operands)),
                _ => Err(ZkvmError::UnknownFunct7 {
                    opcode,
                    funct3,
                    funct7,
                    word,
                }),
            }
        }
        0b0001111 => match bits::funct3(word) {
            0b000 => {
                if bits::rd(word) != 0 || bits::rs1(word) != 0 {
                    Err(ZkvmError::InvalidFenceEncoding { word })
                } else {
                    Ok(Rv32iInstruction::Fence(FenceOperands::decode(word)))
                }
            }
            funct3 => Err(ZkvmError::UnknownFunct3 { opcode, funct3, word }),
        },
        0b1110011 => match bits::funct3(word) {
            0b000 => {
                if bits::rd(word) != 0 || bits::rs1(word) != 0 {
                    return Err(ZkvmError::InvalidSystemEncoding {
                        imm12: bits::system_imm12(word),
                        word,
                    });
                }

                match bits::system_imm12(word) {
                    0x000 => Ok(Rv32iInstruction::Ecall),
                    0x001 => Ok(Rv32iInstruction::Ebreak),
                    imm12 => Err(ZkvmError::InvalidSystemEncoding { imm12, word }),
                }
            }
            funct3 => Err(ZkvmError::UnknownFunct3 { opcode, funct3, word }),
        },
        opcode => Err(ZkvmError::UnknownOpcode { opcode, word }),
    }
}
