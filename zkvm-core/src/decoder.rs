#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Inst {
    Lui { d: u8, i: i32 },
    Auipc { d: u8, i: i32 },
    Jal { d: u8, i: i32 },
    Jalr { d: u8, s1: u8, i: i32 },
    Beq { s1: u8, s2: u8, i: i32 },
    Bne { s1: u8, s2: u8, i: i32 },
    Blt { s1: u8, s2: u8, i: i32 },
    Bge { s1: u8, s2: u8, i: i32 },
    Bltu { s1: u8, s2: u8, i: i32 },
    Bgeu { s1: u8, s2: u8, i: i32 },
    Lb { d: u8, s1: u8, i: i32 },
    Lh { d: u8, s1: u8, i: i32 },
    Lw { d: u8, s1: u8, i: i32 },
    Lbu { d: u8, s1: u8, i: i32 },
    Lhu { d: u8, s1: u8, i: i32 },
    Sb { s1: u8, s2: u8, i: i32 },
    Sh { s1: u8, s2: u8, i: i32 },
    Sw { s1: u8, s2: u8, i: i32 },
    Addi { d: u8, s1: u8, i: i32 },
    Slti { d: u8, s1: u8, i: i32 },
    Sltiu { d: u8, s1: u8, i: i32 },
    Xori { d: u8, s1: u8, i: i32 },
    Ori { d: u8, s1: u8, i: i32 },
    Andi { d: u8, s1: u8, i: i32 },
    Slli { d: u8, s1: u8, h: u8 },
    Srli { d: u8, s1: u8, h: u8 },
    Srai { d: u8, s1: u8, h: u8 },
    Add { d: u8, s1: u8, s2: u8 },
    Sub { d: u8, s1: u8, s2: u8 },
    Sll { d: u8, s1: u8, s2: u8 },
    Slt { d: u8, s1: u8, s2: u8 },
    Sltu { d: u8, s1: u8, s2: u8 },
    Xor { d: u8, s1: u8, s2: u8 },
    Srl { d: u8, s1: u8, s2: u8 },
    Sra { d: u8, s1: u8, s2: u8 },
    Or { d: u8, s1: u8, s2: u8 },
    And { d: u8, s1: u8, s2: u8 },
    Mul { d: u8, s1: u8, s2: u8 },
    Mulh { d: u8, s1: u8, s2: u8 },
    Mulhsu { d: u8, s1: u8, s2: u8 },
    Mulhu { d: u8, s1: u8, s2: u8 },
    Div { d: u8, s1: u8, s2: u8 },
    Divu { d: u8, s1: u8, s2: u8 },
    Rem { d: u8, s1: u8, s2: u8 },
    Remu { d: u8, s1: u8, s2: u8 },
    Fence { f: u8, p: u8, s: u8 },
    Ecall,
    Ebreak,
}

#[derive(Debug)]
pub enum DecodeError {
    IllegalInstruction(u32),
}

pub trait Decoder {
    fn decode(word: u32) -> Result<Inst, DecodeError>;
}

#[inline]
fn sx(x: u32, b: u32) -> i32 {
    ((x << (32 - b)) as i32) >> (32 - b)
}

#[inline]
fn r(x: u32, n: u32) -> u8 {
    ((x >> n) & 31) as u8
}

