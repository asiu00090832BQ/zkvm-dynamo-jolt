use crate::error::DecodeError;
use crate::instruction::*;
pub fn decode(raw: u32) -> Result<Instruction, DecodeError> {
    if raw & 0b11 != 0b11 { return Err(DecodeError::InstructionNot32Bit { raw }); }
    let opcode = opcode(raw);
    match opcode {
        0x37 => Ok(Instruction::Lui { rd: rd(raw)?, imm: imm_u(raw) }),
        0x17 => Ok(Instruction::Auipc { rd: rd(raw)?, imm: imm_u(raw) }),
        0x6f => Ok(Instruction::Jal { rd: rd(raw)?, offset: j_imm(raw) }),
        0x67 => decode_jalr(raw),
        0x63 => decode_branch(raw),
        0x03 => decode_load(raw),
        0x23 => decode_store(raw),
        0x13 => decode_op_imm(raw),
        0x33 => decode_op(raw),
        0x0f => decode_misc_mem(raw),
        0x73 => decode_system(raw),
        _ => Err(DecodeError::UnsupportedOpcode { raw, opcode }),
    }
}
fn decode_jalr(raw: u32) -> Result<Instruction, DecodeError> {
    if funct3(raw) != 0 { return Err(DecodeError::UnsupportedFunct3 { raw, opcode: opcode(raw), funct3: funct3(raw) }); }
    Ok(Instruction::Jalr { rd: rd(raw)?, rs1: rs1(raw)?, offset: imm_i(raw) })
}
fn decode_branch(raw: u32) -> Result<Instruction, DecodeError> {
    let kind = match funct3(raw) { 0b000 => BranchKind::Beq, 0b001 => BranchKind::Bne, 0b100 => BranchKind::Blt, 0b101 => BranchKind::Bge, 0b110 => BranchKind::Bltu, 0b111 => BranchKind::Bgeu, f => return Err(DecodeError::UnsupportedFunct3 { raw, opcode: opcode(raw), funct3: f }) };
    Ok(Instruction::Branch { kind, rs1: rs1(raw)?, rs2: rs2(raw)?, offset: imm_b(raw) })
}
fn decode_load(raw: u32) -> Result<Instruction, DecodeError> {
    let kind = match funct3(raw) { 0b000 => LoadKind::Lb, 0b001 => LoadKind::Lh, 0b010 => LoadKind::Lw, 0b100 => LoadKind::Lbu, 0b101 => LoadKind::Lhu, f => return Err(DecodeError::UnsupportedFunct3 { raw, opcode: opcode(raw), funct3: f }) };
    Ok(Instruction::Load { kind, rd: rd(raw)?, rs1: rs1(raw)?, imm: imm_i(raw) })
}
fn decode_store(raw: u32) -> Result<Instruction, DecodeError> {
    let kind = match funct3(raw) { 0b000 => StoreKind::Sb, 0b001 => StoreKind::Sh, 0b010 => StoreKind::Sw, f => return Err(DecodeError::UnsupportedFunct3 { raw, opcode: opcode(raw), funct3: f }) };
    Ok(Instruction::Store { kind, rs1: rs1(raw)?, rs2: rs2(raw)?, imm: imm_s(raw) })
}
fn decode_op_imm(raw: u32) -> Result<Instruction, DecodeError> {
    match funct3(raw) {
        0b000 => Ok(Instruction::OpImm { kind: OpImmKind::Addi, rd: rd(raw)?, rs1: rs1(raw)?, imm: imm_i(raw) }),
        0b010 => Ok(Instruction::OpImm { kind: OpImmKind::Slti, rd: rd(raw)?, rs1: rs1(raw)?, imm: imm_i(raw) }),
        0b011 => Ok(Instruction::OpImm { kind: OpImmKind::Sltiu, rd: rd(raw)?, rs1: rs1(raw)?, imm: imm_i(raw) }),
        0b100 => Ok(Instruction::OpImm { kind: OpImmKind::Xori, rd: rd(raw)?, rs1: rs1(raw)?, imm: imm_i(raw) }),
        0b110 => Ok(Instruction::OpImm { kind: OpImmKind::Ori, rd: rd(raw)?, rs1: rs1(raw)?, imm: imm_i(raw) }),
        0b111 => Ok(Instruction::OpImm { kind: OpImmKind::Andi, rd: rd(raw)?, rs1: rs1(raw)?, imm: imm_i(raw) }),
        0b001 => Ok(Instruction::OpImmShift { kind: ShiftImmKind::Sll, rd: rd(raw)?, rs1: rs1(raw)?, shamt: shamt(raw) }),
        0b101 => match funct7(raw) { 0b0000000 => Ok(Instruction::OpImmShift { kind: ShiftImmKind::Srl, rd: rd(raw)?, rs1: rs1(raw)?, shamt: shamt(raw) }), 0b0100000 => Ok(Instruction::OpImmShift { kind: ShiftImmKind::Sra, rd: rd(raw)?, rs1: rs1(raw)?, shamt: shamt(raw) }), f => Err(DecodeError::UnsupportedFunct7 { raw, opcode: 0x13, funct3: 0b101, funct7: f }) },
        f => Err(DecodeError::UnsupportedFunct3 { raw, opcode: 0x13, funct3: f }),
    }
}
fn decode_op(raw: u32) -> Result<Instruction, DecodeError> {
    let rd = rd(raw)?; let rs1 = rs1(raw)?; let rs2 = rs2(raw)?;
    match funct7(raw) {
        0b0000000 => match funct3(raw) { 0b000 => Ok(Instruction::Op { kind: OpKind::Add, rd, rs1, rs2 }), 0b001 => Ok(Instruction::Op { kind: OpKind::Sll, rd, rs1, rs2 }), 0b010 => Ok(Instruction::Op { kind: OpKind::Slt, rd, rs1, rs2 }), 0b011 => Ok(Instruction::Op { kind: OpKind::Sltu, rd, rs1, rs2 }), 0b100 => Ok(Instruction::Op { kind: OpKind::Xor, rd, rs1, rs2 }), 0b101 => Ok(Instruction::Op { kind: OpKind::Srl, rd, rs1, rs2 }), 0b110 => Ok(Instruction::Op { kind: OpKind::Or, rd, rs1, rs2 }), 0b111 => Ok(Instruction::Op { kind: OpKind::And, rd, rs1, rs2 }), f => Err(DecodeError::UnsupportedFunct3 { raw, opcode: 0x33, funct3: f }) },
        0b0100000 => match funct3(raw) { 0b000 => Ok(Instruction::Op { kind: OpKind::Sub, rd, rs1, rs2 }), 0b101 => Ok(Instruction::Op { kind: OpKind::Sra, rd, rs1, rs2 }), f => Err(DecodeError::UnsupportedFunct3 { raw, opcode: 0x33, funct3: f }) },
        0b0000001 => match funct3(raw) { 0b000 => Ok(Instruction::MulDiv { kind: MulDivKind::Mul, rd, rs1, rs2 }), 0b001 => Ok(Instruction::MulDiv { kind: MulDivKind::Mulh, rd, rs1, rs2 }), 0b010 => Ok(Instruction::MulDiv { kind: MulDivKind::Mulhsu, rd, rs1, rs2 }), 0b011 => Ok(Instruction::MulDiv { kind: MulDivKind::Mulhu, rd, rs1, rs2 }), 0b100 => Ok(Instruction::MulDiv { kind: MulDivKind::Div, rd, rs1, rs2 }), 0b101 => Ok(Instruction::MulDiv { kind: MulDivKind::Divu, rd, rs1, rs2 }), 0b110 => Ok(Instruction::MulDiv { kind: MulDivKind::Rem, rd, rs1, rs2 }), 0b111 => Ok(Instruction::MulDiv { kind: MulDivKind::Remu, rd, rs1, rs2 }), f => Err(DecodeError::UnsupportedFunct3 { raw, opcode: 0x33, funct3: f }) },
        f => Err(DecodeError::UnsupportedFunct7 { raw, opcode: 0x33, funct3: funct3(raw), funct7: f }),
    }
}
fn decode_misc_mem(raw: u32) -> Result<Instruction, DecodeError> {
    match funct3(raw) { 0b000 => Ok(Instruction::Fence { pred: ((raw >> 24) & 0x0f) as u8, succ: ((raw >> 20) & 0x0f) as u8 }), 0b001 => Ok(Instruction::FenceI), f => Err(DecodeError::UnsupportedFunct3 { raw, opcode: 0x0f, funct3: f }) }
}
fn decode_system(raw: u32) -> Result<Instruction, DecodeError> {
    match ((raw >> 20) & 0x0fff) as u16 { 0x000 => Ok(Instruction::Ecall), 0x001 => Ok(Instruction::Ebreak), _ => Err(DecodeError::UnsupportedSystem { raw, funct3: funct3(raw), imm12: ((raw >> 20) & 0x0fff) as u16 }) }
}
fn opcode(r: u32) -> u8 { (r & 0x7f) as u8 }
fn funct3(r: u32) -> u8 { ((r >> 12) & 0x07) as u8 }
fn funct7(r: u32) -> u8 { ((r >> 25) & 0x7f) as u8 }
fn rd(r: u32) -> Result<Register, DecodeError> { reg("rd", ((r >> 7) & 0x1f) as u8) }
fn rs1(r: u32) -> Result<Register, DecodeError> { reg("rs1", ((r >> 15) & 0x1f) as u8) }
fn rs2(r: u32) -> Result<Register, DecodeError> { reg("rs2", ((r >> 20) & 0x1f) as u8) }
fn reg(f: &'static str, r: u8) -> Result<Register, DecodeError> { if r <= 31 { Ok(r) } else { Err(DecodeError::InvalidRegister { field: f, reg: r }) } }
fn shamt(r: u32) -> u8 { ((r >> 20) & 0x1f) as u8 }
fn imm_i(r: u32) -> i32 { sign_extend(r >> 20, 12) }
fn imm_s(r: u32) -> i32 { sign_extend(((r >> 25) << 5) | ((r >> 7) & 0x1f), 12) }
fn imm_b(r: u32) -> i32 { sign_extend(((r >> 31) << 12) | (((r >> 7) & 0x01) << 11) | (((r >> 25) & 0x3f) << 5) | (((r >> 8) & 0x0f) << 1), 13) }
fn imm_u(r: u32) -> i32 { (r & 0xffff_f000) as i32 }
fn j_imm(r: u32) -> i32 { sign_extend(((r >> 31) << 20) | (((r >> 12) & 0xff) << 12) | (((r >> 20) & 0x01) << 11) | (((r >> 21) & 0x03ff) << 1), 21) }
fn sign_extend(v: u32, b: u32) -> i32 { let s = 32 - b; ((v << s) as i32) >> s }