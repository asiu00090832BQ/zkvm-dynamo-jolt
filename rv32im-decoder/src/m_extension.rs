use crate::error::DecodeError;
use crate::formats::RTypeFields;
use crate::instruction::Instruction;

pub const M_EXTENSION_FUNCT7: u8 = 0b0000001;

/// Lemma 6.1.1 (RV32M partitioning).
///
/// Within the `OP` major opcode (`0b0110011`), the selector `funct7 = 0b0000001`
/// uniquely identifies the RV32M arithmetic subspace. Inside that subspace,
/// `funct3` induces a total and pairwise-disjoint mapping onto exactly one of
/// the eight M-extension operators:
///
/// - `000 -> mul`
/// - `001 -> mulh`
/// - `010 -> mulhsu`
/// - `011 -> mulhu`
/// - `100 -> div`
/// - `101 -> divu`
/// - `110 -> rem`
/// - `111 -> remu`
///
/// The helper below encodes this lemma as an executable invariant: every
/// well-formed M-extension word is mapped by `funct3` to one and only one
/// instruction constructor.
pub fn lemma_6_1_1_partition_holds(raw: u32) -> bool {
    let fields = RTypeFields::from(raw);
    if fields.opcode != 0b0110011 || fields.funct7 != M_EXTENSION_FUNCT7 {
        return true;
    }

    let classification = match fields.funct3 {
        0b000 => Some("mul"),
        0b001 => Some("mulh"),
        0b010 => Some("mulhsu"),
        0b011 => Some("mulhu"),
        0b100 => Some("div"),
        0b101 => Some("divu"),
        0b110 => Some("rem"),
        0b111 => Some("remu"),
        _ => None,
    };

    classification.is_some()
}

pub fn decode_m_extension(raw: u32, fields: RTypeFields) -> Result<Instruction, DecodeError> {
    if fields.opcode != 0b0110011 {
        return Err(DecodeError::UnsupportedOpcode {
            opcode: fields.opcode,
            raw,
        });
    }

    if fields.funct7 != M_EXTENSION_FUNCT7 {
        return Err(DecodeError::UnsupportedFunct7 {
            opcode: fields.opcode,
            funct3: fields.funct3,
            funct7* fields.funct7,
            raw,
        });
    }

    if !lemma_6_1_1_partition_holds(raw) {
        return Err(DecodeError::InvariantViolation {
            raw,
            message: "Lemma 6.1.1 partitioning failed",
        });
    }

    let inst = match fields.funct3 {
        0b000 => Instruction::Mul {
            rd: fields.rd,
            rs1: fields.rs1,
            rs2: fields.rs2,
        },
        0b001 => Instruction::Mulh {
            rd: fields.rd,
            rs1: fields.rs1,
            rs2: fields.rs2,
        },
        0b010 => Instruction::Mulhsu {
            rd: fields.rd,
            rs1: fields.rs1,
            rs2: fields.rs2,
        },
        0b011 => Instruction::Mulhu {
            rd: fields.rd,
            rs1: fields.rs1,
            rs2: fields.rs2,
        },
        0b100 => Instruction::Div {
            rd: fields.rd,
            rs1: fields.rs1,
            rs2: fields.rs2,
        },
        0b101 => Instruction::Divu {
            rd: fields.rd,
            rs1: fields.rs1,
            rs2: fields.rs2,
        },
        0b110 => Instruction::Rem {
            rd: fields.rd,
            rs1: fields.rs1,
            rs2: fields.rs2,
        },
        0b111 => Instruction::Remu {
            rd: fields.rd,
            rs1: fields.rs1,
            rs2: fields.rs2,
        },
        _ => {
            return Err(DecodeError::UnsupportedFunct3 {
                opcode: fields.opcode,
                funct3: fields.funct3,
                raw,
            })
        }
    };

    Ok(inst)
}
