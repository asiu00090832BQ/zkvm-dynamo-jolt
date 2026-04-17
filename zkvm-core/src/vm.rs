use crate::decoder::{
    decode, BranchKind, DecodedInstruction, Instruction, LoadKind, MKind, OpImmKind, OpKind,
    StoreKind, ZkvmError,
};

#[derive(Debug, Clone)]
pub struct Zkvm {
    pc: u32,
    regs: [u32; 32],
    memory: Vec<u8>,
}

impl Zkvm {
    pub fn new(memory_size: usize) -> Self {
        Self {
            pc: 0,
            regs: [0; 32],
            memory: vec![0; memory_size],
        }
    }

    pub fn with_memory(memory: Vec<u8>) -> Self {
        Self {
            pc: 0,
            regs: [0; 32],
            memory,
        }
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u32) -> Result<(), ZkvmError> {
        self.ensure_pc_alignment(pc)?;
        self.pc = pc;
        Ok(())
    }

    pub fn registers(&self) -> &[u32; 32] {
        &self.regs
    }

    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    pub fn decode_current(&self) -> Result<DecodedInstruction, ZkvmError> {
        let word = self.fetch_u32(self.pc)?;
        decode(word)
    }

    pub fn load_program(&mut self, start_addr: u32, program: &[u8]) -> Result<(), ZkvmError> {
        self.ensure_pc_alignment(start_addr)?;
        let start = start_addr as usize;
        let end = start
            .checked_add(program.len())
            .ok_or(ZkvmError::MemoryOutOfBounds {
                addr: start_addr,
                size: program.len(),
            })?;

        if end > self.memory.len() {
            return Err(ZkvmError::MemoryOutOfBounds {
                addr: start_addr,
                size: program.len(),
            });
        }

        self.memory[start..end].copy_from_slice(program);
        self.pc = start_addr;
        Ok(())
    }

