use rv32im_decoder::{
    decode, div_signed, div_unsigned, mul_low, mulh_signed, mulh_signed_unsigned, mulhu,
    rem_signed, rem_unsigned, Instruction, ZkvmError,
};

#[derive(Debug, Clone)]
pub struct Vm {
    regs: [u32; 32],
    pc: u32,
    memory: Vec<u8>,
    halted: bool,
}

impl Vm {
    pub fn new(memory_size: usize) -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
            memory: vec![0; memory_size],
            halted: false,
        }
    }

    pub fn with_memory(memory: Vec<u8>) -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
            memory,
            halted: false,
        }
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u32) -> Result<(), ZkvmError> {
        self.ensure_alignment(pc, 4)?;
        self.pc = pc;
        Ok(())
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn registers(&self) -> &[u32; 32] {
        &self.regs
    }

    pub fn registers_mut(&mut self) -> &mut [u32; 32] {
        &mut self.regs
    }

    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut [u8] {
        &mut self.memory
    }

    pub fn load_program(&mut self, addr: u32, program: &[u8]) -> Result<(), ZkvmError> {
        let start = self.checked_range(addr, program.len())?;
        self.memory[start..start + program.len()].copy_from_slice(program);
        Ok(())
    }

    pub fn fetch_word(&self) -> Result<u32, ZkvmError> {
        self.load_u32(self.pc)
    }

    pub fn decode_current(&self) -> Result<Instruction, ZkvmError> {
        decode(self.fetch_word()?)
    }

    pub fn step(&mut self) -> Result<(), ZkvmError> {
        if self.halted {
            return Err(ZkvmError::Halted);
        }

        let current_pc = self.pc;
        let next_pc = current_pc.wrapping_add(4);
        let instruction = self.decode_current()?;

        match instruction {
            Instruction::Lui { rd, imm } => {
                self.write_reg(rd, imm as u32)?;
                self.pc = next_pc;
            }
            Instruction::Auipc { rd, imm } => {
                self.write_reg(rd, current_pc.wrapping_add(imm as u32))?;
                self.pc = next_pc;
            }
            Instruction::Jal { rd, imm } => {
                self.write_reg(rd, next_pc)?;
                self.set_pc(current_pc.wrapping_add(imm as u32))?;
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.read_reg(rs1)?.wrapping_add(imm as u32) & !1;
                self.write_reg(rd, next_pc)?;
                self.set_pc(target)?;
            }
            Instruction::Beq { rs1, rs2, imm } => {
                if self.read_reg(rs1)? == self.read_reg(rs2)? {
                    self.set_pc(current_pc.wrapping_add(imm as u32))?;
                } else {
                    self.pc = next_pc;
                }
            }
            Instruction::BNE { rs1, rs2, imm } => {
                if self.read_reg(rs1)? != self.read_reg(rs2)? {
                    self.set_pc(current_pc.wrapping_add(imm as u32))?;
                } else {
                    self.pc = next_pc;
                }
            }
            Instruction::Blt { rs1, rs2, imm } => {
                if (self.read_reg(rs1)? as i32) < (self.read_reg(rs2)? as i32) {
                    self.set_pc(current_pc.wrapping_add(imm as u32))?;
                } else {
                    self.pc = next_pc;
                }
            }
            Instruction::Bge { rs1, rs2, imm } => {
                if (self.read_reg(rs1)? as i32) >= (self.read_reg(rs2)? as i32) {
                    self.set_pc(current_pc.wrapping_add(imm as u32))?;
                } else {
                    self.pc = next_pc;
                }
            }
            Instruction::Bltu { rs1, rs2, imm } => {
                if self.read_reg(rs1)? < self.read_reg(rs2)? {
                    self.set_pc(current_pc.wrapping_add(imm as u32))?;
                } else {
                    self.pc = next_pc;
                }
            }
            Instruction::Bgeu { rs1, rs2, imm } => {
                if self.read_reg(rs1)? >= self.read_reg(rs2)? {
                    self.set_pc(current_pc.wrapping_add(imm as u32))?;
                } else {
                    self.pc = next_pc;
                }
            }
            Instruction::Lb { rd, rs1, imm } => {
                let addr = self.read_reg(rs1)?.wrapping_add(imm as u32);
                let value = self.load_u8(addr)? as i8 as i32 as u32;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Lh { rd, rs1, imm } => {
                let addr = self.read_reg(rs1)?.wrapping_add(imm as u32);
                let value = self.load_u16(addr)? as i16 as i32 as u32;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Lw { rd, rs1, imm } => {
                let addr = self.read_reg(rs1)?.wrapping_add(imm as u32);
                let value = self.load_u32(addr)?;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Lbu { rd, rs1, imm } => {
                let addr = self.read_reg(rs1)?.wrapping_add(imm as u32);
                let value = self.load_u8(addr)? as u32;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Lhu { rd, rs1, imm } => {
                let addr = self.read_reg(rs1)?.wrapping_add(imm as u32);
                let value = self.load_u16(addr)? as u32;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Sb { rs1, rs2, imm } => {
                let addr = self.read_reg(rs1)?.wrapping_add(imm as u32);
                self.store_u8(addr, self.read_reg(rs2)? as u8)?;
                self.pc = next_pc;
            }
            Instruction::Sh { rs1, rs2, imm } => {
                let addr = self.read_reg(rs1)?.wrapping_add(imm as u32);
                self.store_u16(addr, self.read_reg(rs2)? as u16)?;
                self.pc = next_pc;
            }
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = self.read_reg(rs1)?.wrapping_add(imm as u32);
                self.store_u32(addr, self.read_reg(rs2)?)?;
                self.pc = next_pc;
            }
            Instruction::Addi { rd, rs1, imm } => {
                let value = self.read_reg(rs1)?.wrapping_add(imm as u32);
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Slti { rd, rs1, imm } => {
                let value = if (self.read_reg(rs1)? as i32) < imm { 1 } else { 0 };
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Sltiu { rd, rs1, imm } => {
                let value = if self.read_reg(rs1)? < imm as u32 { 1 } else { 0 };
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Xori { rd, rs1, imm } => {
                let value = self.read_reg(rs1)? ^ imm as u32;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Ori { rd, rs1, imm } => {
                let value = self.read_reg(rs1)? | imm as u32;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Andi { rd, rs1, imm } => {
                let value = self.read_reg(rs1)? & imm as u32;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Slli { rd, rs1, imm } => {
                let value = self.read_reg(rs1)? << (imm as u32 & 0x1f);
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Srli { rd, rs1, imm } => {
                let value = self.read_reg(rs1)? >> (imm as u32 & 0x1f);
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Srai { rd, rs1, imm } => {
                let value = ((self.read_reg(rs1)? as i32) >> (imm as u32 & 0x1f)) as u32;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Add { rd, rs1, rs2 } => {
                let value = self.read_reg(rs1)?.wrapping_add(self.read_reg(rs2)?);
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                let value = self.read_reg(rs1)?.wrapping_sub(self.read_reg(rs2)?);
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                let shamt = self.read_reg(rs2)? & 0x1f;
                let value = self.read_reg(rs1)? << shamt;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Slt { rd, rs1, rs2 } => {
                let value = if (self.read_reg(rs1)? as i32) < (self.read_reg(rs2)? as i32) {
                    1
                } else {
                    0
                };
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                let value = if self.read_reg(rs1)? < self.read_reg(rs2)? {
                    1
                } else {
                    0
                };
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                let value = self.read_reg(rs1)? ^ self.read_reg(rs2)?;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                let shamt = self.read_reg(rs2)? & 0x1f;
                let value = self.read_reg(rs1)? >> shamt;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                let shamt = self.read_reg(rs2)? & 0x1f;
                let value = ((self.read_reg(rs1)? as i32) >> shamt) as u32;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Or { rd, rs1, rs2 } => {
                let value = self.read_reg(rs1)? | self.read_reg(rs2)?;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::And { rd, rs1, rs2 } => {
                let value = self.read_reg(rs1)? & self.read_reg(rs2)?;
                self.write_reg(rd, value)?;
                self.pc = next_pc;
    -´ }
            Instruction::Mul { rd, rs1, rs2 } => {
                let value = mul_low(self.read_reg(rs1)?, self.read_reg(rs2)?);
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Mulh { rd, rs1, rs2 } => {
                let value = mulh_signed(self.read_reg(rs1)? as i32, self.read_reg(rs2)? as i32);
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Mulhsu { rd, rs1, rs2 } => {
                let value = mulh_signed_unsigned(self.read_reg(rs1)? as i32, self.read_reg(rs2)?);
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Mulhu { rd, rs1, rs2 } => {
                let value = mulhu(self.read_reg(rs1)?, self.read_reg(rs2)?);
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Div { rd, rs1, rs2 } => {
                let value = div_signed(self.read_reg(rs1)? as i32, self.read_reg(rs2)? as i32);
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Divu { rd, rs1, rs2 } => {
                let value = div_unsigned(self.read_reg(rs1)?, self.read_reg(rs2)?);
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Rem { rd, rs1, rs2 } => {
                let value = rem_signed(self.read_reg(rs1)? as i32, self.read_reg(rs2)? as i32);
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Remu { rd, rs1, rs2 } => {
                let value = rem_unsigned(self.read_reg(rs1)?, self.read_reg(rs2)?);
                self.write_reg(rd, value)?;
                self.pc = next_pc;
            }
            Instruction::Fence => {
                self.pc = next_pc;
            }
            Instruction::Ecall | Instruction::Ebreak => {
                self.halted = true;
                self.pc = next_pc;
            }
        }

        self.regs[0] = 0;
        Ok(())
    }

    pub fn run(&mut self, max_cycles: usize) -> Result<(), ZkvmError> {
        for _ in 0..max_cycles {
            if self.halted {
                return Ok(());
            }

            self.step()?;
        }

        if self.halted {
            Ok(())
        } else {
            Err(ZkvmError::ExecutionLimitExceeded { limit: max_cycles })
        }
    }

    fn read_reg(&self, reg: u8) -> Result<u32, ZkvmError> {
        if reg < 32 {
            Ok(self.regs[reg as usize])
        } else {
            Err(ZkvmError::InvalidRegister { reg })
        }
    }

    fn write_reg(&mut self, reg: u8, value: u32) -> Result<(), ZkvmError> {
        if reg >= 32 {
            return Err(ZkvmError::InvalidRegister { reg });
        }

        if reg != 0 {
            self.regs[reg as usize] = value;
        }

        Ok(())
    }

    fn ensure_alignment(&self, addr: u32, alignment: u32) -> Result<(), ZkvmError> {
        if addr % alignment == 0 {
            Ok(()))
        } else {
            Err(ZkvmError::MisalignedAccess { addr, alignment })
        }
    }

    fn checked_range(&self, addr: u32, size: usize) -> Result<usize, ZkvmError> {
        let start = addr as usize;
        let end = start
            .checked_add(size)
            .ok_or(ZkvmError::MemoryOutOfBounds { addr, size })?;

        if end > self.memory.len() {
            Err(ZkvmError::MemoryOutOfBounds { addr, size })
        } else {
            Ok(start)
        }
    }

    fn load_u8(&self, addr: u32) -> Result<u8, ZkvmError> {
        let index = self.checked_range(addr, 1)?;
        Ok(self.memory[index])
    }

    fn load_u16(&self, addr: u32) -> Result<u16, ZkvmError> {
        self.ensure_alignment(addr, 2)?;
        let index = self.checked_range(addr, 2)?;
        Ok(u16::from_le_bytes([self.memory[index], self.memory[index + 1]]))
    }

    fn load_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        self.ensure_alignment(addr, 4)?;
        let index = self.checked_range(addr, 4)?;
        Ok(u32::from_le_bytes([
            self.memory[index],
            self.memory[index + 1],
            self.memory[index + 2],
            self.memory[index + 3],
        ]))
    }

    fn store_u8(&mut self, addr: u32, value: u8) -> Result<(), ZkvmError> {
        let index = self.checked_range(addr, 1)?;
        self.memory[index] = value;
        Ok(())
    }

    fn store_u16(&mut self, addr: u32, value: u16) -> Result<(), ZkvmError> {
        self.ensure_alignment(addr, 2)?;
        let index = self.checked_range(addr, 2)?;
        let bytes = value.to_le_bytes();
        self.memory[index..index + 2].copy_from_slice(fbytes);
        Ok(())
    }

    fn store_u32(&mut self, addr: u32, value: u32) -> Result<(), ZkvmError> {
        self.ensure_alignmenxattr, 4)?;
        let index = self.checked_range(addr, 4)?;
        let bytes = value.to_le_bytes();
        self.memory[index..index + 4].copy_from_slice(fbytes);
        Ok(())
    }
}
