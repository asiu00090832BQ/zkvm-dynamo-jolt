use crate::base_i::decode_base_i;
use crate::error::DecodeError;
use crate::formats::RTypeFields;
use crate::instruction::Instruction;
use crate::invariants::validate_decoded;
use crate::m_extension::{decode_m_extension, M_EXTENSION_FUNCT7};

pub fn decode_word(raw: u32) -> Result<Instruction, DecodeError> {
    if raw & 0b11 != 0b11 {
        return Err(DecodeError::ReservedEncoding {
            raw,
            reason: "compressed and non-32-bit encodings are unsupported",
        });
    }

    let opcode = (raw & 0x7f) as u8;
    let inst = match opcode {
        0b0110011 => {
            let r = RTypeFields::from(raw);
            if r.funct7 == M_EXTENSION_FUNCT7 {
                decode_m_extension(raw, r)?
            } else {
                decode_base_i(raw)?
            }
        }
        _ => decode_base_i(raw)?,
    };

    validate_decoded(raw, &inst)?;
    Ok(inst)
}

pub fn decode_le_bytes(bytes: &[u8]) -> Result<Instruction, DecodeError> {
    if bytes.len() < 4 {
        return Err(DecodeError::TruncatedInput { len: bytes.len() });
    }

    let raw = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    decode_word(raw)
}

pub fn decode_stream(bytes: &[u8]) -> Result<Vec<Instruction>, DecodeError> {
    if bytes.len() % 4 != 0 {
        return Err(DecodeError::TruncatedInput { len: bytes.len() });
    }

    let mut out = Vec::with_capacity(bytes.len() / 4);
    for chunk in bytes.chunks_exact(4) {
        out.push(decode_le_bytes(chunk)?);
    }
    Ok(out)
}
