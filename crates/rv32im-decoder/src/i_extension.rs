use crate::types::{DecoderError, ITypeFields, OPCODE_OP_IMM};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IInstruction {
    Addi(ITypeFields),
    Slti(ITypeFields),
    Sltiu(ITypeFields),
    Xori(ITypeFields),
    Ori(ITypeFields),
    Andi(ITypeFields),
    Slli(ITypeFields),
    Srli(ITypeFields),
    Srai(ITypeFields),
}

impl IInstruction {
    pub fn fields(self) -> ITypeFields {
        match self {
            Self::Addi(f) | Self::Slti(f) | Self::Sltiu(f) | Self::Xori(f) | Self::Ori(f) | Self::Andi(f) | Self::Slli(f) | Self::Srli(f) | Self::Srai(f) => f,
        }
    }
    pub fn rd(self) -> u8 { self.fields().rd }
    pub fn rs1(self) -> u8 { self.fields().rs1 }
}

pub fn decode_rv32i(raw: u32) -> Result<IInstruction, DecodeError> {
    let fields = ITypeFields::decode(raw);
    let opcode = (raw & 0x7f) as u8;
    let funct3 = fields.funct3;

    if opcode != OPCODE_OP_IMM {
        return Err(DecodeError::UnsupportedOpcode(raw));
    }

    match funct3 {
        0b000 => Ok(IInstruction::Addi(fields)),
        0b010 => Ok(IInstruction::Slti(fields)),
        0b011 => Ok(IInstruction::Sltiu(fields)),
        0b100 => Ok(IInstruction::Xori(fields)),
        0b110 => Ok(IInstruction::Ori(fields)),
        0b111 => Ok(IInstruction::Andi(fields)),
        0b001 => Ok(IInstruction::Slli(fields)),
        0b101 => {
            let funct7 = (raw >> 25) as u8;
            match funct7 {
                0b0000000 => Ok(IInstruction::Srli(fields)),
                0b0100000 => Ok(IInstruction::Srai(fields)),
                _ => Err(DecodeError::UnsupportedFunct7(opcode, funct3, funct7, raw)),
            }
        }
        _ => Err(DecodeError::UnsupportedFunct3(opcode, funct3, raw)),
    }
}

pub fn execute_i_extension(inst: IInstruction, rs1_val: u32) -> u32 {
    let f = inst.fields();
    let shamt = (f.raw >> 20 & 0x1f) as u8;
    match inst {
        IInstruction::Addi(_) => rs1_val.wrapping_add(f.imm as u32),
        IInstruction::Slti(_) => if (rs1_val as i32) < f.imm { 1 } else { 0 },
        IInstruction::Sltiu(_) => if rs1_val < f.imm as u32 { 1 } else { 0 },
        IInstruction::Xori(_) => rs1_val ^ f.imm as u32,
        IInstruction::Ori(_) => rs1_val | f.imm as u32,
        IInstruction::Andi(_) => rs1_val & f.imm as u32,
        IInstruction::Slli(_) => rs1_val << shamt,
        IInstruction::Srli(_) => rs1_val >> shamt,
        IInstruction::Srai(_) => ((rs1_val as i32) >> shamt) as u32,
    }
}
