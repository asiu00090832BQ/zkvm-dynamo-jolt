use crate::{
    error::ZkvmError,
    types::{DecodeResult, Limb16, OperandDecomposition},
};

pub const fn decompose_operand_lemma(value: u32) -> Limb16 {
    Limb16::from_u32(value)
}

pub fn verify_lemma_6_1_1(a: u32, b: u32) -> DecodeResult<OperandDecomposition> {
    let decomposition = OperandDecomposition::from_operands(a, b);

    if decomposition.a.recompose() != a {
        return Err(ZkvmError::InvariantViolation(
            "Lemma 6.1.1 failed for operand A during 16-bit limb recomposition",
        ));
    }

    if decomposition.b.recompose() != b {
        return Err(ZkvmError::InvariantViolation(
            "Lemma 6.1.1 failed for operand B during 16-bit limb recomposition",
        ));
    }

    Ok(decomposition)
}

pub fn assert_valid_register(index: u8) -> DecodeResult<()> {
    if index < 32 {
        Ok(())
    } else {
        Err(ZkvmError::InvalidRegister { index })
    }
}

pub fn assert_decoded_registers(rd: u8, rs1: u8, rs2: u8) -> DecodeResult<()> {
    assert_valid_register(rd)?;
    assert_valid_register(rs1)?;
    assert_valid_register(rs2)?;
    Ok(())
}
