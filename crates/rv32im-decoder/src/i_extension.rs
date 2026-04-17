use crate::types::{DecodeError, ITypeFields, OPCODE_OP_IMM};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IInstruction {
    Addi(ITypeFields),
    Slti(ITypeFields),
    Sltiu(ITypeFields),
    Xori(ITypeFields),
    Ori(ITypeFields),
    Andi(ITypeFields),
    Slli(u8, u8, u8),
    Srli(u8, u8, u8),
    Srai(u8, u8, u8),
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
        0b001 => Ok(IInstruction::Slli(fields.rd, fields.rs1, (raw >> 20 & 0x1f) as u8)),
        0b101 => {
            let funct7 = (raw >> 25) as u8;
            let shamt = (raw >> 20 & 0x1f) as u8;
            match funct7 {
                0b0000000 => Ok(IInstruction::Srli(fields.rd, fields.rs1, shamt)),
                0b0100000 => Ok(IInstruction::Srai(fields.rd, fields.rs1, shamt)),
                _ => Err(DecodeError::UnsupportedFunct7(opcode, funct3, funct7, raw)),
            }
        }
        _ => Err(DecodeError::UnsupportedFunct3(opcode, funct3, raw)),
    }
}

pub fn execute_i_extension(inst: IInstruction, rs1_val: u32) -> u32 {
    match inst {
        IInstruction::Addi(f) => rs1_val.wrapping_add(f.imm as u32),
        IInstruction::Slti(f) => if (rs1_val as i32) < f.imm { 1 } else { 0 },
        IInstruction::Sltiu(f) => if rs1_val < f.imm as u32 { 1 } else { 0 },
        IInstruction::Xori(f) => rs1_val ^ f.imm as u32,
        IInstruction::Ori(f) => rs1_val | f.imm as u32,
        IInstruction::Andi(f) => rs1_val & f.imm as u32,
        IInstruction::Slli(_, _, shamt) => rs1_val << shamt,
        IInstruction::Srli(_, _, shamt) => rs1_val >> shamt,
        IInstruction::Srai(_, _, shamt) => ((rs1_val as i32) >> shamt) as u32,
    }
}
