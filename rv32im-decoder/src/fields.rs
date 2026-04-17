use crate::{register::Register, word::Word};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fields {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
    pub csr: u16,
}

impl Fields {
    pub const fn from_word(word: Word) -> Self {
        Self {
            opcode: word.opcode(),
            rd: word.rd(),
            funct3: word.funct3(),
            rs1: word.rs1(),
            rs2: word.rs2(),
            funct7: word.funct7(),
            csr: ((word.raw() >> 20) & 0x0fff) as u16,
        }
    }

    pub const fn rd_register(self) -> Register {
        Register::from_u5(self.rd)
    }

    pub const fn rs1_register(self) -> Register {
        Register::from_u5(self.rs1)
    }

    pub const fn rs2_register(self) -> Register {
        Register::from_u5(self.rs2)
    }
}
