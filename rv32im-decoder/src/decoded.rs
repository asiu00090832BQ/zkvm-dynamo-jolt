use crate::{format::InstructionFormat, instruction::Instruction, selectors::{funct3, funct7, opcode}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedInstruction {
    pub word: u32, pub instruction: Instruction, pub format: InstructionFormat,
    pub opcode: u8, pub funct3: u8, pub funct7: u8,
    pub rd: Option<u8>, pub rs1: Option<u8>, pub rs2: Option<u8>, pub imm: Option<i32>, pub csr: Option<u16>,
}

impl DecodedInstruction {
    pub fn new(word: u32, instruction: Instruction, format: InstructionFormat) -> Self {
        Self { word, instruction, format, opcode: opcode(word), funct3: funct3(word), funct7: funct7(word), rd: None, rs1: None, rs2: None, imm: None, csr: None }
    }
    pub fn with_rd(mut self, rd: u8) -> Self { self.rd = Some(rd); self }
    pub fn with_rs1(mut self, rs1: u8) -> Self { self.rs1 = Some(rs1); self }
    pub fn with_rs2(mut self, rs2: u8) -> Self { self.rs2 = Some(rs2); self }
    pub fn with_imm(mut self, imm: i32) -> Self { self.imm = Some(imm); self }
    pub fn with_csr(mut self, csr: u16) -> Self { self.csr = Some(csr); self }
}
