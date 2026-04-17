use crate::{
    error::ZkvmError,
    instruction::{DecodedInstruction, MInstruction, Rv32Opcode},
    invariants,
    types::DecodeResult,
    util,
};

fn build_m(word: u32, kind: MInstruction) -> DecodeResult<DecodedInstruction> {
    let rd = util::rd(word);
    let rs1 = util::rs1(word);
    let rs2 = util::rs2(word);

    invariants::assert_decoded_registers(rd, rs1, rs2)?;

    Ok(DecodedInstruction::new_m(
        word,
        Rv32Opcode::Op,
        kind,
        rd,
        rs1,
        rs2,
        util::funct3(word),
        util::funct7(word),
    ))
}

pub fn decode(word: u32) -> DecodeResult<DecodedInstruction> {
    if util::opcode(word) != 0b0110011 || util::funct7(word) != 0b0000001 {
        return Err(ZkvmError::UnsupportedInstruction {
            opcode: util::opcode(word),
            funct3: util::funct3(word),
            funct7: util::funct7(word),
        });
    }

    match util::funct3(word) {
        0b000 => build_m(word, MInstruction::Mul),
        0b001 => build_m(word, MInstruction::Mulh),
        0b010 => build_m(word, MInstruction::Mulhsu),
        0b011 => build_m(word, MInstruction::Mulhu),
        0b100 => build_m(word, MInstruction::Div),
        0b101 => build_m(word, MInstruction::Divu),
        0b110 => build_m(word, MInstruction::Rem),
        0b111 => build_m(word, MInstruction::Remu),
        _ => Err(ZkvmError::DecodeFault(
            "invalid funct3 for RV32M instruction",
        )),
    }
}
