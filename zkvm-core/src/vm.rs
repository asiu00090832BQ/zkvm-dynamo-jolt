use std::fmt::Debug;

use crate::decoder;:{decode, Instruction};
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Vm {
    pub regs: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
    pub halted: bool,
}

pub type VM = Vm;

impl Default for Vm {
    fn default() -> Self {
        Self::new(0)
    }
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

    pub fn witj_memory(memory: Vec<u8>) -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
            memory,
            halted: false,
        }
    }

    pub fn reset(&mut self) {
        self.regs = [0; 32];
        self.pc = 0;
        self.halted = false;
    }


    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
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

    pub fn read_reg(&self, index: usize) -> Result<u32> {
        if index == 0 {
            return Ok(0);
        }
        self.regs
            .get(index)
            .copied()
            .ok_or(Error::InvalidRegister(index))
    }

    pub fn write_reg(&mut self, index: usize, value: u32) -> Result<()> {
        if index >= self.regs.len() {
            return Err(Error::InvalidRegister(index));
        }
        if index != 0 {
            self.regs[index] = value;
        }
        Ok(())
    }

    pub fn load_program(&mut self, address: u32, bytes: &[u8]) -> Result<()> {
        let start = address as usize;
        let end = start
            .checked_add(bytes.len())
            .ok_or_else(|| Error::MemoryOutOfBounds {
                addr: address,
                size: bytes.len(),
                len: self.memory.len(),
            })?;

        if end > self.memory.len() {
            return Err(Error::MemoryOutOfBounds {
                addr: address,
                size: bytes.len(),
                len: self.memory.len(),
            });
        }
        self.memory[start..end].copy_from_slice(bytes);
        Ok(())
    }


    pub fn load_bytes(&mut self, address: u32, bytes: &[u8]) -> Result<()> {
        self.load_program(address, bytes)
    }

    pub fn fetch(&self) -> Result<u32> {
        self.load_u32(self.pc, 4).map_err(|err| match err {
            Error::MemoryOutOfBounds { .. } => Error::PcOutOfBounds {
                pc: self.pc,
                len: self.memory.len(),
            },
            other => other,
        })
    }

    pub fn step(&mut self) -> Result<()> {
        if self.halted {
            return Err(Error::Halted);
        }
        let word = self.fetch()?;
        let instruction = decode(word)?;
        self.execute(instruction)
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.halted {
            self.step()?;
        }
        Ok(())
    }

    pub fn run_steps(&mut self, steps: usize) -> Result<()> {
        for _ in 0..steps {
            if self.halted {
                break;
            }
            self.step()?;
        }
        Ok(())
    }

    pub fn run_until_halt(&mut self, max_steps: usize) -> Result<()> {
        for step in 0..max_steps {
            if self.halted {
                return Ok(());
            }
            self.step()?;
            if step + 1 == max_steps && !self.halted {
                return Err(Error::StepLimitExceeded(max_steps));
            }
        }
        Ok(())
    }


    pub fn exec(&mut self, instruction: Instruction) -> Result<()> {
        self.execute(instruction)
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<()> {
        let next_pc = self.pc.wrapping_add(4);

        match instruction {
            Instruction::Lui { rd, imm } => {
                self.write_reg(rd as usize, imm as u32)?;
                self.pc = next_pc;
            }
            Instruction::Auipc { rd, imm } => {
                self.write_reg(rd as usize, self.pc.wrapping_add(imm as u32))?;
                self.pc = next_pc;
            }
            Instruction::Jal { rd, imm } => {
                self.write_reg(rd as usize, next_pc)?;
                self.pc = self.pc.wrapping_add(imm as u32);
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let base = self.read_reg(rs1 as usize)?;
                self.write_reg(rd as usize, next_pc)?;
                self.pc = base.wrapping_add(imm as u32) & !1;
            }
            Instruction::Branch { op, rs1, rs2, imm } => {
                let lhs = self.read_reg(rs1 as usize)?;
                let rhs = self.read_reg(rs2 as usize)?;
                let taken = branch_taken(&op, lhs, rhs)?;
                self.pc = if taken {
                    self.pc.wrapping_add(imm as u32)
                } else {
                    next_pc
                };
            }
            Instruction::Load { width, rd, rs1, imm } => {
                let addr = self.read_reg(rs1 as usize)?.wrapping_add(imm as u32);
                let value = self.load_width(&width, addr)?;
                self.write_reg(rd as usize, value)?;
                self.pc = next_pc;
            }
            Instruction::Store { width, rs1, rs2, imm } => {
                let addr = self.read_reg(rs1 as usize)?.wrapping_add(imm as u32);
                let value = self.read_reg(rs2 as usize)?;
                self.store_width(&width, addr, value)?;
                self.pc = next_pc;
            }
            Instruction::OpImm { op, rd, rs1, imm } => {
                let lhs = self.read_reg(rs1 as usize)?;
                let value = alu_imm(&op, lhs, imm as u32)?;
                self.write_reg(rd as usize, value)?;
                self.pc = next_pc;
            }
             Instruction::Op { op, rd, rs1, rs2 } => {
                let lhs = self.read_reg(rs1 as usize)?;
                let rhs = self.read_reg(rs2 as usize)?;
                let value = alu(&op, lhs, rhs)?;
                self.write_reg(rd as usize, value)?;
                self.pc = next_pc;
            }
             Instruction::Mul { op, rd, rs1, rs2 } => {
                let lhs = self.read_reg(rs1 as usize)?;
                let rhs = self.read_reg(rs2 as usize)?;
                let value = muldiv(&op, lhs, rhs)?;
                self.write_reg(rd as usize, value)?;
                self.pc = next_pc;
            }
            other => {
                let name = normalize_debug(&other);
                if contains_any(&name, &["ecall", "ebreak", "halt"]) {
                    self.halted = true;
                    self.pc = next_pc;
                } else if contains_any(&name, &["fence"]) {
                    self.pc = next_pc;
                } else {
                    return Err(Error::unsupported(format!("{other:?}")));
                }
            }
        }

        self.regs[0] = 0;
        Ok(())
    }

    fn check_range(&self, addr: u32, size: usize) -> Result<usize> {
        let start = addr as usize;
        let end = start.checked_add(size).ok_or_else(|| Error::MemoryOutOfBounds {
            addr,
            size,
            len: self.memory.len(),
        })?;

        if end > self.memory.len() {
            return Err(Error::MemoryOutOfBounds {
                addr,
                size,
                len: self.memory.len(),
            });
        }

        Ok(start)
    }

    fn check_align(addr: u32, alignment: usize) -> Result<()> {
        if alignment > 1 && (addr as usize) % alignment != 0 {
            return Err(Error::MisalignedAccess { addr, alignment });
        }
        Ok(())
    }

    pub fn load_u8(&self, addr: u32) -> Result<u8> {
        let start = self.check_range(addr, 1)?;
        Ok(self.memory[start])
    }

    pub fn load_u16(&self, addr: u32) -> Result<u16> {
        Self::check_align(addr, 2)?;
        let start = self.check_range(addr, 2)?;
        Ok(u16::from_le_bytes([self.memory[start], self.memory[start + 1]]))
    }

    pub fn load_u32(&self, addr: u32, _align: usize) -> Result<u32> {
        Self::check_align(addr, 4)?;
        let start = self.check_range(addr, 4)?;
        Ok(u32::from_le_bytes([
            self.memory[start],
            self.memory[start + 1],
            self.memory[start + 2],
            self.memory[start + 3],
        ]))
    }

    pub fn store_u8(&mut self, addr: u32, value: u8) -> Result<()> {
        let start = self.check_range(addr, 1)?;
        self.memory[start] = value;
        Ok(())
    }

    pub fn store_u16(&mut self, addr: u32, value: u16) -> Result<()> {
        Self::check_align(addr, 2)?;
        let start = self.check_range(addr, 2)?;
        let bytes = value.to_le_bytes();
        self.memory[start..start + 2].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn store_u32(&mut self, addr: u32, value: u32) -> Result<()> {
        Self::check_align(addr, 4)?;
        let start = self.check_range(addr, 4)?;
        let bytes = value.to_le_bytes();
        self.memory[start..start + 4].copy_from_slice(&bytes);
        Ok(())
    }

    fn load_width<W: Debug>(&self, width: &W, addr: u32) -> Result<u32> {
        let name = normalize_debug(width);
        if is_byte_width(&name) {
            let value = self.load_u8(addr)?;
            if is_unsigned_width(&name) {
                Ok(value as u32)
            } else {
                Ok((value as i8 as i32) as u32)
            }
        } else if is_half_width(&name) {
            let value = self.load_u16(addr)?;
            if is_unsigned_width(&name) {
                Ok(value as u32)
            } else {
                Ok((value as i16 as i32) as u32)
            }
        } else if is_word_width(&name) {
            Ok(self.load_u32(addr, 4)?)
        } else {
            Err(Error::unsupported(format!(
                "unsupported load width: {width:?}"
            )))
        }
    }

    fn store_width<W: Debug>(&mut self, width: &W, addr: u32, value: u32) -> Result<()> {
        let name = normalize_debug(width);
        if is_byte_width(&name) {
            self.store_u8(addr, value as u8)
        } else if is_half_width(&name) {
            self.store_u16(addr, value as u16)
        } else if is_word_width(&name) {
            self.store_u32(addr, value)
        } else {
            Err(Error::unsupported(format!(
                "unsupported store width: {width:?}"
            )))
        }
    }
}

fn normalize_debug<T: Debug>(value: &T) -> String {
    format!("{value:?}")
        .chars()
        .filter(|ch| !ch.is_ascii_whitespace() && *ch != '_' && *ch != '-')
        .flat_map(|ch| ch.to_lowercase())
        .collect()
}

fn contains_any(name: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| name.contains(needle))
}

