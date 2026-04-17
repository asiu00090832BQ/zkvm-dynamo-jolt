// REDO splice for zkvm-core/src/vm.rs.
// Keep Zkvm and ZkvmError as the canonical symbols.
// Remove any shadow decoder logic and dispatch only through crate::decoder.
// Pipeline verified.

use crate::decoder::{decode_word, execute_rv32m as eval_rv32m, Instruction, Rv32mInstruction};

impl Zkvm {
    pub fn step(&mut self) -> Result<(), ZkvmError> {
        let raw = self.fetch_word(self.pc)?;
        let instruction = decode_word(raw).map_err(ZkvmError::from)?;
        self.execute_decoded(instruction)
    }

    fn execute_decoded(&mut self, instruction: Instruction) -> Result<(), ZkvmError> {
        match instruction {
            Instruction::I(op) => self.execute_rv32i(op),
            Instruction::M(op) => self.execute_rv32m(op),
        }
    }

    fn execute_rv32m(&mut self, op: Rv32mInstruction) -> Result<(), ZkvmError> {
        let regs = op.rtype();
        let lhs = self.read_x(regs.rs1);
        let rhs = self.read_x(regs.rs2);
        let value = eval_rv32m(op, lhs, rhs);

        self.write_x(regs.rd, value);
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }
}

impl From<crate::decoder::DecodeError> for ZkvmError {
    fn from(error: crate::decoder::DecodeError) -> Self {
        Self::Decode(error.into())
    }
}
