use crate::{
    bitfield::{funct3, funct7, opcode, rd, rs1, rs2},
    Instruction, ZkvmError,
};

pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    if opcode(word) != 0x33 {
        return Err(ZkvmError::InvalidOpcode {
            opcode: opcode(word),
            word,
        });
    }

    if funct7(word) != 0x01 {
        return Err(ZkvmError::InvalidFunct7 {
            opcode: opcode(word),
            funct3: funct3(word),
            funct7: funct7(word),
            word,
        });
    }

    let rd = rd(word);
    let rs1 = rs1(word);
    let rs2 = rs2(word);

    match funct3(word) {
        0x0 => Ok(Instruction::Mul { rd, rs1, rs2 }),
        0x1 => Ok(Instruction::Mulh { rd, rs1, rs2 }),
        0x2 => Ok(Instruction::Mulhsu { rd, rs1, rs2 }),
        0x3 => Ok(Instruction::Mulhu { rd, rs1, rs2 }),
        0x4 => Ok(Instruction::Div { rd, rs1, rs2 }),
        0x5 => Ok(Instruction::Divu { rd, rs1, rs2 }),
        0x6 => Ok(Instruction::Rem { rd, rs1, rs2 }),
        0x7 => Ok(Instruction::Remu { rd, rs1, rs2 }),
        f3 => Err(ZkvmError::InvalidFunct3 {
            opcode: opcode(word),
            funct3: f3,
            word,
        }),
    }
}

pub fn mul(a: u32, b: u32) -> u32 {
    let a = a as u64;
    let b = b as u64;

    let a0 = a & 0xFFFF;
    let a1 = a >> 16;
    let b0 = b & 0xFFFF;
    let b1 = b >> 16;

    let p00 = a0 * b0;
    let p01 = a0 * b1;
    let p10 = a1 * b0;
    let p11 = a1 * b1;

    let res = p00 + ((p01 + p10) << 16) + (p11 << 32);
    res as u32
}

pub fn mulh(a: u32, b: u32) -> u32 {
    let lhs = a as i32 as i64;
    let rhs = b as i32 as i64;
    ((lhs * rhs) >> 32) as u32
}

pub fn mulhsu(a: u32, b: u32) -> u32 {
    let lhs = a as i32 as i64;
    let rhs = b as i64;
    ((lhs * rhs) >> 32) as u32
}

pub fn mulhu(a: u32, b: u32) -> u32 {
    (((a as u64) * (b as u64)) >> 32) as u32
}

pub fn div(a: u32, b: u32) -> u32 {
    let lhs = a as i32;
    let rhs = b as i32;

    if rhs == 0 {
        u32::MAX
    } else if lhs == i32::MIN && rhs == -1 {
        lhs as u32
    } else {
        (lhs / rhs) as u32
    }
}

pub fn divu(a: u32, b: u32) -> u32 {
    if b == 0 {
        u32::MAX
    } else {
        a / b
    }
}

pub fn rem(a: u32, b: u32) -> u32 {
    let lhs = a as i32;
    let rhs = b as i32;

    if rhs == 0 {
        a
    } else if lhs == i32::MIN && rhs == -1 {
        0
    } else {
        (lhs % rhs) as u32
    }
}

pub fn remu(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        a % b
    }
}
