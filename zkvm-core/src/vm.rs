use crate::decoder::{decode_word, execute_rv32m, execute_i_extension, DecodedInstruction, MInstruction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zkvm {
    pub pc: u32,
    pub registers: [u32; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkvmError {
    Decode(crate::decoder::DecodeError),
    MemoryOutOfBounds(u32),
}

impl Zkvm {
    pub fn step(&mut self, instruction_word: u32) -> Result<(), ZkvmError> {
        let decoded = decode_word(instruction_word).map_err(ZkvmError::Decode)?;
        match decoded {
            DecodedInstruction::I(inst) => {
                let rs1_val = self.registers[inst.fields().rs1 as usize];
                let val = execute_i_extension(inst, rs1_val);
                self.set_reg(inst.fields().rd, val);
            }
            DecodedInstruction::M(inst) => {
                let rs1_val = self.registers[inst.fields().rs1 as usize];
                let rs2_val = self.registers[inst.fields().rs2 as usize];
                let val = execute_rv32m(inst, rs1_val, rs2_val);
                self.set_reg(inst.fields().rd, val);
            }
        }
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    fn set_reg(&mut self, rd: u8, val: u32) {
        if rd != 0 {
            self.registers[rd as usize] = val;
        }
    }
}
