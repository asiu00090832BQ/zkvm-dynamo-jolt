#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawFields {
    raw: u32,
}

impl RawFields {
    pub const fn new(raw: u32) -> Self {
        Self { raw }
    }

    pub const fn opcode(self) -> u8 {
        (self.raw & 0x7f) as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RType {
    raw: u32,
}

impl RType {
    pub const fn new(raw: u32) -> Self {
        Self { raw }
    }

    pub const fn rd(self) -> u8 {
        ((self.raw >> 7) & 0x1f) as u8
    }

    pub const fn funct3(self) -> u8 {
        ((self.raw >> 12) & 0x07) as u8
    }

    pub const fn rs1(self) -> u8 {
        ((self.raw >> 15) & 0x1f) as u8
    }

    pub const fn rs2(self) -> u8 {
        ((self.raw >> 20) & 0x1f) as u8
    }

    pub const fn funct7(self) -> u8 {
        ((self.raw >> 25) & 0x7f) as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IType {
    raw: u32,
}

impl IType {
    pub const fn new(raw: u32) -> Self {
        Self { raw }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SType {
    raw: u32,
}

impl SType {
    pub const fn new(raw: u32) -> Self {
        Self { raw }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BType {
    raw: u32,
}

impl BType {
    pub const fn new(raw: u32) -> Self {
        Self { raw }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UType {
    raw: u32,
}

impl UType {
    pub const fn new(raw: u32) -> Self {
        Self { raw }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JType {
    raw: u32,
}

impl JType {
    pub const fn new(raw: u32) -> Self {
        Self { raw }
    }
}

pub const fn sign_extend(value: u32, width: u8) -> i32 {
    let shift = 32 - width as u32;
    ((value << shift) as i32) >> shift
}