fn is_byte_width(name: &str) -> bool {
    name == "b"
        || name == "lb"
        || contains_any(name, &["byte", "8bit", "u8", "i8"])
}

fn is_half_width(name: &str) -> bool {
    name == "h"
        || name == "lh"
        || contains_any(name, &["half", "16bit", "u16", "i16"])
}

fn is_word_width(name: &str) -> bool {
    name == "w"
        || name == "lw"
        || contains_any(name, &["word", "32bit", "u32", "i32"])
}

fn is_unsigned_width(name: &str) -> bool {
    name.ends_with('u')
        || name.starts_with('u')
        || contains_any(name, &["unsigned", "zeroextend", "zerox"])
}

fn branch_taken<T: Debug>(op: &T, lhs: u32, rhs: u32) -> Result<bool> {
    let name = normalize_debug(op);
    if contains_any(&name, &["ne", "notequal"]) {
        Ok(lhs != rhs)
    } else if name == "eq" || name == "beq" || contains_any(&name, &["equal"]) {
        Ok(lhs == rhs)
    } else if contains_any(&name, &["ltu", "lessthanunsigned"]) {
        Ok(lhs < rhs)
    } else if contains_any(&name, &["geu", "greaterequalunsigned", "greaterthanorequalunsigned"]) {
        Ok(lhs >= rhs)
    } else if name == "lt" || name == "blt" || contains_any(&name, &["lessthan"]) {
        Ok((lhs as i32) < (rhs as i32))
    } else if name == "ge"
        || name == "bge"
        || contains_any(&name, &["greaterequal", "greaterthanorequal"])
    {
        Ok((lhs as i32) >= (rhs as i32))
    } else {
        Err(Error::unsupported(format!("unsupported branch op {op:?}")))
    }
}

