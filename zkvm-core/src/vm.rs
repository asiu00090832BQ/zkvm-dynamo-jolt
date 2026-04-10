use crate::decoder::{
    AluOpImmKind, AluOpKind, BranchKind, DecodeError, DecodedInstruction, LoadKind, MulDivKind,
    StoreKind,
};
use crate::elf_loader::{ElfError, ElfImage};

#[derive(Debug, Clone)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub enable_m_extension: bool,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        ZkvmConfig {
            memory_size: 16 * 1024 * 1024,
            enable_m_extension: true,
        }
    }
}

#[derive(Debug)]
pub enum VmError {
    Decode(DecodeError),
    Elf(ElfError),
    MemoryOutOfBounds,
    MisalignedAccess,
    MisalignedPc,
    IllegalInstruction,
    StepLimitReached,
    Halted,
}

impl From<DecodeError> for VmError {
    fn from(e: DecodeError) -> Self {
        VmError::Decode(e)
    }
}

impl From<ElfError> for VmError {
    fn from(e: ElfError) -> Self {
        VmError::Elf(e)
    }
}

#[derive(Debug)]
pub struct Zkvm {
    pub config: ZkvmConfig,
    pub pc: u32,
    pub regs: [u32; 32],
    pub memory: Vec<u8>,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig, image: ElfImage) -> Result<Self, VmError> {
        let mut memory = vec![0u8; config.memory_size];

        for (seg, data) in &image.segments {
            let start = seg.vaddr as usize;
            let end = start.checked_add(data.len()).ok_or(VmError::MemoryOutOfBounds)?;
            if end > memory.len() {
                return Err(VmError::MemoryOutOfBounds);
            }
            memory[start..end].copy_from_slice(data);

            let memsz = seg.mem_size as usize;
            if memsz > data.len() {
                let zero_start = end;
                let zero_end = start
                    .checked_add(memsz)
                    .ok_or(VmError::MemoryOutOfBounds)?;
                if zero_end > memory.len() {
                    return Err(VmError::MemoryOutOfBounds);
                }
                for b in &mut memory[zero_start..zero_end] {
                    *b = 0;
                }
            }
        }

        Ok(Zkvm {
            config,
            pc: image.entry,
            regs: [0u32; 32],
            memory,
        })
    }

    pub fn reset(&mut self, image: ElfImage) -> Result<(), VmError> {
        if self.memory.len() != self.config.memory_size {
            self.memory.resize(self.config.memory_size, 0);
        } else {
            for b in &mut self.memory {
                *b = 0;
            }
        }

        for (seg, data) in &image.segments {
            let start = seg.vaddr as usize;
            let end = start.checked_add(data.len()).ok_or(VmError::MemoryOutOfBounds)?;
            if end > self.memory.len() {
                return Err(VmError::MemoryOutOfBounds);
            }
            self.memory[start..end].copy_from_slice(data);

            let memsz = seg.mem_size as usize;
            if memsz > data.len() {
                let zero_start = end;
                let zero_end = start
                    .checked_add(memsz)
                    .ok_or(VmError::MemoryOutOfBounds)?;
                if zero_end > self.memory.len() {
                    return Err(VmError::MemoryOutOfBounds);
                }
                for b in &mut self.memory[zero_start..zero_end] {
                    *b = 0;
                }
            }
        }

        self.pc = image.entry;
        self.regs = [0u32; 32];
        Ok(())
    }

    pub fn run(&mut self, max_cycles: u64) -> Result<(), VmError> {
        let mut cycles = 0u64;
        loop {
            if cycles >= max_cycles {
                return Err(VmError::StepLimitReached);
            }
            match self.step() {
                Ok(()) => {}
                Err(VmError::Halted) => return Ok(()),
                Err(e) => return Err(e),
            }
            cycles += 1;
        }
    }

    pub fn step(&mut self) -> Result<(), VmError> {
        if self.pc % 4 != 0 {
            return Err(VmError::MisalignedPc);
        }

        let pc = self.pc as usize;
        let word = self.load_u32(pc)?;

        let instr = crate::decoder::decode(word, &self.config)?;

        self.execute(instr)
    }

    fn execute(&mut self, instr: DecodedInstruction) -> Result<(), VmError> {
        match instr {
            DecodedInstruction::Lui { rd, imm } => {
                self.write_reg(rd, imm as u32);
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::Auipc { rd, imm } => {
                let val = (self.pc as i32).wrapping_add(imm) as u32;
                self.write_reg(rd, val);
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::Jal { rd, imm } => {
                let next = self.pc.wrapping_add(4);
                let target = (self.pc as i32).wrapping_add(imm) as u32;
                self.write_reg(rd, next);
                self.pc = target;
            }
            DecodedInstruction::Jalr { rd, rs1, imm } => {
                let base = self.read_reg(rs1);
                let next = self.pc.wrapping_add(4);
                let target = ((base as i32).wrapping_add(imm) as u32) & !1u32;
                self.write_reg(rd, next);
                self.pc = target;
            }
            DecodedInstruction::Branch {
                kind,
                rs1,
                rs2,
                imm,
            } => {
                let v1 = self.read_reg(rs1);
                let v2 = self.read_reg(rs2);
                let take = match kind {
                    BranchKind::Beq => v1 == v2,
                    BranchKind::Bne => v1 != v2,
                    BranchKind::Blt => (v1 as i32) < (v2 as i32),
                    BranchKind::Bge => (v1 as i32) >= (v2 as i32),
                    BranchKind::Bltu => v1 < v2,
                    BranchKind::Bgeu => v1 >= v2,
                };
                if take {
                    self.pc = (self.pc as i32).wrapping_add(imm) as u32;
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
            }
            DecodedInstruction::Load {
                kind,
                rd,
                rs1,
                imm,
            } => {
                let addr = ((self.read_reg(rs1) as i32).wrapping_add(imm) as u32) as usize;
                let val = match kind {
                    LoadKind::Byte => {
                        let b = self.load_u8(addr)? as i8 as i32 as u32;
                        b
                    }
                    LoadKind::Half => {
                        if addr % 2 != 0 {
                            return Err(VmError::MisalignedAccess);
                        }
                        let h = self.load_u16(addr)? as i16 as i32 as u32;
                        h
                    }
                    LoadKind::Word => {
                        if addr % 4 != 0 {
                            return Err(VmError::MisalignedAccess);
                        }
                        self.load_u32(addr)?
                    }
                    LoadKind::ByteUnsigned => self.load_u8(addr)? as u32,
                    LoadKind::HalfUnsigned => {
                        if addr % 2 != 0 {
                            return Err(VmError::MisalignedAccess);
                        }
                        self.load_u16(addr)? as u32
                    }
                };
                self.write_reg(rd, val);
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::Store {
                kind,
                rs1,
                rs2,
                imm,
            } => {
                let addr = ((self.read_reg(rs1) as i32).wrapping_add(imm) as u32) as usize;
                let val = self.read_reg(rs2);
                match kind {
                    StoreKind::Byte => {
                        self.store_u8(addr, val as u8)?;
                    }
                    StoreKind::Half => {
                        if addr % 2 != 0 {
                            return Err(VmError::MisalignedAccess);
                        }
                        self.store_u16(addr, val as u16)?;
                    }
                    StoreKind::Word => {
                        if addr % 4 != 0 {
                            return Err(VmError::MisalignedAccess);
                        }
                        self.store_u32(addr, val)?;
                    }
                }
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::OpImm {
                kind,
                rd,
                rs1,
                imm,
            } => {
                let v = self.read_reg(rs1);
                let result = match kind {
                    AluOpImmKind::Addi => (v as i32).wrapping_add(imm) as u32,
                    AluOpImmKind::Slti => {
                        if (v as i32) < imm {
                            1
                        } else {
                            0
                        }
                    }
                    AluOpImmKind::Sltiu => {
                        if v < imm as u32 {
                            1
                        } else {
                            0
                        }
                    }
                    AluOpImmKind::Xori => v ^ (imm as u32),
                    AluOpImmKind::Ori => v | (imm as u32),
                    AluOpImmKind::Andi => v & (imm as u32),
                    AluOpImmKind::Slli => {
                        let shamt = (imm as u32) & 0x1f;
                        v.wrapping_shl(shamt)
                    }
                    AluOpImmKind::Srli => {
                        let shamt = (imm as u32) & 0x1f;
                        v.wrapping_shr(shamt)
                    }
                    AluOpImmKind::Srai => {
                        let shamt = (imm as u32) & 0x1f;
                        ((v as i32).wrapping_shr(shamt)) as u32
                    }
                };
                self.write_reg(rd, result);
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::Op {
                kind,
                rd,
                rs1,
                rs2,
            } => {
                let v1 = self.read_reg(rs1);
                let v2 = self.read_reg(rs2);
                let result = match kind {
                    AluOpKind::Add => v1.wrapping_add(v2),
                    AluOpKind::Sub => v1.wrapping_sub(v2),
                    AluOpKind::Sll => {
                        let shamt = v2 & 0x1f;
                        v1.wrapping_shl(shamt)
                    }
                    AluOpKind::Slt => {
                        if (v1 as i32) < (v2 as i32) {
                            1
                        } else {
                            0
                        }
                    }
                    AluOpKind::Sltu => {
                        if v1 < v2 {
                            1
                        } else {
                            0
                        }
                    }
                    AluOpKind::Xor => v1 ^ v2,
                    AluOpKind::Srl => {
                        let shamt = v2 & 0x1f;
                        v1.wrapping_shr(shamt)
                    }
                    AluOpKind::Sra => {
                        let shamt = v2 & 0x1f;
                        ((v1 as i32).wrapping_shr(shamt)) as u32
                    }
                    AluOpKind::Or => v1 | v2,
                    AluOpKind::And => v1 & v2,
                };
                self.write_reg(rd, result);
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::MulDiv {
                kind,
                rd,
                rs1,
                rs2,
            } => {
                let v1 = self.read_reg(rs1);
                let v2 = self.read_reg(rs2);
                let result = match kind {
                    MulDivKind::Mul => {
                        let res = (v1 as u64).wrapping_mul(v2 as u64);
                        res as u32
                    }
                    MulDivKind::Mulh => {
                        let a = v1 as i32 as i64;
                        let b = v2 as i32 as i64;
                        let res = a.wrapping_mul(b) as u128;
                        (res >> 32) as u32
                    }
                    MulDivKind::Mulhsu => {
                        let a = v1 as i32 as i64;
                        let b = v2 as u64 as i64;
                        let res = a.wrapping_mul(b) as u128;
                        (res >> 32) as u32
                    }
                    MulDivKind::Mulhu => {
                        let a = v1 as u64;
                        let b = v2 as u64;
                        let res = a.wrapping_mul(b) as u128;
                        (res >> 32) as u32
                    }
                    MulDivKind::Div => {
                        if v2 == 0 {
                            u32::MAX
                        } else if v1 == 0x8000_0000 && v2 == 0xffff_ffff {
                            0x8000_0000
                        } else {
                            (v1 as i32).wrapping_div(v2 as i32) as u32
                        }
                    }
                    MulDivKind::Divu => {
                        if v2 == 0 {
                            u32::MAX
                        } else {
                            v1.wrapping_div(v2)
                        }
                    }
                    MulDivKind::Rem => {
                        if v2 == 0 {
                            v1
                        } else if v1 == 0x8000_0000 && v2 == 0xffff_ffff {
                            0
                        } else {
                            (v1 as i32).wrapping_rem(v2 as i32) as u32
                        }
                    }
                    MulDivKind::Remu => {
                        if v2 == 0 {
                            v1
                        } else {
                            v1.wrapping_rem(v2)
                        }
                    }
                };
                self.write_reg(rd, result);
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::Fence => {
                self.pc = self.pc.wrapping_add(4);
            }
            DecodedInstruction::Ecall | DecodedInstruction::Ebreak => {
                return Err(VmError::Halted);
            }
        }
        Ok(())
    }

    fn read_reg(&self, index: u8) -> u32 {
        if index == 0 {
            0
        } else {
            self.regs[index as usize]
        }
    }

    fn write_reg(&mut self, index: u8, value: u32, {
        if index != 0 {
            self.regs[index as usize] = value;
        }
    }

    fn load_u8(&self, addr: usize) -> Result<u8, VmError> {
        self.memory.get(addr).copied().ok_or(VmError::MemoryOutOfBounds)
    }

    fn load_u16(&self, addr: usize) -> Result<u16, VmError> {
        if addr + 2 > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds);
        }
        Ok(u16::from_le_bytes([
            self.memory[addr],
            self.memory[addr + 1],
        ]))
    }

    fn load_u32(&self, addr: usize) -> Result<u32, VmError> {
        if addr + 4 > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds);
        }
        Ok(u32::from_le_bytes([
            self.memory[addr],
            self.memory[addr + 1],
            self.memory[addr + 2],
            self.memory[addr + 3],
        ]))
    }

    fn store_u8(&mut self, addr: usize, value: u8) -> Result<(), VmError> {
        if let Some(slot) = self.memory.get_mut(addr) {
            *slot = value;
            Ok(())
        } else {
            Err(VmError::MemoryOutOfBounds)
        }
    }

    fn store_u16(&mut self, addr: usize, value: u16) -> Result<(), VmError> {
        if addr + 2 > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds);
        }
        let bytes = value.to_le_bytes();
        self.memory[addr] = bytes[0];
        self.memory[addr + 1] = bytes[1];
        Ok(())
    }

    fn store_u32(&mut self, addr: usize, value: u32) -> Result<(), VmError> {
        if addr + 4 > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds);
        }
        let bytes = value.to_le_bytes();
        self.memory[addr] = bytes[0];
        self.memory[addr + 1] = bytes[1];
        self.memory[addr + 2] = bytes[2];
        self.memory[addr + 3] = bytes[3];
        Ok(())
    }
}
