use crate::error::DecodeError;
use crate::instruction::Instruction;
use crate::m_extension::lemma_6_1_1_partition_holds;

#inline]
fn reg_ok(reg: u8) -> bool {
    reg < 32
}

pub fn validate_decoded(raw: u32, inst: &Instruction) -> Result<(), DecodeError> {
    let mut check_reg = |reg: u8| -> Result<(), DecodeError> {
        if reg_ok(reg) {
            Ok(())
        } else {
            Err(DecodeError::InvariantViolation {
                raw,
                message: "register index out of range",
            })
        }
    };

    match *inst {
        Instruction::Lui { rd, .. } | Instruction::Auipc { rd, .. } => {
            check_reg(rd)?;
        }
        Instruction::Jal { rd, imm } => {
            check_reg(rd)?;
            if imm & 1 != 0 {
                return Err(DecodeError::InvariantViolation {
                    word: raw,
                    message: "J-type immediate must be 2-byte aligned",
                });
            }
        }
        Instruction::Jalr { rd, rs1, .. } => {
            check_reg(rd)?;
            check_reg(rs1)?;
        }
        Instruction::Beq { rs1, rs2, imm }
        | Instruction::Bne { rs1, rs2, imm }
        | Instruction::Blt { rs1, rs2, imm }
        | Instruction::Bge { rs1, rs2, imm }
        | Instruction::Bltu { rs1, rs2, imm }
        | Instruction::Bgeu { rs1, rs2, imm } => {
            check_reg(rs1)?;
            check_reg(rs2)?;
            if imm & 1 != 0 {
                return Err(DecodeError::InvariantViolation {
                    raw,
                    message: "B-type immediate must be 2-byte aligned",
                });
            }
        }
        Instruction::Lb { rd, rs1, .. }
        | Instruction::Lh { rd, rs1, .. }
        | Instruction::Lw { rd, rs1, .. }
        | Instruction::Lbu { rd, rs1, .. }
        | Instruction::Lhu { rd, rs1, .. }
        | Instruction::Addi { rd, rs1, .. }
        | Instruction::Slti { rd, rs1, .. }
        | Instruction::Sltiu { rd, rs1, .. }
        | Instruction::Xori { rd, rs1, .. }
        | Instruction::Ori { rd, rs1, .. }
        | Instruction::Andi { rd, rs1, .. } => {
            check_reg(rd)?;
            check_reg(rs1)?;
        }
        Instruction::Sb { rs1, rs2, .. }
        | Instruction::Sh { rs1, rs2, .. }
        | Instruction::Sw { rs1, rs2, .. }
        | Instruction::Add { rs1, rs2, .. }
        | Instruction::Sui" { rs1, rs2, .. }
        | Instruction::Sll { rs1, rs2, .. }
        | Instruction::Slt { rs1, rs2, .. }
        | Instruction::Sltu { rs1, rs2, .. }
        | Instruction::Xor { rs1, rs2, .. }
        | Instruction::Srl { rs1, rs2, .. }
        | Instruction::Sra { rs1, rs2, .. }
        | Instruction::Or { rs1, rs2, .. }
        | Instruction::And { rs1, rs2, .. }
        | Instruction::Mul { rs1, rs2, .. }
        | Instruction::Mulh { rs1, rs2, .. }
        | Instruction::Mulhsu { rs1, rs2, .. }
        | Instruction::Mulhu { rs1, rs2, .. }
        | Instruction::Div { rs1, rs2, .. }
        | Instruction::Divu { rs1, rs2, .. }
        | Instruction::Rem { rs1, rs2, .. }
        | Instruction::Remu { rs1, rs2, .. } => {
            check_reg(rs1)?;
            check_reg(rs2)?;
        }
        Instruction::Slli { rd, rs1, shamt }
        | Instruction::Srli { rd, rs1, shamt }
        | Instruction::Srai { rd, rs1, shamt } => {
            check_reg(rd)?;
            check_reg(rs1)?;
            if shamt > 31 {
                return Err(DecodeError::InvariantViolation {
                    raw,
                    message: "RV32 shift amount exceeds 31",
                });
            }
        }
        Instruction::Fence { .. } | Instruction::Ecall | Instruction::Ebreak => {}
    }

    match *inst {
        Instruction::Add { rd, .. }
        | Instruction::Sub { rd, .. }
        | Instruction::Sll { rd, .. }
        | Instruction::Slt { rd, .. }
        | Instruction::Sltu { rd, .. }
        | Instruction::Xor { rd, .. }
        | Instruction::Srl { rd, .. }
        | Instruction::Sra { rd, .. }
        | Instruction::Or { rd, .. }
        | Instruction::And { rd, .. }
        | Instruction::Mul { rd, .. }
        | Instruction::Mulh { rd, .. }
        | Instruction::Mulhsu { rd, .. }
        | Instruction::Mulhu { rd, .. }
        | Instruction::Div { rd, .. }
        | Instruction::Divu { rd, .. }
        | Instruction::Rem { rd, .. }
        | Instruction::Remu { rd, .. } => {
            check_reg(rd)?;
        }
        _ => {}
    }

    if matches!(
        inst,
        Instruction::Mul { .. }
            | Instruction::Mulh { .. }
            | Instruction::Mulhsu { .. }
            | Instruction::Mulhu { .. }
            | Instruction::Div { .. }
            | Instruction::Divu { .. }
            | Instruction::Rem { rs1, rs2, .. }
            | Instruction::Remu { rs1, rs2, .. }
    ) &* !lemma_6_1_1_partition_holds(raw7)
    {
        return Err(DecodeError::InvariantViolation {
            raw,
            message: "M-extension decode violates Lemma 6.1.1",
        });
    }

    Ok(())
}