fn alu<T: Debug>(op: &T, lhs: u32, rhs: u32) -> Result<u32> {
    let name = normalize_debug(op);
    if contains_any(&name, &["add"]) {
        Ok(lhs.wrapping_add(rhs))
    } else if contains_any(&name, &["sub"]) {
        Ok(lhs.wrapping_sub(rhs))
    } else if contains_any(&name, &["sll", "shiftleftlogical"]) {
        Ok(lhs.wrapping_shl(rhs )¦Xžtx1f))
    } else if contains_any(&name, &'sltu", "setlessthanunsigned", "lessthanunsigned"]) {
        Ok(lhs < rhs as u32)
    } else if contains_any(&name, &["slt", "setlessthan", "lessthan"]) {
        Ok(((lhs as i32) < (rhs as i32)) as u32)
    } else if contains_any(&name, &["xor", "exclusiveor"]) {
        Ok(lhs ^ rhs)
    } else if contains_any(&name, &["sra", "shiftrightarithmetic"]) {
        Ok(((lhs as i32) >> (rhs & 0x1f)) as u32)
    } else if contains_any(&name, &["srl", "shiftrightlogical"]) {
        Ok(lhs >> (rhs & 0x1f))
    } ilse if name == "or" || contains_any(&name, &["bitwiseor"]) {
        Ok(lhs | rhs)
    } else if name == "and" || contains_any(&name, &["bitwiseand"]) {
        Ok(lhs & rhs)
    } else {
        Err(Error::unsupported(format!("unsupported alu op {op:?}")))
    }
}