    pub fn run(&mut self, max_steps: usize) -> Result<(), ZkvmError> {
        for _ in 0..max_steps {
            self.step()?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), ZkvmError> {
        let pc = self.pc;
        let word = self.fetch_u32(pc)?;
        let decoded = decode(word)?;
        self.execute(pc, decoded.instruction)?;
        self.regs[0] = 0;
        Ok(())
    }

    fn execute(&mut self, pc: u32, instruction: Instruction) -> Result<(), ZkvmError> {
        match instruction {
            Instruction::M { kind, rd, rs1, rs2 } => {
                self.execute_m_extension(pc, kind, rd, rs1, rs2)
            }
            _ => self.execute_base_instruction(pc, instruction),
        }
    }

    fn execute_base_instruction(
        &mut self,
        pc: u32,
        instruction: Instruction,
    ) -> Result<(), ZkvmError> {
        match instruction {
            Instruction::Lui { rd, imm } => {
                self.write_reg(rd, imm);
                self.advance_pc(pc);
            }
            Instruction::Auipc { rd, imm } => {
                self.write_reg(rd, pc.wrapping_add(imm));
                self.advance_pc(pc);
            }
            Instruction::Jal { rd, imm } => {
                let next_pc = pc.wrapping_add(4);
                let target = Self::add_signed_u32(pc, imm);
                self.ensure_pc_alignment(target)?;
                self.write_reg(rd, next_pc);
                self.pc = target;
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let next_pc = pc.wrapping_add(4);
                let base = self.read_reg(rs1);
                let target = Self::add_signed_u32(base, imm) & !1;
                self.ensure_pc_alignment(target)?;
                self.write_reg(rd, next_pc);
                self.pc = target;
            }
            Instruction::Branch {
                kind,
                rs1,
                rs2,
                imm,
            } => {
                let lhs = self.read_reg(rs1);
                let rhs = self.read_reg(rs2);
                let taken = match kind {
                    BranchKind::Beq => lhs == rhs,
                    BranchKind::Bne => lhs != rhs,
                    BranchKind::Blt => (lhs as i32) < (rhs as i32),
                    BranchKind::Bge => (lhs as i32) >= (rhs as i32),
                    BranchKind::Bltu => lhs < rhs,
                    BranchKind::Bgeu => lhs >= rhs,
                };

                if taken {
                    let target = Self::add_signed_u32(pc, imm);
                    self.ensure_pc_alignment(target)?;
                    self.pc = target;
                } else {
                    self.advance_pc(pc);
                }
            }
            Instruction::Load { kind, rd, rs1, imm } => {
                let addr = Self::add_signed_u32(self.read_reg(rs1), imm);
                let value = match kind {
                    LoadKind::Lb => (self.load_u8(addr)? as i8 as i32) as u32,
                    LoadKind::Lh => (self.load_u16(addr)? as i16 as i32) as u32,
                    LoadKind::Lw => self.load_u32(addr)?,
                    LoadKind::Lbu => self.load_u8(addr)? as u32,
                    LoadKind::Lhu => self.load_u16(addr)? as u32,
                };
                self.write_reg(rd, value);
                self.advance_pc(pc);
            }
            Instruction::Store {
                kind,
                rs1,
                rs2,
                imm,
            } => {
                let addr = Self::add_signed_u32(self.read_reg(rs1), imm);
                let value = self.read_reg(rs2);
                match kind {
                    StoreKind::Sb => self.store_u8(addr, value as u8)?,
                    StoreKind::Sh => self.store_u16(addr, value as u16)?,
                    StoreKind::Sw => self.store_u32(addr, value)?,
                }
                self.advance_pc(pc);
            }
            Instruction::OpImm { kind, rd, rs1, imm } => {
                let lhs = self.read_reg(rs1);
                let result = match kind {
                    OpImmKind::Addi => lhs.wrapping_add(imm as u32),
                    OpImmKind::Slti => ((lhs as i32) < imm) as u32,
                    OpImmKind::Sltiu => (lhs < imm as u32) as u32,
                    OpImmKind::Xori => lhs ^ imm as u32,
                    OpImmKind::Ori => lhs | imm as u32,
                    OpImmKind::Andi => lhs & imm as u32,
                    OpImmKind::Slli => lhs << (imm as u32 & 0x1f),
                    OpImmKind::Srli => lhs >> (imm as u32 & 0x1f),
                    OpImmKind::Srai => ((lhs as i32) >> (imm as u32 & 0x1f)) as u32,
                };
                self.write_reg(rd, result);
                self.advance_pc(pc);
            }
            Instruction::Op { kind, rd, rs1, rs2 } => {
                let lhs = self.read_reg(rs1);
                let rhs = self.read_reg(rs2);
                let result = match kind {
                    OpKind::Add => lhs.wrapping_add(rhs),
                    OpKind::Sub => lhs.wrapping_sub(rhs),
                    OpKind::Sll => lhs << (rhs & 0x1f),
                    OpKind::Slt => ((lhs as i32) < (rhs as i32)) as u32,
                    OpKind::Sltu => (lhs < rhs) as u32,
                    OpKind::Xor => lhs ^ rhs,
                    OpKind::Srl => lhs >> (rhs & 0x1f),
                    OpKind::Sra => ((lhs as i32) >> (rhs & 0x1f)) as u32,
                    OpKind::Or => lhs | rhs,
                    OpKind::And => lhs & rhs,
                };
                self.write_reg(rd, result);
                self.advance_pc(pc);
            }
            Instruction::Fence { .. } => {
                self.advance_pc(pc);
            }
            Instruction::Ecall => return Err(ZkvmError::Ecall),
            Instruction::Ebreak => return Err(ZkvmError::Ebreak),
            Instruction::M { .. } => unreachable!("M extension routed separately"),
        }

        Ok(())
    }

    fn execute_m_extension(
        &mut self,
        pc: u32,
        kind: MKind,
        rd: u8,
        rs1: u8,
        rs2: u8,
    ) -> Result<(), ZkvmError> {
        let lhs = self.read_reg(rs1);
        let rhs = self.read_reg(rs2);

        let result = match kind {
            MKind::Mul => Self::mul_low_u32_limb(lhs, rhs),
            MKind::Mulh => {
                let product = (lhs as i32 as i64) * (rhs as i32 as i64);
                (product >> 32) as u32
            }
            MKind::Mulhsu => {
                let product = (lhs as i32 as i64) * (rhs as u64 as i64);
                (product >> 32) as u32
            }
            MKind::Mulhu => (Self::mul_u64_with_16bit_limbs(lhs, rhs) >> 32) as u32,
            MKind::Div => {
                let dividend = lhs as i32;
                let divisor = rhs as i32;
                if divisor == 0 {
                    u32::MAX
                } else if dividend == i32::MIN && divisor == -1 {
                    dividend as u32
                } else {
                    (dividend / divisor) as u32
                }
            }
            MKind::Divu => {
                if rhs == 0 {
                    u32::MAX
                } else {
                    lhs / rhs
                }
            }
            MKind::Rem => {
                let dividend = lhs as i32;
                let divisor = rhs as i32;
                if divisor == 0 {
                    dividend as u32
                } else if dividend == i32::MIN && divisor == -1 {
                    0
                } else {
                    (dividend % divisor) as u32
                }
            }
            MKind::Remu => {
                if rhs == 0 {
                    lhs
                } else {
                    lhs % rhs
                }
            }
        };

        self.write_reg(rd, result);
        self.advance_pc(pc);
        Ok(())
    }

    fn read_reg(&self, reg: u8) -> u32 {
        self.regs[reg as usize]
    }

    fn write_reg(&mut self, reg: u8, value: u32) {
        if reg != 0 {
            self.regs[reg as usize] = value;
        }
    }

    fn advance_pc(&mut self, pc: u32) {
        self.pc = pc.wrapping_add(4);
    }

    fn ensure_pc_alignment(&self, pc: u32) -> Result<(), ZkvmError> {
        if pc & 0x3 != 0 {
            Err(ZkvmError::InstructionAddressMisaligned(pc))
        } else {
            Ok(())
        }
    }

    fn ensure_alignment(addr: u32, alignment: u32) -> Result<(), ZkvmError> {
        if addr % alignment != 0 {
            Err(ZkvmError::MisalignedAccess { addr, alignment })
        } else {
            Ok(())
        }
    }

    fn checked_index(&self, addr: u32, size: usize) -> Result<usize, ZkvmError> {
        let start = addr as usize;
        let end = start.checked_add(size).ok_or(ZkvmError::MemoryOutOfBounds {
            addr,
            size,
        })?;

        if end > self.memory.len() {
            Err(ZkvmError::MemoryOutOfBounds { addr, size })
        } else {
            Ok(start)
        }
    }

    fn fetch_u32(&self, pc: u32) -> Result<u32, ZkvmError> {
        self.ensure_pc_alignment(pc)?;
        let start = pc as usize;
        let end = start.checked_add(4).ok_or(ZkvmError::PcOutOfBounds(pc))?;
        if end > self.memory.len() {
            return Err(ZkvmError::PcOutOfBounds(pc));
        }

        Ok(
            (self.memory[start] as u32)
                | ((self.memory[start + 1] as u32) << 8)
                | ((self.memory[start + 2] as u32) << 16)
                | ((self.memory[start + 3] as u32) << 24),
        )
    }

    fn load_u8(&self, addr: u32) -> Result<u8, ZkvmError> {
        let index = self.checked_index(addr, 1)?;
        Ok(self.memory[index])
    }

    fn load_u16(&self, addr: u32) -> Result<u16, ZkvmError> {
        Self::ensure_alignment(addr, 2)?;
        let index = self.checked_index(addr, 2)?;
        Ok((self.memory[index] as u16) | ((self.memory[index + 1] as u16) << 8))
    }

    fn load_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        Self::ensure_alignment(addr, 4)?;
        let index = self.checked_index(addr, 4)?;
        Ok(
            (self.memory[index] as u32)
                | ((self.memory[index + 1] as u32) << 8)
                | ((self.memory[index + 2] as u32) << 16)
                | ((self.memory[index + 3] as u32) << 24),
        )
    }

