#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawInstruction(pub u32);

impl RawInstruction {
    pub fn new(word: u32) -> Self {
        Self(word)
    }

    pub fn word(self) -> u32 {
        self.0
    }

    pub fn opcode(self) -> u8 {
        (self.0 & 0x7f) as u8
    }

    pub fn rd(self) -> u8 {
        ((self.0 >> 7) & 0x1f) as u8
    }

    pub fn funct3(self) -> u8 {
        ((self.0 >> 12) & 0x07) as u8
    }

    pub fn rs1(self) -> u8 {
        ((self.0 >> 15) & 0x1f) as u8
    }

    pub fn rs2(self) -> u8 {
        ((self.0 >> 20) & 0x1f) as u8
    }

    pub fn funct7(self) -> u8 {
        ((self.0 >> 25) & 0x7f) as u8
    }

    pub fn i_imm(self) -> i32 {
        sign_extend((self.0 >> 20) & 0x0fff, 12)
    }

    pub fn s_imm(self) -> i32 {
        let imm = (((self.0 >> 25) & 0x7f) << 5) | ((self.0 >> 7) & 0x1f);
        sign_extend(imm, 12)
    }

    pub fn b_imm(self) -> i32 {
        let imm = (((self.0 >> 31) & 0x01) << 12)
            | (((self.0 >> 7) & 0x01) << 11)
            | (((self.0 >> 25) & 0x3f) << 5)
            | (((self.0 >> 8) & 0x0f) << 1);
        sign_extend(imm, 13)
    }

    pub fn u_imm(self) -> i32 {
        (self.0 & 0xfffff000) as i32
    }

    pub fn j_imm(self) -> i32 {
        let imm = (((self.0 >> 31) & 0x01) << 20)
            | (((self.0 >> 12) & 0xff) << 12)
            | (((self.0 >> 20) & 0x01) << 11)
            | (((self.0 >> 21) & 0x03ff) << 1);
        sign_extend(imm, 21)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RType {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub funct7: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IType {
    pub rd: u8,
    pub rs1: u8,
    pub imm: i32,
    pub funct3: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SType {
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
    pub funct3: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BType {
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
    pub funct3: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UType {
    pub rd: u8,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JType {
    pub rd: u8,
    pub imm: i32,
}

impl From<RawInstruction> for RType {
    fn from(raw: RawInstruction) -> Self {
        Self {
            rd: raw.rd(),
            rs1: raw.rs1(),
            rs2: raw.rs2(),
            funct3: raw.funct3(),
            funct7: raw.funct7(),
        }
    }
}

impl From<RawInstruction> for IType {
    fn from(raw: RawInstruction) -> Self {
        Self {
            rd: raw.rd(),
            rs1: raw.rs1(),
            imm: raw.i_imm(),
            funct3: raw.funct3(),
        }
    }
}

impl From<RawInstruction> for SType {
    fn from(raw: RawInstruction) -> Self {
        Self {
            rs1: raw.rs1(),
            rs2: raw.rs2(),
            imm: raw.s_imm(),
            funct3: raw.funct3(),
        }
    }
}

impl From<RawInstruction> for BType {
    fn from(raw: RawInstruction) -> Self {
        Self {
            rs1: raw.rs1(),
            rs2: raw.rs2(),
            imm: raw.b_imm(),
            funct3: raw.funct3(),
        }
    }
}

impl From<RawInstruction> for UType {
    fn from(raw: RawInstruction) -> Self {
        Self {
            rd: raw.rd(),
            imm: raw.u_imm(),
        }
    }
}

impl From<RawInstruction> for JType {
    fn from(raw: RawInstruction) -> Self {
        Self {
            rd: raw.rd(),
            imm: raw.j_imm(),
        }
    }
}

fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32 - bits;
    ((value << shift) as i32) >> shift
}