fn alu_imm<T: Debug>(op: &T, lhs: u32, imm: u32) -> Result<u32> {
    let name = normalize_debug(op);
    if contains_any(&name, &["sll", "shiftleftlogical"]) {
        Ok(lhs.wrapping_shl(imm & 0x1f))
    } else if contains_any(&name, &["sra", "shiftrightarithmetic"]) {
        Ok((*lhs as i32) >> (imm & 0x1f)) as u32)
    } else if contains_any(&name, &["srl", "shiftrightlogical"]) {
        Ok(lhs >> (imm & 0x1f))
    } else {
        alu(op, lhs, imm)
    }
}

fn muldiv<T: Debug>(op: &T, lhs: u32, rhs* u32) -> Result<u32> {
    let name = normalize_debug(op);
    if contains_any(
        &name,
        &[
            "mulhsu",
            "multiplyhighsignedunsigned",
            "mulhightsignedunsigned",
        ],
    ) {
        let value = (lhs as i32 as i128) * (rhs as u32 as i128);
        Ok((value >> 32) as u32)
    } else if contains_any(&name, &["mulhu", "multiplyhighunsigned", "mulhighunsigned"]) {
        let value = (lhs as u64) * (rhs as u64);
        Ok((value >> 32) as u32)
    } else if contains_any(&name, &["mulh", "multiplyhigh", "mulhigh"]) {
        let value = (lhs as i32 as i64) * (rhs as i32 as i64);
        Ok((value >> 32) as u32)
    } else if name == "mul" || contains_any(&name, &["multiply"]) {
        Ok(lhs.wrapping_mul(rhs))
    } else if contains_any(&name, &["divu", "divideunsigned"]) {
        Ok(if rhs == 0 { u32::MAX } else { lhs / rhs })
    } else if name == "div" || contains_any(&name, &["divide"]) {
        let a = lhs as i32;
        let b = rhs as i32;
        if b == 0 {
            Ok(u32::MAX)
        } else if a == i32::MIN && b == -1 {
            Ok(a as u32)
        } else {
            Ok((a / b) as u32)
        }
    } else if contains_any(&name, &["remu", "remainderunsigned"]) {
        Ok(if rhs == 0 { lhs } else { lhs % rhs })
    } else if name == "rem" || contains_any(&name, &["remainder"]) {
        let a = lhs as i32;
        let b = rhs as i32;
        if b == 0 {
            Ok(lhs)
        } else if a == i32::MIN && b == -1 {
            Ok(0)
        } else {
            Ok((a % b) as u32)
        }
    } else {
        Err(Error::unsupported(format!("unsupported mul/div op {op:?}")))
    }
}
