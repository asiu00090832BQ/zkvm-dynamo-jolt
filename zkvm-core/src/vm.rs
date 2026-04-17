use rv32im_decoder::{
    decode, mul_u32_via_u16_limbs, signed_mulh, signed_unsigned_mulh, unsigned_mulh, Instruction,
    ZkvmError,
};

#[derive(Clone, Debug)]
pub struct Zkvm {
    registers: [u32; 32],
    program: Vec<u32>,
    pc: u32,
    halted: bool,
}

impl Zkvm {
    pub fn new(program: Vec<u32>) -> Self {
        let mut registers = [0; 32];
        Self { registers, program, pc: 0, halted: false }
    }

    pub fn pc(&self) -> u32 { self.pc }
    pub fn halted(&self) -> bool { self.halted }

    pub fn register(&self, index: usize) -> Result<u32, ZkvmError> {
        self.registers.get(index).copied().ok_or(ZkvmError::RegisterOutOfBounds(index))
    }

    pub fn set_register(&mut self, index: usize, value: u32) -> Result<(), ZkvmError> {
        if index >= self.registers.len() { return Err(ZkvmError::RegisterOutOfBounds(index)); }
        if index != 0 { self.registers[index] = value; }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), ZkvmError> {
        while !self.halted {
            let index = (self.pc / 4) as usize;
            if index >= self.program.len() { break; }
            self.step()?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), ZkvmError> {
        if self.halted { return Ok(()); }
        let index = (self.pc / 4) as usize;
        let word = *self.program.get(index).ok_or(ZkvmError::PcOutOfBounds(self.pc))?;
        let instruction = decode(word)?;
        self.execute(instruction)
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), ZkvmError> {
        let current_pc = self.pc;
        match instruction {
            Instruction::Lui { rd, imm } => self.write(rd, imm as u32),
            Instruction::Auipc { rd, imm } => self.write(rd, current_pc.wrapping_add(imm as u32)),
            Instruction::Jal { rd, imm } => {
                self.write(rd, current_pc.wrapping_add(4));
                self.pc = current_pc.wrapping_add(imm as u32);
                return Ok(());
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let base = self.read(rs1)? as i32;
                let target = (base.wrapping_add(imm) as u32) & !1;
                self.write(rd, current_pc.wrapping_add(4));
                self.pc = target;
                return Ok(());
            }
            Instruction::Add { rd, rs1, rs2 } => {
                let val = self.read(rs1)?.wrapping_add(self.read(rs2)?);
                self.write(rd, val);
            }
            Instruction::Mul { rd, rs1, rs2 } => {
                let product = mul_u32_via_u16_limbs(self.read(rs1)?, self.read(rs2)?);
                self.write(rd, product as u32);
            }
            Instruction::Mulh { rd, rs1, rs2 } => {
                let val = signed_mulh(self.read(rs1)? as i32, self.read(rs2)? as i32);
                self.write(rd, val);
            }
            Instruction::Mulhsu { rd, rs1, rs2 } => {
                let val = signed_unsigned_mulh(self.read(rs1)? as i32, self.read(rs2)?);
                self.write(rd, val);
            }
            Instruction::Mulhu { rd, rs1, rs2 } => {
                let val = unsigned_mulh(self.read(rs1)?, self.read(rs2)?);
                self.write(rd, val);
            }
            Instruction::Ecall | Instruction::Ebreak => self.halted = true,
            _ => {}
        }
        self.pc = current_pc.wrapping_add(4);
        self.registers[0] = 0;
        Ok(())
    }

    fn read(&self, index: u8) -> Result<u32, ZkvmError> { self.register(index as usize) }
    fn write(&mut self, index: u8, value: u32) { if index != 0 { self.registers[index as usize] = value; } }
}