pub fn decode(x: u32) -> Result<Inst, DecodeError> {
    use Inst::*;

    let o = x & 127;
    let d = r(x, 7);
    let f = (x >> 12) & 7;
    let s1 = r(x, 15);
    let s2 = r(x, 20);
    let g = (x >> 25) & 127;
    let u = (x & 0xfffff000) as i32;
    let ii = sx(x >> 20, 12);
    let si = sx(((x >> 25) << 5) | ((x >> 7) & 31), 12);
    let bi = sx(
        ((x >> 31) << 12) | (((x >> 7) & 1) << 11) | (((x >> 25) & 63) << 5) | (((x >> 8) & 15) << 1),
        13,
    );
    let ji = sx(
        ((x >> 31) << 20) | (((x >> 25) & 1023) << 1) | (((x >> 20) & 1) << 11) | (x & 0x000ff000),
        21,
    );

    match o {
        0x37 => Ok(Lui { d, i: u }),
        0x17 => Ok(Auipc { d, i: u }),
        0x6f => Ok(Jal { d, i: ji }),
        0x67 => match f {
            0 => Ok(Jalr { d, s1, i: ii }),
            _ => Err(DecodeError::IllegalInstruction(x)),
        },
        0x63 => match f {
            0 => Ok(Beq { s1, s2, i: bi }),
            1 => Ok(Bne { s1, s2, i: bi }),
            4 => Ok(Blt { s1, s2, i: bi }),
            5 => Ok(Bge { s1, s2, i: bi }),
            6 => Ok(Bltu { s1, s2, i: bi }),
            7 => Ok(Bgeu { s1, s2, i: bi }),
            _ => Err(DecodeError::IllegalInstruction(x)),
        },
        0x03 => match f {
            0 => Ok(Lb { d, s1, i: ii }),
            1 => Ok(Lh { d, s1, i: ii }),
            2 => Ok(Lw { d, s1, i: ii }),
            4 => Ok(Lbu { d, s1, i: ii }),
            5 => Ok(Lhu { d, s1, i: ii }),
            _ => Err(DecodeError::IllegalInstruction(x)),
        },
        0x23 => match f {
            0 => Ok(Sb { s1, s2, i: si }),
            1 => Ok(Sh { s1, s2, i: si }),
            2 => Ok(Sw { s1, s2, i: si }),
            _ => Err(DecodeError::IllegalInstruction(x)),
        },
        0x13 => match f {
            0 => Ok(Addi { d, s1, i: ii }),
            2 => Ok(Slti { d, s1, i: ii }),
            3 => Ok(Sltiu { d, s1, i: ii }),
            4 => Ok(Xori { d, s1, i: ii }),
            6 => Ok(Ori { d, s1, i: ii }),
            7 => Ok(Andi { d, s1, i: ii }),
            1 => match g {
                0 => Ok(Slli { d, s1, h: s2 }),
                _ => Err(DecodeError::IllegalInstruction(x)),
            },
            5 => match g {
                0 => Ok(Srli { d, s1, h: s2 }),
                32 => Ok(Srai { d, s1, h: s2 }),
                _ => Err(DecodeError::IllegalInstruction(x)),
            },
            _ => Err(DecodeError::IllegalInstruction(x)),
        },
        0x33 => match (g, f) {
            (0, 0) => Ok(Add { d, s1, s2 }),
            (32, 0) => Ok(Sub { d, s1, s2, }),
            (0, 1) => Ok(Sll { d, s1, s2 }),
            (0, 2) => Ok(Slt { d, s1, s2 }),
            (0, 3) => Ok(Sltu { d, s1, s2 }),
            (0, 4) => Ok(Xor { d, s1, s2 }),
            (0, 5) => Ok(Srl { d, s1, s2 }),
            (32, 5) => Ok(Sra { d, s1, s2, }),
            (0, 6) => Ok(Or { d, s1, s2, }),
            (0, 7) => Ok(And { d, s1, s2 }),
            (1, 0) => Ok(Mul { d, s1, s2 }),
            (1, 1) => Ok(Mulh { d, s1, s2 }),
            (1, 2) => Ok(Mulhsu { d, s1, s2 }),
            (1, 3) => Ok(Mulhu { d, s1, s2 }),
            (1, 4) => Ok(Div { d, s1, s2 }),
            (1, 5) => Ok(Divu { d, s1, s2 }),
            (1, 6) => Ok(Rem { d, s1, s2 }),
            (1, 7) => Ok(Remu { d, s1, s2 }),
            _ => Err(DecodeError::IllegalInstruction(x)),
        },
        0x0f => {
            if f == 0 && d == 0 && s1 == 0 {
                Ok(Fence {
                    f: ((x >> 28) & 15) as u8,
                    p: ((x >> 24) & 15) as u8,
                    s: ((x >> 20) & 15) as u8,
                })
            } else {
                Err(DecodeError::IllegalInstruction(x))
            }
        }
        0x73 => {
            if f == 0 && d == 0 && s1 == 0 {
                match x >> 20 {
                    0 => Ok(Ecall),
                    1 => Ok(Ebreak),
                    _ => Err(DecodeError::IllegalInstruction(x)),
                }
            } else {
                Err(DecodeError::IllegalInstruction(x))
            }
        }
        _ => Err(DecodeError::IllegalInstruction(x)),
    }
}