    fn store_u8(&mut self, addr: u32, value: u8) -> Result<(), ZkvmError> {
        let index = self.checked_index(addr, 1)?;
        self.memory[index] = value;
        Ok(())
    }

    fn store_u16(&mut self, addr: u32, value: u16) -> Result<(), ZkvmError> {
        Self::ensure_alignment(addr, 2)?;
        let index = self.checked_index(addr, 2)?;
        self.memory[index] = (value & 0x00ff) as u8;
        self.memory[index + 1] = (value >> 8) as u8;
        Ok(())
    }

    fn store_u32(&mut self, addr: u32, value: u32) -> Result<(), ZkvmError> {
        Self::ensure_alignment(addr, 4)?;
        let index = self.checked_index(addr, 4)?;
        self.memory[index] = (value & 0x0000_00ff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0x0000_00ff) as u8;
        self.memory[index + 2] = ((value >> 16) & 0x0000_00ff) as u8;
        self.memory[index + 3] = ((value >> 24) & 0x0000_00ff) as u8;
        Ok(())
    }

    fn add_signed_u32(base: u32, offset: i32) -> u32 {
        base.wrapping_add(offset as u32)
    }

    /// Lemma 6.1.1 compliance: MUL is computed through 16-bit limbs
    /// a0, a1, b0, and b1, and then reduced to the low 32 bits.
    fn mul_low_u32_limb(a: u32, b: u32) -> u32 {
        Self::mul_u64_with_16bit_limbs(a, b) as u32
    }

    fn mul_u64_with_16bit_limbs(a: u32, b: u32) -> u64 {
        let a0 = (a & 0x0000_ffff) as u64;
        let a1 = (a >> 16) as u64;
        let b0 = (b & 0x0000_ffff) as u64;
        let b1 = (b >> 16) as u64;

        let p0 = a0 * b0;
        let p1 = a0 * b1;
        let p2 = a1 * b0;
        let p3 = a1 * b1;

        p0.wrapping_add((p1.wrapping_add(p2)) << 16)
            .wrapping_add(p3 << 32)
    }
}
