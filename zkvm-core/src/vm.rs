use rv32im_decoder::{decode, execute_m_instruction, Instruction, ZkvmError};

pub struct ZkvmConfig { pub max_cycles: Option<u64> }
pub struct Zkvm {
    pub regs: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
    pub halted: bool,
    pub cycles: u64,
    pub config: ZkvmConfig,
}

impl Zkvm {
    pub fn step(&mut self) -> Result<(), ZkvmError> {
        let inst_word = self.load_u32(self.pc)?;
        let decoded = decode(inst_word).map_err(|e| ZkvmError::DecodeError(e.to_string()))?;
        self.execute(decoded.instruction, decoded.rd, decoded.rs1, decoded.rs2, decoded.imm)?;
        self.regs[0] = 0;
        self.cycles += 1;
        Ok(())
    }

    fn execute(&mut self, inst: Instruction, rd: u8, rs1: u8, rs2: u8, imm: i32) -> Result<(), ZkvmError> {
        let next_pc = self.pc.wrapping_add(4);
        match inst {
            Instruction::Add => { self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_add(self.regs[rs2 as usize]); self.pc = next_pc; }
            Instruction::Sub => { self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_sub(self.regs[rs2 as usize]); self.pc = next_pc; }
            Instruction::Mul | Instruction::Div | Instruction::Rem | Instruction::Mulh | Instruction::Mulhsu | Instruction::Mulhu | Instruction::Divu | Instruction::Remu => {
                let val = execute_m_instruction(inst, self.regs[rs1 as usize], self.regs[rs2 as usize]).unwrap();
                self.regs[rd as usize] = val;
                self.pc = next_pc;
            }
            Instruction::Ecall => { self.halted = true; }
            _ => { self.pc = next_pc; }
        }
        Ok(())
    }

    fn load_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        let a = addr as usize;
        if a + 4 > self.memory.len() { return Err(ZkvmError::MemoryOutOfBounds { addr, size: 4 }); }
        Ok(u32::from_le_bytes([self.memory[a], self.memory[a+1], self.memory[a+2], self.memory[a+3]]))
    }
}