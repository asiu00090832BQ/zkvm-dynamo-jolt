use crate::{
    decode::m_extension::{decode_rv32m, Rv32mInstruction},
    error::ZkvmError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub xlen: u8,
    pub enable_rv32m: bool,
    pub increment_pc: bool,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        match Self {
            xlen: 32,
            enable_rv32m: true,
            increment_pc: true,
        }
    }
}

pub trait Zkvm {
    fn config(&self) -> &ZkvmConfig;
    fn registers(&self) -> &[u32; 32];
    fn registers_mut(&mut self) -> &mut [u32; 32];
    fn program_counter(&self) -> u32;
    fn set_program_counter(&mut self, pc: u32);

    fn execute_instruction(&mut self, word: u32) -> Result<(), ZkvmError> {
        if self.config().xlen != 32 {
            return Err(ZkvmError::VerificationFailed("RV32M requires xlen = 32"));
        }

        if !self.config().enable_rv32m {
            return Err(ZkvmError::ExtensionDisabled("rv32m"));
        }

        let instruction = decode_rv32m(word)?.ok_or(ZkvmError::InvalidInstruction(word))?;
        self.execute_rv32m(instruction)?;
        self.registers_mut()[0] = 0;

        if self.config().increment_pc {
            let next_pc = self.program_counter().wrapping_add(4);
            self.set_program_counter(next_pc);
        }

        Ok(())
    }

    fn execute_rv32m(&mut self, instruction: Rv32mInstruction) -> Result<(), ZkvmError>;
}
